use uuid::Uuid;
use iced::{ button, Application, Column, Element, Text, Command};
use file_search::tab::{TabControl, TabItemView, TabMessages};
use file_search::{Message, create_button};
use crate::search::SearchUi;

mod search;

struct TM {}
impl TabMessages<Message> for TM {
    fn tab_selected(id: Uuid) -> Message {
        Message::TabSelected(id)
    }
}

struct FileSearch {
    tab: TabControl<Message, TM>,
}

impl Application for FileSearch {
    type Message = Message;

    fn new() -> (FileSearch, Command<Message>) {
        let mut tc = TabControl::new();
        tc.add("Search", SearchUi::default());
        tc.add("Test 1", Counter::default());
        tc.add("Test 2", Counter::default());
        (
            FileSearch {
                tab: tc
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Search")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(id) => {
                self.tab.select_tab(id);
            }
            Message::Inc => {
                self.tab.update(message);
            },
            msg => {
                self.tab.update(msg)
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
//      self.tab.view().explain(Color::BLACK)
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
                create_button("Increment", &mut self.btn)
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
    FileSearch::run();
}
