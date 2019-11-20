use file_search::{open_file, Message, SearchMessage, create_button, ResultItem};
use iced::{
    scrollable, button, text_input, Button,
    Column, Element, Length, Scrollable, Text, TextInput, Row, Command
};

use file_search::tab::TabItemView;

const PAGE_SIZE: i32 = 50;

#[derive(Default)]
pub struct SearchUi {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    show_more_btn: button::State,
    search_text : String,
    search_active: bool,
    show_more: bool,
    offset: i32,
    search_results: Vec<ResultItemWidget>,
}

#[derive(Debug, Default)]
struct ResultItemWidget {
    item: ResultItem,
    button: button::State
}

impl ResultItemWidget {
    fn new(item: ResultItem) -> Self {
        ResultItemWidget {
            button: button::State::default(),
            item
        }
    }

    fn view(&mut self) -> Element<SearchMessage> {
        Button::new(
            &mut self.button,
            Text::new(&self.item.path)
        )
        .width(Length::Fill)
        .on_press(SearchMessage::ItemSelected(self.item.path.clone()))
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
            let mut results = Column::new().spacing(4);
            for (i, search_result) in &mut self.search_results.iter_mut().enumerate() {
                if i as i32 >= (self.offset + PAGE_SIZE) {
                    self.show_more = true;
                    break;
                }
                results = results.push(search_result.view());
            }

            let mut col = Scrollable::new(&mut self.scrollable)
                .padding(15)
                .height(Length::Fill)
                .push(results);

            if self.show_more {
                col = col.push(
                    create_button("Load more", &mut self.show_more_btn)
                        .on_press(SearchMessage::ShowMore)
                );
            }

            Column::new()
                .push(search_bar)
                .push(col)
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
                self.show_more = false;
                self.offset = 0;
                self.search_active = true;
            }
            SearchMessage::ShowMore => {
                self.offset += PAGE_SIZE;
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
            Message::SearchResult(Ok(search_result)) => {
                self.search_active = false;
                for item in search_result {
                    self.search_results.push(ResultItemWidget::new(item));
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
