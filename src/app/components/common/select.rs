// src/string_select.rs
use iced::widget::{column, combo_box, text};
use iced::{Element, Length};

// 2. コンポーネントの状態（State）を定義
#[derive(Clone, Debug)]
pub struct Select {
    // コンボボックスの状態（選択肢のマスターリストを管理）
    options: combo_box::State<String>,
    // 現在入力されているテキスト
    input_value: String,
    // 最終的に選択された値
    selected_item: Option<String>,
    label: String,
}

// 1. コンポーネントから発生するイベント（メッセージ）を定義
#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),   // 入力内容が変わったとき
    OptionSelected(String), // 選択肢が選ばれたとき
    Closed,                 // メニューが閉じられたとき
}

impl Select {
    // 3. 初期化：選択アイテム群を登録する
    pub fn new(options: Vec<String>, label: impl Into<String>) -> Self {
        Self {
            options: combo_box::State::new(options),
            input_value: String::new(),
            selected_item: None,
            label: label.into(),
        }
    }

    // 内部の状態を更新するロジック
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::OptionSelected(item) => {
                self.selected_item = Some(item.clone());
                self.input_value = item; // 選択した値を入力欄に反映
            }
            Message::Closed => {
                // 必要に応じて、閉じられた時の処理を記述
            }
        }
    }

    // 4. View：UIを描画
    // 親のMessage型に変換できるように設計しても良いですが、
    // ここではコンポーネント自身のMessageを返します。
    pub fn view(&self) -> Element<'_, Message> {
        let label = text(&self.label);

        // combo_box にはフィルタリング前の「State全体」を渡します
        let selection_input = combo_box(
            &self.options,               // Stateへの参照
            "都市を検索...",             // プレースホルダ
            self.selected_item.as_ref(), // 現在の選択値
            Message::OptionSelected,     // 選択時のメッセージ
        )
        .on_input(Message::InputChanged) // 入力時のメッセージ（ここでフィルタリングが動く）
        .width(250);

        let content = column![
            text("都市を選択してください:").size(20),
            selection_input,
            text(format!(
                "選択中: {}",
                self.selected_item.as_deref().unwrap_or("未選択")
            )),
        ]
        .spacing(20);

        column![label, content].spacing(10).into()
    }
}
