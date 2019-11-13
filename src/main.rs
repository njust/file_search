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

struct SearchUi<'s> {
    tab: TabControl<'s>,
}

#[derive(Debug, Default)]
struct TabItem<'a> {
    id: Uuid,
    label: String,
    view: Option<fn() -> Element<'a, Message>>,
    button: button::State
}

impl<'a> TabItem<'a> {
    fn new(label: &'static str, view: fn() -> Element<'a, Message>) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.to_owned(),
            view: Some(view),
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

impl<'a> Application for SearchUi<'a> {
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
struct TabControl<'s> {
    tab_items: HashMap<Uuid, TabItem<'s>>,
    tab_view: Option<fn() -> Element<'s, Message>>,
}

impl<'s> TabControl<'s> {
    fn new(tabs: Vec<TabItem<'s>>) -> Self {
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
            self.tab_view = tab.view;
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tabs = self.tab_items.iter_mut().fold(Row::new(), |row, (_tab_id, tab)| {
            row.push(tab.tab_header())
        });

        let mut cols = Column::new()
            .push(tabs);

        if let Some(active_tab) = self.tab_view {
            cols = cols.push(active_tab());
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
        TabItem::new("Tab1", || {
            Text::new("hudel").into()
        }),
        TabItem::new("Tab2", || {
            Text::new("gerda").into()
        }),
    ];
    let sui = SearchUi {
        tab: TabControl::new(tabs)
    };
    sui.run();
}
