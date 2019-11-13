use uuid::Uuid;
use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Application, Button,
    Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

use std::collections::HashMap;

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
    view: Box<dyn TabItemView>,
    button: button::State
}

impl TabItem {
    fn new(label: &'static str, view: Box<dyn TabItemView>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.to_owned(),
            view,
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
    tab_items: HashMap<Uuid, TabItem>,
    tab_view: Option<Box<dyn TabItemView>>,
}

impl TabControl {
    fn new(tabs: Vec<TabItem>) -> Self {
        let tab_map = tabs.into_iter().fold(HashMap::new(), |mut map, tab| {
            map.insert(tab.id, tab);
            map
        });
        Self {
            tab_items: tab_map,
            ..Self::default()
        }
    }

    pub fn select_tab(&mut self, id: Uuid) {
        if let Some(tab) = self.tab_items.get(&id) {
//            self.tab_view = Some(tab.view);
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tabs = self.tab_items.iter_mut().fold(Row::new(), |row, (_tab_id, tab)| {
            row.push(tab.tab_header())
        });

        let mut cols = Column::new()
            .push(tabs);

        if let Some(ref tab_view) = &self.tab_view {
//            cols = cols.push(tab_view.view());
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
    let tabs = vec![
        TabItem::new("Tab1", Box::new(Counter::default()))
    ];
    let sui = SearchUi {
        tab: TabControl::new(tabs)
    };
    sui.run();
}
