use std::borrow::{BorrowMut};
use file_search::{RecursiveDirIterator, open_file};
use uuid::Uuid;
use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Application, Button,
    Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

use dirs;
use std::collections::HashMap;

const ITEMS_PER_PAGE: i32 = 100;

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed,
    ItemSelected(String),
    LoadMorePressed,
    TabSelected(Uuid)
}

#[derive(Default)]
struct SearchUi<'s> {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    load_more_btn: button::State,
    search_text : String,
    search_results: Vec<ResultItem>,
    tab_items: HashMap<Uuid, TabItem<'s>>,
    search_iter: Option<RecursiveDirIterator>,
    show_more_visible: bool,
    tab_view: Option<fn() -> Element<'s, Message>>,
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

trait TabItemView {
    fn view(&mut self) -> Element<Message>;
}

impl<'a> SearchUi<'a> {
    fn new(mut tabs: Vec<TabItem<'a>>) -> Self {
        let tab_map = tabs.into_iter().fold(HashMap::new(), |mut map, tab| {
            map.insert(tab.id, tab);
            map
        });
        Self {
            tab_items: tab_map,
            ..Self::default()
        }
    }
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

    fn search_page(&mut self) -> Element<Message>  {
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

impl<'a> Application for SearchUi<'a> {
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
                if let Some(home_dir) = dirs::home_dir() {
                    self.search_results.clear();
                    self.search_iter.take();
                    if let Ok(res) = RecursiveDirIterator::new(home_dir) {
                        self.search_iter = Some(res);
                        self.load_results();
                    }
                }
            }
            Message::LoadMorePressed => {
                self.load_results();
            }
            Message::TabSelected(id) => {
                if let Some(tab) = self.tab_items.get(&id) {
                    self.tab_view = tab.view;
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let tabs = self.tab_items.iter_mut().fold(Row::new(), |row, (tab_id, tab)| {
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

fn main() {
    let tabs = vec![
        TabItem::new("Tab1", || {
            Text::new("hudel").into()
        }),
        TabItem::new("Tab2", || {
            Text::new("gerda").into()
        }),
    ];
    let ui = SearchUi::new(tabs);
    ui.run();
}
