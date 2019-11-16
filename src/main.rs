use uuid::Uuid;
use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Application, Button,
    Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

use file_search::tab::{TabControl, TabItemView, TabMessages};

#[derive(Debug, Clone, Copy)]
enum Message {
    TabSelected(Uuid),
    Inc
}

struct TM {}
impl TabMessages<Message> for TM {
    fn tab_selected(id: Uuid) -> Message {
        Message::TabSelected(id)
    }
}

struct SearchUi {
    tab: TabControl<Message, TM>,
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
                self.tab.update(message);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
      self.tab.view()
    }
}

#[derive(Default)]
struct Counter {
    cnt: i32,
    btn: button::State,
}

impl TabItemView for Counter {
    type Message = Message;
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

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Inc => {
                self.cnt += 1;
            }
            _ => ()
        }
    }
}


#[derive(Default)]
struct Counter2 {
    cnt: i32,
    btn: button::State,
}

impl TabItemView for Counter2 {
    type Message = Message;
    fn view(&mut self) -> Element<Message> {
        let txt = format!("Cnaaaat: {}", self.cnt);
        Column::new()
            .push(Text::new(txt.as_str()))
            .push(
                Button::new(&mut self.btn, Text::new("Inc!"))
                    .on_press(Message::Inc)
            )
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Inc => {
                self.cnt += 1;
            }
            _ => ()
        }
    }
}

fn main() {
    let mut tc = TabControl::new();
    tc.add("Tab1", Counter::default());
    tc.add("Tab2", Counter2::default());
    let sui = SearchUi {
        tab: tc
    };
    sui.run();
}
