use gtk::prelude::*;
use relm4::prelude::*;

pub struct CardDetailModel {
    // drawer 内の状態（選択項目など）があればここに定義
}

#[derive(Debug)]
pub enum CardDetailInput {
    // メインから drawer への命令
    CloseRequested,
}

#[derive(Debug)]
pub enum CardDetailOutput {
    // drawer からメインへの通知（「閉じるボタンが押された」など）
    CloseRequested,
}

#[relm4::component(pub)]
impl Component for CardDetailModel {
    type Init = ();
    type Input = CardDetailInput;
    type Output = CardDetailOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_width_request: 250,
            // add_css_class: "background", // 背景色を確保

            // ヘッダー部分
            gtk::CenterBox {
                set_margin_all: 10,
                #[wrap(Some)]
                set_center_widget = &gtk::Label {
                    set_label: "メニュー",
                    // add_css_class: "title-4",
                },
                #[wrap(Some)]
                set_end_widget = &gtk::Button {
                    set_icon_name: "window-close-symbolic",
                    // add_css_class: "flat",
                    // メインウィンドウへ「閉じてほしい」と通知
                    connect_clicked => CardDetailInput::CloseRequested,
                }
            },

            gtk::Separator {},

            // メニューリスト
            gtk::ListBox {
                add_css_class: "navigation-sidebar",
                set_vexpand: true,

                gtk::Label { set_label: "ホーム", set_margin_all: 10 },
                gtk::Label { set_label: "設定", set_margin_all: 10 },
            }
        }
    }

    fn init(
        _: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CardDetailModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            // 内部ロジックが必要な場合
            CardDetailInput::CloseRequested => {
                sender.output(CardDetailOutput::CloseRequested).unwrap()
            }
        }
    }
}
