use std::path::PathBuf;

use relm4::gtk::prelude::*;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt, gtk,
};

use crate::app::views::dashboard::{DashboardInput, DashboardModel};

pub struct AppModel {
    selected_path: String,
    dashboard: Controller<DashboardModel>,
}

#[derive(Debug)]
pub enum AppInput {
    OpenFolderDialog,
    FolderSelected(Option<PathBuf>),
}

#[relm4::component(pub)]
impl Component for AppModel {
    type Init = u8;

    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Window {
            set_title: Some("Change Shelf"),
            set_default_width: 800,
            set_default_height: 500,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "フォルダを選択",
                    connect_clicked => AppInput::OpenFolderDialog,
                },

                #[local_ref]
                dashboard_widget -> gtk::Box {},
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dashboard = DashboardModel::builder().launch(()).detach();

        let model = AppModel {
            selected_path: String::new(),
            dashboard,
        };

        // Insert the macro code generation here
        let dashboard_widget = model.dashboard.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            AppInput::OpenFolderDialog => {
                // ダイアログの構築
                let dialog = gtk::FileChooserDialog::new(
                    Some("フォルダを選択してください"),
                    Some(root),                           // 親ウィンドウを指定
                    gtk::FileChooserAction::SelectFolder, // フォルダ選択モード
                    &[
                        ("_Cancel", gtk::ResponseType::Cancel),
                        ("_Open", gtk::ResponseType::Accept),
                    ],
                );

                // 応答時の処理
                let sender_clone = sender.clone();
                dialog.connect_response(move |d, response| {
                    if response == gtk::ResponseType::Accept {
                        // 選択されたフォルダを取得
                        let file = d.file();
                        let path = file.and_then(|f| f.path());
                        sender_clone.input(AppInput::FolderSelected(path));
                    }
                    // ダイアログを閉じる
                    d.destroy();
                });

                dialog.show();
            }
            AppInput::FolderSelected(Some(path)) => {
                self.selected_path = path.to_string_lossy().to_string();

                self.dashboard
                    .sender()
                    .send(DashboardInput::RootSelected(PathBuf::from(
                        &self.selected_path,
                    )))
                    .unwrap()
            }
            AppInput::FolderSelected(None) => {
                // キャンセルされた場合
            }
        }
    }
}
