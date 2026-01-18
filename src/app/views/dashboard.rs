use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::app::components::{
    common::{
        drawer::Drawer,
        select::{self, Select},
    },
    dashboard::card::{self, Card},
};
use chrono::{DateTime, Utc};
use endringer::repository::repository;
use iced::{
    Element,
    Length::Fill,
    widget::{Container, button, column, container, row, scrollable, stack, text},
};

#[derive(Default)]
pub struct Dashboard {
    selected_path: Option<PathBuf>,
    cards: Vec<card::Card>,
    selected_card_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPick,
    CardMessage(usize, card::Message),
    DrawerClose,
}

impl Dashboard {
    pub fn new() -> Self {
        let cards = vec![];

        Self {
            selected_path: None,
            cards,
            selected_card_id: None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // カードのリストをループで生成
        let card_list = row(self
            .cards
            .iter()
            .map(|card| {
                let id = card.id;
                // 子の view を map して親の Message に変換
                card.view().map(move |msg| Message::CardMessage(id, msg))
            })
            .collect::<Vec<_>>())
        .spacing(20);

        let path_display = self
            .selected_path
            .as_ref()
            .map(|p| p.to_string_lossy()) // Cow<str> が返る
            .unwrap_or_else(|| "(未選択)".into());

        let container = Container::new(column![
            text("Dashboard").size(30),
            button("フォルダを選択").on_press(Message::FolderPick),
            text(path_display),
            scrollable(row![card_list,].spacing(20).padding(20))
        ])
        .width(Fill)
        .height(Fill);
        stack![
            container,
            if let Some(selected_card_id) = self.selected_card_id {
                // ここで CardData を Element に変換する
                let card = &self.cards[selected_card_id];

                // 汎用 Drawer に詳細表示用ウィジェットを流し込む
                Drawer::new(
                    format!(
                        "詳細: {}",
                        card.path.file_name().unwrap_or_default().to_string_lossy()
                    ),
                    self.view_card_details(),
                )
                .on_close(Message::DrawerClose)
                .view()
            } else {
                text("").into()
            }
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FolderPick => {
                let path = rfd::FileDialog::new()
                    .set_title("フォルダを選択してください")
                    .pick_folder();
                if let Some(path) = path {
                    self.selected_path = Some(path);
                    self.cards_update();
                }
            }
            Message::CardMessage(id, card_message) => {
                // match card_msg {
                //     card::Message::DemoDelete => {
                //         // 特定のIDのカードを削除
                //         self.cards.retain(|c| c.id != id);
                //     }
                // }
                match self.cards.iter_mut().find(|x| x.id == id) {
                    Some(x) => {
                        x.update(card_message);
                    }
                    _ => (),
                };
                self.selected_card_id = Some(id)
            }
            Message::DrawerClose => self.selected_card_id = None,
        }
    }

    fn cards_update(self: &mut Self) {
        // let mut ret: Vec<Card> = vec![];

        if let Some(path) = self.selected_path.as_ref() {
            if path.is_dir() && path.join(".git").is_dir() {
                self.cards = vec![card(0, path.as_path())];
                return;
            }
        }

        let repos = match fs::read_dir(self.selected_path.clone().unwrap_or_default()) {
            Ok(entries) => {
                entries
                    .flatten()
                    .filter(|entry| {
                        let path = entry.path();

                        // 2. ディレクトリかどうかを確認
                        if path.is_dir() {
                            // 3. その中に ".git" フォルダが存在するか確認
                            if path.join(".git").is_dir() {
                                return true;
                            }
                        }
                        false
                    })
                    .collect()
            }
            Err(_) => vec![],
        };

        self.cards = repos
            .iter()
            .enumerate()
            .map(|(id, x)| card(id, x.path().as_path()))
            .collect();
    }

    fn view_card_details(&self) -> Element<'_, Message> {
        // let handle = Handle::from_pixels(width as u32, height as u32, pixels);

        // Image::new(handle).into()
        // column![text(self.selected_card_id.clone().unwrap_or(123)),]
        //     .spacing(10)
        //     .into()

        // let dag = endringer::dag(self.selected_path.clone().unwrap_or_default().as_path())
        //     .expect("failed to get dag");
        // let graph = build_petgraph(&dag);

        let card = self
            .cards
            .iter()
            .find(|x| {
                if let Some(card_id) = self.selected_card_id {
                    card_id == x.id
                } else {
                    false
                }
            })
            .expect("failed to find selected card");

        let commits = card
            .repository
            .list_commits()
            .expect("failed to get commits");

        let rows = commits
            .iter()
            .map(|x| {
                let datetime: DateTime<Utc> = x.timestamp.into();
                let datetime_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

                // 各行の Column
                container(column![
                    text(x.commit_id.to_string()),
                    text(x.summary.to_owned()),
                    text(x.author.to_owned()),
                    text(datetime_str),
                ])
                .padding(10)
                .into()
            })
            .collect::<Vec<_>>();
        // let mut rows = vec![];
        // for node_idx in graph.node_indices() {
        //     let row = Row::new();
        //     if let Some(info) = graph.node_weight(node_idx) {
        //         row.push(text(info.summary.to_owned()));
        //     }
        //     rows.push(row.into());
        // }
        column(rows).into()
    }
}

fn card(id: usize, path: &Path) -> Card {
    let repository = repository(path).expect("failed to get repository");

    let status_digest = match repository.status_digest() {
        Ok(a) => Some(a),
        Err(_) => None,
    };
    let local_branches = repository.local_branches().expect("failed to get branches");
    let options = local_branches
        .iter()
        .enumerate()
        .map(|(id, x)| select::SelectOption {
            id,
            label: x.name.to_owned(),
        })
        .collect();
    let branch_selector = Select::new(options, "test".to_owned());

    Card {
        id,
        path: path.to_path_buf(),
        repository,
        status_digest,
        branch_selector,
    }
}
