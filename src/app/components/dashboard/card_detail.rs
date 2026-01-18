use std::{collections::HashMap, path::PathBuf, rc::Rc};

use endringer::types::{CommitInfo, DagInfo};

#[derive(Debug, Clone)]
pub struct CardDetail {
    pub id: usize,
    pub path: PathBuf,
    pub status_digest: Option<StatusDigest>,
}

#[derive(Debug, Clone)]
pub enum Message {
    DemoDelete, // 削除ボタンが押された
}

impl CardDetail {
    pub fn new(id: usize, path: PathBuf, status_digest: Option<StatusDigest>) -> Self {
        Self {
            id,
            path,
            status_digest,
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

        c = c.push(button("削除").on_press(Message::DemoDelete));

        container(c.spacing(10).align_x(Alignment::Center))
            .padding(20)
            .width(Length::Fixed(150.0))
            .style(container::rounded_box) // 0.14のスタイル指定
            .into()
    }
}
