extern crate azul;

use azul::prelude::*;

struct MyDataModel {}

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        Dom::div()
    }
}

fn main() {
    let mut app = App::new(MyDataModel {}, AppConfig::default()).unwrap();
    let mut window_options = WindowCreateOptions::default();
    window_options.state.has_decorations = false;
    let window = app.create_window(window_options, css::native()).unwrap();
    app.run(window).unwrap();
}
