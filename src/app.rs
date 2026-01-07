use relm4::RelmApp;

use crate::app::window::AppModel;

mod components;
mod views;
mod window;

pub fn start() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
