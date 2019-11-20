use file_search::{open_file, Message, SearchMessage, create_button};
use iced::{
    scrollable, button, text_input, Button,
    Column, Element, Length, Scrollable, Text, TextInput, Row, Command
};

use file_search::tab::TabItemView;

#[derive(Default)]
pub struct SearchUi {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    search_text : String,
    search_active: bool,
    search_results: Vec<ResultItem>,
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
    fn search_page(&mut self) -> Element<SearchMessage>  {
        let input = TextInput::new(
            &mut self.input,
            "Search",
            &self.search_text,
            SearchMessage::InputChanged
        ).padding(4);

        let search = self.search_text.clone();
        let btn = create_button("Search", &mut self.button)
            .on_press(SearchMessage::SearchPressed(search));

        let search_bar = Row::new()
            .spacing(8)
            .push(input)
            .push(btn);

        if self.search_active {
            Column::new()
                .push(search_bar)
                .push(Text::new("Searching.."))
                .height(Length::Fill)
                .into()
        }else {
            let results = self.search_results.iter_mut().fold(
                Column::new().spacing(4),
                | column, result| {
                    column.push(result.view())
                });

            Column::new()
                .push(search_bar)
                .push(
                    Scrollable::new(&mut self.scrollable)
                        .padding(15)
                        .height(Length::Fill)
                        .push(results)
                )
                .height(Length::Fill)
                .into()
        }
    }

    fn handle_message(&mut self, message: SearchMessage) {
        match message {
            SearchMessage::InputChanged(search_text) => {
                self.search_text = search_text;
            }
            SearchMessage::ItemSelected(ref item) => {
                open_file(item);
            }
            SearchMessage::SearchPressed(_) => {
                self.search_results.clear();
                self.search_active = true;
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
            Message::SearchResult(Ok(r)) => {
                self.search_active = false;
                for i in r {
                    self.search_results.push(ResultItem::new(i));
                }
            }
            Message::SearchMsg(search_msg) => {
                self.handle_message(search_msg)
            }
            _ => ()
        }
        Command::none()
    }
}
