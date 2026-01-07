use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use relm4::factory::FactoryVecDeque;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::app::components::dashboard::{
    card::{Card, CardModel, CardOutput},
    card_detail::{CardDetailInput, CardDetailModel, CardDetailOutput},
};

pub struct DashboardModel {
    card_detail_visible: bool,
    pub cards: FactoryVecDeque<Card>,
    card_detail: Controller<CardDetailModel>,
}

#[derive(Debug)]
pub enum DashboardInput {
    RootSelected(PathBuf),
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

            gtk::Overlay {
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    #[local_ref]
                    card_box -> gtk::Box {}
                },

                add_overlay = &gtk::Revealer {
                    set_transition_type: gtk::RevealerTransitionType::SlideLeft,
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Fill,

                    #[watch]
                    set_reveal_child: model.card_detail_visible,

                    // 子コンポーネントのウィジェットを配置
                    #[local_ref]
                    card_detail -> gtk::Box {},
                }
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
            DashboardInput::RootSelected(path_buf) => {
                let repos = match fs::read_dir(&path_buf) {
                    Ok(entries) => {
                        entries
                            .flatten()
                            .filter(|entry| {
                                let path = entry.path();

                                // 2. ディレクトリかどうかを確認
                                if path.is_dir() {
                                    // 3. その中に ".git" フォルダが存在するか確認
                                    if path.join(".git").is_dir() {
                                        return true;
                                    }
                                }
                                false
                            })
                            .collect()
                    }
                    Err(_) => vec![],
                };

                let mut guard = self.cards.guard();
                guard.clear();
                for repo in repos {
                    guard.push_back(CardModel {
                        path: Rc::from(repo.path()),
                    });
                }
            }
            DashboardInput::RepoSelected(path_buf) => {
                self.card_detail
                    .sender()
                    .send(CardDetailInput::RepoSelected(path_buf))
                    .unwrap();
                self.card_detail_visible = true
            }
            DashboardInput::CloseCardDetail => self.card_detail_visible = false,
        }
    }
}
