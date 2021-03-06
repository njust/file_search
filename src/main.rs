use iced::{ button, Application, Column, Element, Text, Command, Settings};
use file_search::tab::{TabControl, TabItemView, TabMessages};
use file_search::{Message, create_button, Search, SearchMessage};
use crate::search::SearchUi;
mod search;

struct TM;
impl TabMessages<Message> for TM {
    fn tab_selected(id: i16) -> Message {
        Message::TabSelected(id)
    }
}

struct FileSearch {
    tab: TabControl<Message, TM>,
}

impl Application for FileSearch {
    type Message = Message;
    type Executor = iced::executor::Default;

    fn new() -> (FileSearch, Command<Message>) {
        let mut tc = TabControl::new();
        tc.add("Search", SearchUi::default());
        tc.add("Test 1", Counter::default());
        tc.add("Test 2", Counter::default());
        (
            FileSearch {
                tab: tc,
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
            Message::SearchMsg(ref search) => {
                match search {
                    SearchMessage::SearchPressed(search) => {
                        self.tab.update(message.clone());
                        if let Some(home_dir) = dirs::home_dir() {
                            let home = String::from(home_dir.to_str().expect("Invalid homepath"));
                            return Command::perform(
                                Search::new(search.to_owned(), home).run(), Message::SearchResult
                            );
                        }
                    }
                    _ => {
                        return self.tab.update(Message::SearchMsg(search.clone()))
                    }
                }
            }
            msg => {
                return self.tab.update(msg);
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

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Inc => {
                self.cnt += 1;
            }
            _ => ()
        }
        Command::none()
    }
}

fn main() {
    FileSearch::run(Settings::default());
}
