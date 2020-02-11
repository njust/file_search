use iced::{
    button, Background, Button, HorizontalAlignment,
    Color, Column, Element, Length, Text, Row, Command
};

use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait TabItemView {
    type Message;
    fn view(&mut self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
}

pub struct TabItem<Message, MsgSender: TabMessages<Message>> {
    id: i16,
    label: String,
    button: button::State,
    mh: PhantomData<MsgSender>,
    t: PhantomData<Message>,
}

impl<Message, MsgSender: TabMessages<Message>> TabItem<Message, MsgSender>
where Message: 'static + Clone + Debug
{
    pub fn new(label: &'static str, id: i16) -> Self {
        Self {
            id,
            label: label.to_owned(),
            button: button::State::default(),
            mh: PhantomData::default(),
            t: PhantomData::default()
        }
    }

    fn tab_header(&mut self, active: bool) -> Element<Message>
    {
        let color = if active {
            Color { r: 0.0, g: 0.0, b: 0.5, a: 1.0 }
        }else {
            Color { r: 0.0, g: 0.0, b: 1.0, a: 0.0 }
        };

        Button::new(
            &mut self.button,
            Text::new(&self.label).horizontal_alignment(HorizontalAlignment::Center),
        )
            .padding(6)
            .on_press(MsgSender::tab_selected(self.id))
            .width(Length::Units(200))
            .into()

    }
}

pub trait TabMessages<T> {
    fn tab_selected(id: i16) -> T;
}

#[derive(Default)]
pub struct TabControl<Message, MsgSender: TabMessages<Message>> {
    tab_items: HashMap<i16, Box<dyn TabItemView<Message =Message>>>,
    tab_header: Vec<TabItem<Message, MsgSender>>,
    selected_tab: Option<i16>,
    mh: PhantomData<MsgSender>,
}

impl<Message: 'static + Clone + Debug, MsgSender: TabMessages<Message>> TabControl<Message, MsgSender> {
    pub fn new() -> TabControl<Message, MsgSender> {
        Self {
            tab_items: HashMap::new(),
            tab_header: Vec::new(),
            selected_tab: None,
            mh: PhantomData::default()
        }
    }

    pub fn add<T: 'static + TabItemView<Message = Message>>(&mut self, label: &'static str, view: T) {
        let view = Box::new(view);
        let id = self.tab_header.len() as i16;
        self.tab_header.push(TabItem::new(label, id));
        self.tab_items.insert(id, view);
        if None == self.selected_tab {
            self.selected_tab = Some(id);
        }
    }

    pub fn select_tab(&mut self, id: i16) {
        self.selected_tab = Some(id);
    }

    pub fn view(&mut self) -> Element<Message> {
        let selected_tab = &self.selected_tab.clone();
        let tabs = self.tab_header.iter_mut().enumerate().fold(Row::new().spacing(3), |row,  (i, tab)| {
            let mut active = false;
            if let Some(active_tab_id) = selected_tab {
                active = active_tab_id == &tab.id;
            }
            row.push(tab.tab_header(active))
        });

        let mut cols = Column::new()
            .spacing(5)
            .push(tabs);

        if let Some(selected_tab_id) = self.selected_tab {
            if let Some(selected_tab) = self.tab_items.get_mut(&selected_tab_id) {
                cols = cols.push(selected_tab.view());
            }
        }

        cols
            .padding(6)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        if let Some(selected_tab_id) = self.selected_tab {
            if let Some(selected_tab) = self.tab_items.get_mut(&selected_tab_id) {
                selected_tab.update(message);
            }
        }
        Command::none()
    }
}