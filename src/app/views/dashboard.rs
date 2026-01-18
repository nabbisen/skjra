use std::{collections::HashMap, fs, path::PathBuf};

// components/search_bar.rs
use crate::app::components::{
    common::{drawer::Drawer, select::Select},
    dashboard::card::{self, Card},
};
use endringer::types::{CommitInfo, DagInfo};
use iced::{
    Element,
    Length::Fill,
    widget::{Container, button, column, row, scrollable, stack, text},
};
use petgraph::graph::DiGraph;

#[derive(Default)]
pub struct Dashboard {
    selected_path: Option<PathBuf>,
    cards: Vec<card::Card>,
    selected_card_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FolderPick,
    DemoDelete(usize, card::Message),
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
                card.view().map(move |msg| Message::DemoDelete(id, msg))
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
            Message::DemoDelete(id, _card_msg) => {
                // match card_msg {
                //     card::Message::DemoDelete => {
                //         // 特定のIDのカードを削除
                //         self.cards.retain(|c| c.id != id);
                //     }
                // }
                self.selected_card_id = Some(id)
            }
            Message::DrawerClose => self.selected_card_id = None,
        }
    }

    fn cards_update(self: &mut Self) {
        // let mut ret: Vec<Card> = vec![];

        if let Some(path) = self.selected_path.clone() {
            if path.is_dir() && path.join(".git").is_dir() {
                self.cards = vec![Card {
                    id: 0,
                    path: path.clone(),
                    status_digest: match endringer::status_digest(path.as_path()) {
                        Ok(a) => Some(a),
                        Err(_) => None,
                    },
                    branch_selector: Select::new(
                        vec!["test".to_owned(), "test2".to_owned()],
                        "test".to_owned(),
                    ),
                }];
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
            .map(|(i, x)| Card {
                id: i,
                path: x.path(),
                status_digest: match endringer::status_digest(x.path().as_path()) {
                    Ok(a) => Some(a),
                    Err(_) => None,
                },
                branch_selector: Select::new(
                    vec!["test".to_owned(), "test2".to_owned()],
                    "test".to_owned(),
                ),
            })
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

        let rows = (0..100)
            .map(|i| {
                // 各行の Column
                row![text(format!("row {}", i)), text("item A"), text("item B"),].into()
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

fn build_petgraph(dag: &DagInfo) -> DiGraph<CommitInfo, ()> {
    let mut graph = DiGraph::<CommitInfo, ()>::new();
    let mut id_map = HashMap::new();

    // ノードの追加
    for (oid, info) in dag.nodes.clone() {
        let idx = graph.add_node(info);
        id_map.insert(oid, idx);
    }

    // エッジの追加 (子 -> 親)
    for (child_oid, parent_oid) in dag.edges.clone() {
        if let (Some(&child_idx), Some(&parent_idx)) =
            (id_map.get(&child_oid), id_map.get(&parent_oid))
        {
            graph.add_edge(child_idx, parent_idx, ());
        }
    }
    graph
}
