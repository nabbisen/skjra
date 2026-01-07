use gtk::prelude::{BoxExt, GtkWindowExt};
use relm4::gtk::prelude::*;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, gtk,
};

use crate::app::components::common::dialog::{DialogInput, DialogModel, DialogOutput};
use crate::app::views::dashboard::{DashboardModel, DashboardOutput};

pub struct AppModel {
    dashboard: Controller<DashboardModel>,
    dialog: Controller<DialogModel>,
}

#[derive(Debug)]
pub enum AppInput {
    RepoSelected,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppInput;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Change Shelf"),
            set_default_width: 800,
            set_default_height: 500,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                #[local_ref]
                dashboard_widget -> gtk::Box {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dashboard =
            DashboardModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    DashboardOutput::RepoSelected => AppInput::RepoSelected,
                });

        let dialog = DialogModel::builder()
            .transient_for(&root)
            .launch(true)
            // .forward(sender.input_sender(), |msg| match msg {
            //     _ => (),
            // })
            .detach();

        let model = AppModel { dashboard, dialog };

        // Insert the macro code generation here
        let dashboard_widget = model.dashboard.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppInput::RepoSelected => {
                self.dialog.sender().send(DialogInput::Show).unwrap();
            }
        }
    }
}
