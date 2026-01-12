use iced::font::Weight;
use iced::widget::{button, column, container, mouse_area, row, scrollable, text};
use iced::{Alignment, Element, Length};

/// 汎用的な Drawer コンポーネント
pub struct Drawer<'a, Message> {
    title: String,
    content: Element<'a, Message>, // 表示する中身（特定の型に依存しない）
    on_close: Option<Message>,
}

impl<'a, Message> Drawer<'a, Message>
where
    Message: Clone + 'a,
{
    /// 新しい Drawer を作成。content には任意のウィジェットを渡せる。
    pub fn new(title: impl Into<String>, content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            on_close: None,
        }
    }

    /// 閉じるボタンが押された時のメッセージを設定
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// 表示用メソッド
    pub fn view(self) -> Element<'a, Message> {
        // --- Drawer 本体のレイアウト ---
        let title = self.title.to_owned();
        let drawer_body = container(
            column![
                // ヘッダー部分
                row![
                    text(title)
                        .size(24)
                        .font(iced::Font {
                            weight: Weight::Bold, // ここで太字を指定
                            ..Default::default()
                        })
                        .width(Length::Fill),
                    // text(title).size(24).width(Length::Fill),
                    button("✕")
                        .on_press_maybe(self.on_close.clone())
                        .style(button::text)
                ]
                .align_y(Alignment::Center),
                // メインコンテンツ部分
                scrollable(self.content)
                    .width(Length::Fill)
                    .height(Length::Fill)
            ]
            .spacing(20),
        )
        .padding(25)
        // .width(400) // 固定幅にすると「右から出ている感」が出る
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Color::BLACK.into()),
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                offset: iced::Vector::new(-2.5, 0.0),
                blur_radius: 7.5,
            },
            ..Default::default()
        });

        // --- 背景（クリックで閉じるエリア） ---
        let backdrop_container = container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| {
                container::Style::default().background(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2))
            });

        // Option<Message> を判定して MouseArea を適用
        let backdrop: Element<'a, Message> = if let Some(msg) = self.on_close {
            mouse_area(backdrop_container)
                .on_press(msg) // ここで Option の中身を渡す
                .into()
        } else {
            backdrop_container.into()
        };

        // 背景と本体を並べて配置
        row![backdrop, drawer_body].into()
    }
}
