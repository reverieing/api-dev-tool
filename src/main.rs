use api_dev_tool::HttpClient;
use iced::{
    Element, Length, Task,
    widget::{button, column, container, scrollable, text},
};
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
enum Message {
    FetchPressed,
    Fetched(Result<String, String>),
}

struct State {
    response: String,
    loading: bool,
}

fn new_state_from_response(response: String) -> State {
    State {
        response,
        loading: false,
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchPressed => {
            // mark loading and spawn an async task that performs the HTTP request
            state.loading = true;
            let url = "https://jsonplaceholder.typicode.com/posts/1".to_string();

            Task::perform(
                async move {
                    // Create a new Tokio runtime for performing the HTTP request because the
                    // iced executor does not provide a Tokio reactor. We block on the request
                    // inside this dedicated runtime so reqwest/tokio can run.
                    match Runtime::new() {
                        Ok(rt) => rt.block_on(async move {
                            match HttpClient::default_config() {
                                Ok(client) => match client.get(&url).await {
                                    Ok(body) => Ok(body),
                                    Err(e) => Err(format!("{}", e)),
                                },
                                Err(e) => Err(format!("{}", e)),
                            }
                        }),
                        Err(e) => Err(format!("Tokio runtime init error: {}", e)),
                    }
                },
                Message::Fetched,
            )
        }
        Message::Fetched(result) => {
            // update state with result and stop loading
            state.loading = false;
            match result {
                Ok(body) => state.response = body,
                Err(err_msg) => state.response = format!("Request error: {}", err_msg),
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    // Button: shows "Fetching..." while loading, otherwise clickable "Fetch"
    let fetch_btn = if state.loading {
        button(text::Text::new("Fetching...")).padding(10)
    } else {
        button(text::Text::new("Fetch"))
            .padding(10)
            .on_press(Message::FetchPressed)
    };

    // Response text inside a scrollable area.
    let response_text = text::Text::new(&state.response).size(16);
    let scroll = scrollable(response_text).height(Length::Fill);

    // Layout
    let content = column![
        text::Text::new("Fetched Response:").size(20),
        fetch_btn,
        scroll
    ]
    .spacing(10)
    .padding(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

#[tokio::main]
async fn main() -> iced::Result {
    // Start with default prompt; the user can press the Fetch button to load data.
    let response = String::from("Press Fetch to load data.");

    // Provide the `new`, `update`, and `view` functions to the iced application.
    iced::application(
        move || new_state_from_response(response.clone()),
        update,
        view,
    )
    .run()
}
