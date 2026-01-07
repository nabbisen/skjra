use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::app::components::dashboard::card::{Card, CardModel};

pub struct DashboardModel {
    pub cards: FactoryVecDeque<Card>,
}

#[derive(Debug)]
pub enum DashboardMsg {
    Randomize,
}

#[relm4::component(pub)]
impl Component for DashboardModel {
    type Init = ();
    type Input = DashboardMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,

            gtk::Button {
                set_label: "ランダム生成",
                connect_clicked => DashboardMsg::Randomize,
            },

            gtk::ScrolledWindow {
                set_vexpand: true,
                #[local_ref]
                card_box -> gtk::Box {}
            }
        }
    }

    fn init(_init: Self::Init, _root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let cards = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 5))
            .forward(sender.input_sender(), |_| {
                // 子コンポーネントの Output が () の場合は、
                // 親の Input に変換するメッセージがないため何もしない
                unreachable!() 
            });
        let model = DashboardModel { cards };
        let card_box = model.cards.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            DashboardMsg::Randomize => {
                let mut guard = self.cards.guard();
                guard.clear();
                for i in 1..=10 {
                    guard.push_back(CardModel { label: format!("Card #{}", i) });
                }
            }
        }
    }
}