// app.rs
use iced::Element; // 子をインポート

use crate::app::views::dashboard;

#[derive(Default)]
pub struct Window {
    dashboard: dashboard::Dashboard,
}

#[derive(Debug, Clone)]
pub enum Message {
    Search(dashboard::Message), // 子のメッセージを内包
}

impl Window {
    pub fn new() -> Self {
        let dashboard = dashboard::Dashboard::new();
        Self { dashboard }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // .map() を使って Element<dashboard::Message> を Element<Message> に変換
        self.dashboard.view().map(Message::Search)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Search(sub_msg) => {
                // 子の update を呼び出す
                let _ = self.dashboard.update(sub_msg);
            }
        }
    }
}
