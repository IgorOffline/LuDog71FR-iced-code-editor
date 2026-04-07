use iced::widget::{column, container, mouse_area, text_input};
use iced::{Element, Task};
use iced_code_editor::{CodeEditor, Message as EditorMessage};

struct MyApp {
    editor: CodeEditor,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    EditorEvent(EditorMessage),
    InputChanged(String),
    TextInputClicked,
}

impl Default for MyApp {
    fn default() -> Self {
        let code = r#"fn main() {
    println!("Hello, world!");
}
"#;

        Self {
            editor: CodeEditor::new(code, "rust"),
            input_value: String::new(),
        }
    }
}

impl MyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorEvent(event) => {
                self.editor.update(&event).map(Message::EditorEvent)
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                self.editor.lose_focus();
                Task::none()
            }
            Message::TextInputClicked => {
                self.editor.lose_focus();
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = mouse_area(
            text_input("Type something...", &self.input_value)
                .on_input(Message::InputChanged)
                .padding(8),
        )
        .on_press(Message::TextInputClicked);

        container(
            column![input, self.editor.view().map(Message::EditorEvent)]
                .spacing(10)
                .height(iced::Fill),
        )
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(MyApp::update, MyApp::view)
}
