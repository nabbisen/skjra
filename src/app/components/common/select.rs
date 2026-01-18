// src/string_select.rs
use iced::widget::{column, pick_list, text};
use iced::{Element, Length};

// 1. コンポーネントから発生するイベント（メッセージ）を定義
#[derive(Debug, Clone)]
pub enum Message {
    ItemSelected(String), // 選択変更時に発生するイベント
}

// 2. コンポーネントの状態（State）を定義
#[derive(Clone, Debug)]
pub struct Select {
    options: Vec<String>,           // 登録された選択肢
    selected_value: Option<String>, // 現在選択されている値
    label: String,                  // ラベル（任意）
}

impl Select {
    // 3. 初期化：選択アイテム群を登録する
    pub fn new(options: Vec<String>, label: impl Into<String>) -> Self {
        Self {
            options,
            selected_value: None, // 初期状態は未選択
            label: label.into(),
        }
    }

    // 内部の状態を更新するロジック
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ItemSelected(value) => {
                self.selected_value = Some(value);
            }
        }
    }

    // 親側から現在の値を取得するためのヘルパー
    pub fn get_selected(&self) -> Option<&String> {
        self.selected_value.as_ref()
    }

    // 4. View：UIを描画
    // 親のMessage型に変換できるように設計しても良いですが、
    // ここではコンポーネント自身のMessageを返します。
    pub fn view(&self) -> Element<'_, Message> {
        let label = text(&self.label);

        // pick_list(選択肢の参照, 現在の選択値, 変更時のメッセージ生成関数)
        let pick_list = pick_list(
            self.options.as_slice(),
            self.selected_value.clone(),
            Message::ItemSelected,
        )
        .placeholder("選択してください...")
        .width(Length::Fill);

        column![label, pick_list].spacing(10).into()
    }
}
