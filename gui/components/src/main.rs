#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate azul;

use azul::prelude::*;

struct List {
    items: Vec<&'static str>,
    selected: Option<usize>,
}

const CUSTOM_CSS: &str = ".selected { background-color: black; color: white; }";

impl Layout for List {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        self.items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                Dom::label(*item)
                    .with_class(if self.selected == Some(idx) {
                        "selected".into()
                    } else {
                        ""
                    })
                    .with_callback(On::MouseDown, print_which_item_was_selected)
            })
            .collect::<Dom<Self>>()
    }
}

fn print_which_item_was_selected(event: CallbackInfo<List>) -> UpdateScreen {
    let selected = event.target_index_in_parent();
    let mut should_redraw = DontRedraw;

    if selected != event.state.data.selected {
        event.state.data.selected = selected;
        should_redraw = Redraw;
    }

    println!("selected item: {:?}", event.state.data.selected);

    should_redraw
}

fn main() {
    let data = List {
        items: vec!["Hello", "World", "my", "name", "is", "Lorem", "Ipsum"],
        selected: None,
    };

    let mut app = App::new(data, AppConfig::default()).unwrap();
    let css = css::override_native(CUSTOM_CSS).unwrap();
    let window = app
        .create_window(WindowCreateOptions::default(), css)
        .unwrap();
    app.run(window).unwrap();
}
