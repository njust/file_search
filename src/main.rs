use std::borrow::{BorrowMut};
use file_search::{RecursiveDirIterator};
use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Application, Button,
    Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

const ITEMS_PER_PAGE: i32 = 100;

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed,
    ItemSelected(String),
    LoadMorePressed
}

#[derive(Default)]
struct SearchUi {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    load_more_btn: button::State,
    search_text : String,
    search_results: Vec<ResultItem>,
    search_iter: Option<RecursiveDirIterator>,
    show_more_visible: bool
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

    fn view(&mut self) -> Element<Message> {
        Button::new(
            &mut self.button,
            Text::new(&self.path)
        )
        .width(Length::Fill)
        .on_press(Message::ItemSelected(self.path.clone()))
        .background(Background::Color(Color::WHITE))
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
}

fn create_button<'a, Message>(label: &str, state: &'a mut button::State) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center))
        .border_radius(4)
        .background(Background::Color(Color{
            r: 0.0, g: 0.0, b: 0.2, a: 0.5
        }))
        .padding(4)
}

fn open_file(file_path: &String) {
    if cfg!(target_os = "windows") {
        std::process::Command::new(file_path)
            .spawn()
            .expect("asd");
    }else {
        std::process::Command::new("/usr/bin/xdg-open")
            .arg(file_path)
            .spawn()
            .expect("asd");
    }
}

impl Application for SearchUi {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Search")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(search_text) => {
                self.search_text = search_text;
            }
            Message::ItemSelected(ref item) => {
                open_file(item);
            }
            Message::SearchPressed => {
                self.search_results.clear();
                self.search_iter.take();
                if let Ok(res) = RecursiveDirIterator::new(r"/home/nico/sync/") {
                    self.search_iter = Some(res);
                    self.load_results();
                }
            }
            Message::LoadMorePressed => {
                self.load_results();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input,
            "Search",
            &self.search_text,
            Message::InputChanged
        )
            .on_submit(Message::SearchPressed)
            .padding(4);

        let btn = create_button("Search", &mut self.button)
            .on_press(Message::SearchPressed);

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
            .spacing(10)
            .padding(15)
            .push(results);

        if self.show_more_visible {
            let more_btn = create_button("Load more", &mut self.load_more_btn)
                .on_press(Message::LoadMorePressed);
            result_scrollable = result_scrollable.push(more_btn);
        }

        Column::new()
            .spacing(40)
            .padding(40)
            .push(search_bar)
            .push(result_scrollable)
            .into()
    }
}

fn main() {
    SearchUi::default().run();
}
