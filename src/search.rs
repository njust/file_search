use std::borrow::{BorrowMut};
use file_search::{RecursiveDirIterator, open_file, Message, SearchMessage, create_button};
use iced::{
    scrollable, button, text_input, Button,
    Column, Element, Length, Scrollable, Text, TextInput, Row, Command
};

use dirs;
use file_search::tab::TabItemView;

const ITEMS_PER_PAGE: i32 = 100;

#[derive(Default)]
pub struct SearchUi {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    load_more_btn: button::State,
    search_text : String,
    search_results: Vec<ResultItem>,
    search_iter: Option<RecursiveDirIterator>,
    show_more_visible: bool,
}

#[derive(Debug, Default)]
struct ResultItem {
    path: String,
    button: button::State
}

impl ResultItem {
    fn new(path: String) -> Self {
        ResultItem {
            button: button::State::default(),
            path
        }
    }

    fn view(&mut self) -> Element<SearchMessage> {
        Button::new(
            &mut self.button,
            Text::new(&self.path)
        )
        .width(Length::Fill)
        .on_press(SearchMessage::ItemSelected(self.path.clone()))
        .into()
    }
}


impl SearchUi {
    fn load_results(&mut self) {
        if let Some(mut res) = self.search_iter.take() {
            let search = &self.search_text.to_lowercase();
            let mut match_count = 0;
            self.show_more_visible = false;
            for r in res.borrow_mut() {
                if match_count > ITEMS_PER_PAGE {
                    self.show_more_visible = true;
                    break;
                }
                if let Ok(entry) = r {
                    let path = entry.path();
                    let path = path.to_str().expect("as").to_owned();
                    if path.to_lowercase().contains(search) {
                        self.search_results.push(ResultItem::new(path));
                        match_count += 1;
                    }
                }
            }
            self.search_iter = Some(res);
        }
    }

    fn search_page(&mut self) -> Element<SearchMessage>  {
        let input = TextInput::new(
            &mut self.input,
            "Search",
            &self.search_text,
            SearchMessage::InputChanged
        )
            .on_submit(SearchMessage::SearchPressed)
            .padding(4);

        let btn = create_button("Search", &mut self.button)
            .on_press(SearchMessage::SearchPressed);

        let search_bar = Row::new()
            .spacing(8)
            .push(input)
            .push(btn);

        let results = self.search_results.iter_mut().fold(
            Column::new().spacing(4),
            | column, result| {
                column.push(result.view())
            });

        let mut result_scrollable = Scrollable::new(&mut self.scrollable)
            .padding(15)
            .height(Length::Fill)
            .push(results);

        if self.show_more_visible {
            let more_btn = create_button("Load more", &mut self.load_more_btn)
                .on_press(SearchMessage::LoadMorePressed);
            result_scrollable = result_scrollable.push(more_btn);
        }

        Column::new()
            .push(search_bar)
            .push(result_scrollable)
            .height(Length::Fill)
            .into()
    }

    fn handle_message(&mut self, message: SearchMessage) {
        match message {
            SearchMessage::InputChanged(search_text) => {
                self.search_text = search_text;
            }
            SearchMessage::ItemSelected(ref item) => {
                open_file(item);
            }
            SearchMessage::SearchPressed => {
                if let Some(home_dir) = dirs::home_dir() {
                    self.search_results.clear();
                    self.search_iter.take();
                    if let Ok(res) = RecursiveDirIterator::new(home_dir) {
                        self.search_iter = Some(res);
                        self.load_results();
                    }
                }
            }
            SearchMessage::LoadMorePressed => {
                self.load_results();
            }
        }
    }
}

impl TabItemView for SearchUi {
    type Message = Message;

    fn view(&mut self) -> Element<Self::Message> {
        self.search_page().map(move |message| {
            Message::SearchMsg(message)
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SearchMsg(search_msg) => {
                self.handle_message(search_msg)
            }
            _ => ()
        }
        Command::none()
    }
}
