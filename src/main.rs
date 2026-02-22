use api_dev_tool::HttpClient;
use iced::{
    Element, Length, Task,
    widget::{column, container, scrollable, text},
};

#[derive(Debug, Clone)]
enum Message {}

struct State {
    response: String,
}

fn new_state_from_response(response: String) -> State {
    State { response }
}

fn update(_state: &mut State, _message: Message) -> Task<Message> {
    // This simple viewer is non-interactive; no tasks to run.
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    // Build a scrollable area that contains the fetched response.
    // Use the `column!` macro for layout and the `scrollable(...)` function to allow scrolling.
    let response_text = text::Text::new(&state.response).size(16);

    // `scrollable(...)` takes the content directly; no need to construct a Scrollable::new manually.
    let scroll = scrollable(response_text).height(Length::Fill);

    let content = column![text::Text::new("Fetched Response:").size(20), scroll]
        .spacing(10)
        .padding(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}

#[tokio::main]
async fn main() -> iced::Result {
    // Fetch the response before launching the GUI so the UI can start with data.
    let response = match HttpClient::default_config() {
        Ok(client) => match client
            .get("https://jsonplaceholder.typicode.com/posts/1")
            .await
        {
            Ok(body) => body,
            Err(e) => format!("Request error: {}", e),
        },
        Err(e) => format!("Client init error: {}", e),
    };

    // Provide the `new`, `update`, and `view` functions to the iced application.
    // The `new` function here is a closure that captures the pre-fetched response.
    iced::application(
        move || new_state_from_response(response.clone()),
        update,
        view,
    )
    .run()
}
