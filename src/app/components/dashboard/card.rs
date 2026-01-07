use std::{path::Path, rc::Rc, time::UNIX_EPOCH};

use chrono::{DateTime, Local};
use endringer::status_digest;
use relm4::{gtk::prelude::*, prelude::*};

// 子コンポーネントに渡すデータ
pub struct CardModel {
    pub path: Rc<Path>,
}

#[derive(Debug)]
pub enum CardOutput {
    RepoSelected,
}

pub struct Card {
    path: Rc<Path>,
    status_digest: endringer::types::StatusDigest,
}

#[relm4::factory(pub)]
impl FactoryComponent for Card {
    type Init = CardModel;
    type Input = ();
    type Output = CardOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box; // 親がどのウィジェットか指定

    view! {
        gtk::Frame {
            set_label: Some(&self.path.to_str().unwrap()),

            set_margin_all: 8,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 10,

                gtk::Label {
                    set_selectable: true,
                    set_label: &self.status_digest.repo_name,
                },
                gtk::Label {
                    set_selectable: true,
                    set_label: &self.status_digest.current_branch,
                },
                gtk::Label {
                    set_selectable: true,
                    set_label: &self.status_digest.last_commit_summary,
                },
                gtk::Label {
                    set_selectable: true,
                    set_label: &({
                        let duration_since_epoch = &self.status_digest.last_commit_time
                            .duration_since(UNIX_EPOCH)
                            .expect("SystemTime earlier than UNIX_EPOCH");

                        // Build a chrono DateTime<Local> from the seconds + nanoseconds
                        let datetime: DateTime<Local> = DateTime::from_timestamp(
                                duration_since_epoch.as_secs() as i64,
                                duration_since_epoch.subsec_nanos(),
                            )
                            .expect("Invalid timestamp")
                            .with_timezone(&Local);
                        let x = datetime.to_string();
                        x
                    }),
                },

                gtk::Button {
                    set_label: "選択",
                    connect_clicked => sender.output(CardOutput::RepoSelected).unwrap(),
                },
            }
        }
    }

    // init_model でモデルを初期化
    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let status_digest = status_digest(&init.path).expect("failed to get status digest");
        Self {
            path: init.path,
            status_digest,
        }
    }

    // Factory特有の更新処理（今回はシンプルにそのまま）
    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}
