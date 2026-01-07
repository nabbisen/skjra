use relm4::{gtk::prelude::*, prelude::*};

// 子コンポーネントに渡すデータ
pub struct CardModel {
    pub label: String,
}

pub struct Card {
    label: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for Card {
    type Init = CardModel;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box; // 親がどのウィジェットか指定

    view! {
        gtk::Frame {
            set_label: Some(&self.label),
            set_margin_all: 8,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 10,

                gtk::Label {
                    set_label: "Card Content",
                }
            }
        }
    }

    // init_model でモデルを初期化
    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { label: init.label }
    }

    // Factory特有の更新処理（今回はシンプルにそのまま）
    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}
