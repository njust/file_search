use uuid::Uuid;
use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Application, Button,
    Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Uuid),
    Inc
}

struct SearchUi {
    tab: TabControl,
}

struct TabItem {
    id: Uuid,
    label: String,
    button: button::State
}

impl TabItem {
    fn new(label: &'static str, id: Uuid) -> Self {
        Self {
            id,
            label: label.to_owned(),
            button: button::State::default()
        }
    }

    fn tab_header(&mut self) -> Element<Message> {
        Button::new(
            &mut self.button,
            Text::new(&self.label),
        )
            .width(Length::Units(200))
            .on_press(Message::TabSelected(self.id))
            .into()
    }
}

impl Application for SearchUi {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Search")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TabSelected(id) => {
                self.tab.select_tab(id);
            }
            Message::Inc => {
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
      self.tab.view()
    }
}

#[derive(Default)]
struct TabControl {
    tab_items: HashMap<Uuid, Rc<Box<dyn TabItemView>>>,
    tab_header: HashMap<Uuid, TabItem>,
    tab_view: Option<Rc<RefCell<dyn TabItemView>>>,
}

impl TabControl {
    fn new() -> Self {
        Self::default()
    }

    pub fn add_tab(&mut self, label: &'static str, view: Box<dyn TabItemView>) {
        let id = Uuid::new_v4();
        self.tab_header.insert(id, TabItem::new(label, id));
        self.tab_items.insert(id, Rc::new(view));
    }

    pub fn select_tab(&mut self, id: Uuid) {
        if let Some(tab) = self.tab_items.get(&id) {
//            self.tab_view = Some(tab.clone());
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tabs = self.tab_header.iter_mut().fold(Row::new(), |row, (_tab_id, tab)| {
            row.push(tab.tab_header())
        });

        let mut cols = Column::new()
            .push(tabs);

        if let Some(tab_view) = &self.tab_view {
            let mut tab_view = tab_view.clone();
            let mut view = tab_view.borrow_mut();
            let v = view.view();
            cols = cols.push(v);
        }

        return cols.into();
    }
}

#[derive(Default)]
struct Counter {
    cnt: i32,
    btn: button::State,
}

trait TabItemView {
    fn view(&mut self) -> Element<Message>;
}

impl TabItemView for Counter {
    fn view(&mut self) -> Element<Message> {
        let txt = format!("Cnt: {}", self.cnt);
        Column::new()
            .push(Text::new(txt.as_str()))
            .push(
                Button::new(&mut self.btn, Text::new("Inc!"))
                    .on_press(Message::Inc)
            )
            .into()
    }
}

fn main() {
    let mut tc = TabControl::new();
    tc.add_tab("Tab1", Box::new(Counter::default()));
    tc.add_tab("Tab2", Box::new(Counter::default()));
    let sui = SearchUi {
        tab: tc
    };
    sui.run();
}
