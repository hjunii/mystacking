use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, scrollable, text_input,
    text
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Element, Fill};
use appconfig::AppConfigManager;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MyStackingConfig{
    siril_path: String,
    data_path: String,
    output_path: String
}

fn main() -> iced::Result {
    iced::run("My Stacking", MyStacking::update, MyStacking::view)
}

pub struct MyStacking {
    screen: Screen,
    siril_path: String,
    data_path: String,
    output_path: String    
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    SirilPathChanged(String),
    DataPathChanged(String),
    OutputPathChanged(String)
}

impl MyStacking {
    
    fn update(&mut self, message: Message) {
        match message {
            Message::BackPressed => {
                if let Some(screen) = self.screen.previous() {
                    self.screen = screen;
                }
            }
            Message::NextPressed => {
                if let Some(screen) = self.screen.next() {
                    self.screen = screen;
                }
            }
            Message::SirilPathChanged(siril_path) => {
                self.siril_path = siril_path;
            }
            Message::DataPathChanged(data_path) => {
                self.data_path = data_path;
            }
            Message::OutputPathChanged(output_path) => {
                self.output_path = output_path;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let controls =
            row![]
                .push_maybe(self.screen.previous().is_some().then(|| {
                    padded_button("Back")
                        .on_press(Message::BackPressed)
                        .style(button::secondary)
                }))
                .push(horizontal_space())
                .push_maybe(self.can_continue().then(|| {
                    padded_button("Next").on_press(Message::NextPressed)
                }));
        
        let screen = match self.screen {
            Screen::Start => self.start(),
            Screen::Selection => self.selection(),
            Screen::Result => self.result(),

        };

        let content: Element<_> = column![screen, controls,]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        let scrollable = scrollable(
            container(content)
            .center_x(Fill),
        );

        container(scrollable).center_y(Fill).into()
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::Start => !self.siril_path.is_empty() && !self.data_path.is_empty() && !self.output_path.is_empty(),
            Screen::Selection => true,
            Screen::Result => false
        }
    }

    fn start(&self) -> Column<Message> {
        let mut siril_path = text_input("Type something to continue...", &self.siril_path)
            .on_input(Message::SirilPathChanged)
            .padding(10)
            .size(30);
        let mut  data_path = text_input("Type something to continue...", &self.data_path)
            .on_input(Message::DataPathChanged)
            .padding(10)
            .size(30);
        let mut output_path = text_input("Type something to continue...", &self.output_path)
            .on_input(Message::OutputPathChanged)
            .padding(10)
            .size(30);

        Self::container("Setup")
            .push("Siril Path")
            .push(siril_path.secure(false))
            .push("Data Path")
            .push(data_path)
            .push("Output Path")
            .push(output_path)
    }

    fn selection(&self) -> Column<Message> {
        Self::container("Selection!")
    }

    fn result(&self) -> Column<Message> {
        Self::container("Result!")
    }

    fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Start,
    Selection,
    Result
}

impl Screen {
    const ALL: &'static [Self] = &[
        Screen::Start,
        Screen::Selection,
        Screen::Result
    ];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}

fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
    button(text(label)).padding([12, 24])
}

impl Default for MyStacking {
    fn default() -> Self {
        Self {
            screen: Screen::Start,
            siril_path: String::from(""),
            data_path: String::from(""),
            output_path: String::from("")
        }
    }
}