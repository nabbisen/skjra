use std::path::PathBuf;

use endringer::types::StatusDigest;
use iced::widget::{Column, button, column, container, text};
use iced::{Alignment, Element, Length};

use crate::app::components::common::select::{self, Select};
use crate::app::utils::system_time_to_string;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub path: PathBuf,
    pub status_digest: Option<StatusDigest>,
    pub branch_selector: Select,
}

#[derive(Debug, Clone)]
pub enum Message {
    DemoDelete, // 削除ボタンが押された
    SelectMessage(select::Message),
}

impl Card {
    pub fn new(
        id: usize,
        path: PathBuf,
        status_digest: Option<StatusDigest>,
        branch_selector: Select,
    ) -> Self {
        Self {
            id,
            path,
            status_digest,
            branch_selector,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut c: Column<'_, Message> = column![];

        let dir_name = self
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if let Some(status_digest) = self.status_digest.clone() {
            // todo: currently endringer can't open repo whose name is different from dir name
            // therefore, always go to else clause
            let repo_name = if status_digest.repo_name != dir_name {
                format!("{} (Dir: {})", status_digest.repo_name, dir_name)
            } else {
                status_digest.repo_name
            };
            c = c.push(text(repo_name).size(20));
            c = c.push(text(status_digest.current_branch));
            c = c.push(text(status_digest.last_commit_summary));
            c = c.push(text(system_time_to_string(status_digest.last_commit_time)));
        } else {
            c = c.push(text(dir_name).size(20));
        };

        c = c.push(button("詳細").on_press(Message::DemoDelete));

        c = c.push(
            self.branch_selector
                .view()
                .map(move |msg| Message::SelectMessage(msg)),
        );

        container(c.spacing(10).align_x(Alignment::Center))
            .padding(20)
            .width(Length::Fixed(150.0))
            .style(container::rounded_box) // 0.14のスタイル指定
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectMessage(select_message) => {
                // 1. まず子に処理させて、子の状態を更新する
                self.branch_selector.update(select_message.clone());

                // 2. その上で、もし「選択」イベントだったら親としての追加処理をする
                if let select::Message::OptionSelected(selected_value) = select_message {
                    println!("親が選択を検知しました: {}", selected_value);
                    // ここで親にしかできない処理（API呼び出しなど）を書く
                }
            }
            Message::DemoDelete => {}
        }
    }
}
