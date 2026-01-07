use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::app::components::dashboard::{
    card::{Card, CardModel, CardOutput},
    card_detail::{CardDetailModel, CardDetailOutput},
};

pub struct DashboardModel {
    card_detail_visible: bool,
    pub cards: FactoryVecDeque<Card>,
    card_detail: Controller<CardDetailModel>,
}

#[derive(Debug)]
pub enum DashboardInput {
    Demo,
    RepoSelected(PathBuf),
    CloseCardDetail,
}

#[relm4::component(pub)]
impl Component for DashboardModel {
    type Init = ();
    type Input = DashboardInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,

            gtk::Button {
                set_label: "デモ生成",
                connect_clicked => DashboardInput::Demo,
            },

            gtk::ScrolledWindow {
                set_vexpand: true,
                #[local_ref]
                card_box -> gtk::Box {}
            },

            gtk::Revealer {
                        set_transition_type: gtk::RevealerTransitionType::SlideLeft,
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::Fill,

                        #[watch]
                        set_reveal_child: model.card_detail_visible,

                        // 子コンポーネントのウィジェットを配置
                        #[local_ref]
                        card_detail -> gtk::Box {},
                    }
            },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let card_detail_visible = false;

        let cards = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 5))
            .forward(sender.input_sender(), |msg| match msg {
                CardOutput::RepoSelected(path_buf) => DashboardInput::RepoSelected(path_buf),
            });

        let card_detail =
            CardDetailModel::builder()
                .launch(())
                .forward(sender.input_sender(), |output| match output {
                    CardDetailOutput::CloseRequested => DashboardInput::CloseCardDetail,
                });

        let model = DashboardModel {
            card_detail_visible,
            cards,
            card_detail,
        };

        let card_box = model.cards.widget();
        let card_detail = model.card_detail.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            DashboardInput::Demo => {
                let mut guard = self.cards.guard();
                guard.clear();
                for _i in 1..=3 {
                    guard.push_back(CardModel {
                        path: Rc::from(Path::new(".")),
                    });
                }
            }
            DashboardInput::RepoSelected(_path_buf) => self.card_detail_visible = true,
            DashboardInput::CloseCardDetail => self.card_detail_visible = false,
        }
    }
}
