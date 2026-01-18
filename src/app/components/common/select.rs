// src/string_select.rs
use iced::Element;
use iced::widget::{column, combo_box, text};

// 2. コンポーネントの状態（State）を定義
#[derive(Clone, Debug)]
pub struct Select {
    // コンボボックスの状態（選択肢のマスターリストを管理）
    options: combo_box::State<SelectOption>,
    // 最終的に選択された値
    selected_item: Option<SelectOption>,
    // 現在入力されているテキスト
    input_value: String,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub id: usize,
    pub label: String,
}

impl std::fmt::Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

// 1. コンポーネントから発生するイベント（メッセージ）を定義
#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),         // 入力内容が変わったとき
    OptionSelected(SelectOption), // 選択肢が選ばれたとき
}

impl Select {
    // 初期化：選択アイテム群を登録する
    pub fn new(options: Vec<SelectOption>, label: impl Into<String>) -> Self {
        Self {
            options: combo_box::State::new(options),
            input_value: String::new(),
            selected_item: None,
            label: label.into(),
        }
    }

    // View：UIを描画
    // 親のMessage型に変換できるように設計しても良いですが、
    // ここではコンポーネント自身のMessageを返します。
    pub fn view(&self) -> Element<'_, Message> {
        let label = text(&self.label);

        // combo_box にはフィルタリング前の「State全体」を渡します
        let selection_input = combo_box(
            &self.options,               // Stateへの参照
            "検索...",                   // プレースホルダ
            self.selected_item.as_ref(), // 現在の選択値
            Message::OptionSelected,     // 選択時のメッセージ
        )
        .on_input(Message::InputChanged) // 入力時のメッセージ（ここでフィルタリングが動く）
        .width(250);

        let content = column![
            text("選択してください:").size(20),
            selection_input,
            text(format!(
                "選択中: {}",
                self.selected_item
                    .as_ref()
                    .map(|item| item.to_string())
                    .unwrap_or_else(|| "未選択".to_string())
            )),
        ]
        .spacing(20);

        column![label, content].spacing(10).into()
    }

    // 内部の状態を更新するロジック
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::OptionSelected(item) => {
                self.selected_item = Some(item.clone());
                self.input_value = item.to_string(); // 選択した値を入力欄に反映
            }
        }
    }
}
