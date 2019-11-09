use std::collections::{HashSet};
use search_test::{RecursiveDirIterator, ignore_entry, has_extension};

use iced::{
    scrollable, button, text::HorizontalAlignment, Background, text_input, Align, Application, Button,
    Checkbox, Color, Column, Element, Length, Scrollable, Text, TextInput, Row
};

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SearchPressed
}

#[derive(Debug, Default)]
struct SearchUi {
    input: text_input::State,
    scrollable: scrollable::State,
    button: button::State,
    search_text : String,
    search_results: Vec<String>
}

impl Application for SearchUi {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(search_text) => {
                self.search_text = search_text;
            }
            Message::SearchPressed => {
                self.search_results.clear();
                if let Ok(res) = search() {
                    for r in res {
                        if let Ok(entry) = r {
                            let path = entry.path();
                            let path = path.to_str().expect("as").to_owned();
                            if path.contains(&self.search_text) {
                                self.search_results.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input,
            "Search",
            &self.search_text,
            Message::InputChanged
        ).padding(4);

        let btn = Button::new(
            &mut self.button,
            Text::new("Search"))
            .padding(4)
            .on_press(Message::SearchPressed);

        let search_bar = Row::new()
            .spacing(8)
            .push(input)
            .push(btn);

        let results = self.search_results.iter_mut().enumerate().fold(
            Column::new().spacing(4),
            | column, (i, result)| {
                column.push(Text::new(result.as_str()))
            });

        Column::new()
            .spacing(40)
            .padding(40)
            .push(search_bar)
            .push(
                Scrollable::new(&mut self.scrollable)
                    .push(results)
            )
            .into()
    }
}

fn search() -> Result<RecursiveDirIterator, std::io::Error> {
    let ignore_list = vec![".svn", "obj", "bin", "debug", "release", ".git"].into_iter().collect::<HashSet<&str>>();
    let extension_list = vec!["cpp", "h"].into_iter().collect::<HashSet<&str>>();

    return RecursiveDirIterator::new(r"/home/nico/sync/");
}

fn main() {
    SearchUi::default().run();
//    if let Ok(dir_iter) = RecursiveDirIterator::new(r"C:\Users\nico\source\DevBranches\AgentUI\") {
//        let ignore_list = vec![".svn", "obj", "bin", "debug", "release", ".git"].into_iter().collect::<HashSet<&str>>();
//        let extension_list = vec!["cpp", "h"].into_iter().collect::<HashSet<&str>>();
//        let files = dir_iter.filter(move |file| {
//            if let Ok(file) = file {
//                let path = file.path();
//                return !ignore_entry(&path, &ignore_list) && has_extension(&path, &extension_list);
//            }
//            return false;
//        });
//    }
}
