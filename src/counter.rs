use iced::widget::canvas::Geometry;
use iced::widget::{button, canvas, column, text, text_input, Canvas};
use iced::{mouse, Alignment, Element, Length, Rectangle, Sandbox, Theme};
use iced_runtime::{Program, Command};
use iced_wgpu::graphics::Renderer;
use iced_wgpu::Backend;

pub struct Counter {
    pub value: i32,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Program for Counter {
    // type Renderer = iced_wgpu::Renderer<Theme>;
    type Renderer = iced::Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced_runtime::Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            },
            // Message::TextInput(value) => {
            //     self.text_value = value;
            // }
        }

        Command::none()

        // Command::perform(async { println!("hello from future") }, |_| {
        //     Message::Nothing
        // })
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        let canvas: Canvas<&Counter, Message> = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);
 
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed),
            canvas,
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

impl<Message> canvas::Program<Message, iced::Renderer> for Counter {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }
}
