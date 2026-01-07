use std::{path::Path, rc::Rc};

use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::app::components::dashboard::card::{Card, CardModel, CardOutput};

pub struct DashboardModel {
    pub cards: FactoryVecDeque<Card>,
}

#[derive(Debug)]
pub enum DashboardInput {
    Demo,
    RepoSelected,
}

#[derive(Debug)]
pub enum DashboardOutput {
    RepoSelected,
}

#[relm4::component(pub)]
impl Component for DashboardModel {
    type Init = ();
    type Input = DashboardInput;
    type Output = DashboardOutput;
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
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cards = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 5))
            .forward(sender.input_sender(), |msg| match msg {
                CardOutput::RepoSelected => DashboardInput::RepoSelected,
            });
        let model = DashboardModel { cards };
        let card_box = model.cards.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
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
            DashboardInput::RepoSelected => sender.output(DashboardOutput::RepoSelected).unwrap(),
        }
    }
}
