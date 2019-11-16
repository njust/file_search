use uuid::Uuid;
use iced::{
    button, Background, Button,
    Color, Column, Element, Length, Text, Row
};

use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait TabItemView {
    type Message;
    fn view(&mut self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
}

pub struct TabItem<Message, MsgSender: TabMessages<Message>> {
    id: Uuid,
    label: String,
    button: button::State,
    mh: PhantomData<MsgSender>,
    t: PhantomData<Message>,
}

impl<Message, MsgSender: TabMessages<Message>> TabItem<Message, MsgSender>
where Message: 'static + Clone + Debug
{
    pub fn new(label: &'static str, id: Uuid) -> Self {
        Self {
            id,
            label: label.to_owned(),
            button: button::State::default(),
            mh: PhantomData::default(),
            t: PhantomData::default()
        }
    }

    fn tab_header(&mut self) -> Element<Message>
    {
        Button::new(
            &mut self.button,
            Text::new(&self.label),
        )
            .background(Background::Color(Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 0.0
            }))
            .padding(6)
            .on_press(MsgSender::tab_selected(self.id))
            .width(Length::Units(200))
            .into()

    }
}

pub trait TabMessages<T> {
    fn tab_selected(id: Uuid) -> T;
}

#[derive(Default)]
pub struct TabControl<Message, MsgSender: TabMessages<Message>> {
    tab_items: HashMap<Uuid, Box<dyn TabItemView<Message =Message>>>,
    tab_header: HashMap<Uuid, TabItem<Message, MsgSender>>,
    selected_tab: Option<Uuid>,
    mh: PhantomData<MsgSender>,
}

impl<Message: 'static + Clone + Debug, MsgSender: TabMessages<Message>> TabControl<Message, MsgSender> {
    pub fn new() -> TabControl<Message, MsgSender> {
        Self {
            tab_items: HashMap::new(),
            tab_header: HashMap::new(),
            selected_tab: None,
            mh: PhantomData::default()
        }
    }

    pub fn add<T: 'static + TabItemView<Message = Message>>(&mut self, label: &'static str, view: T) {
        let view = Box::new(view);
        let id = Uuid::new_v4();
        self.tab_header.insert(id, TabItem::new(label, id));
        self.tab_items.insert(id, view);
    }

    pub fn select_tab(&mut self, id: Uuid) {
        self.selected_tab = Some(id);
    }

    pub fn view(&mut self) -> Element<Message> {
        let tabs = self.tab_header.iter_mut().fold(Row::new().spacing(3), |row, (_tab_id, tab)| {
            row.push(tab.tab_header())
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

    pub fn update(&mut self, message: Message) {
        if let Some(selected_tab_id) = self.selected_tab {
            if let Some(selected_tab) = self.tab_items.get_mut(&selected_tab_id) {
                selected_tab.update(message);
            }
        }
    }
}