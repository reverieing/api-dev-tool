use api_dev_tool::HttpClient;
use iced::{
    Element, Length, Task,
    widget::{button, column, container, scrollable, text},
};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::oneshot;

#[derive(Debug, Clone)]
enum Message {
    FetchPressed,
    Fetched(Result<String, String>),
}

struct State {
    response: String,
    loading: bool,
    rt_handle: Handle,
}

fn new_state_from_response(response: String, rt_handle: Handle) -> State {
    State {
        response,
        loading: false,
        rt_handle,
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchPressed => {
            // mark loading and spawn an async task that performs the HTTP request
            state.loading = true;
            let url = "https://jsonplaceholder.typicode.com/posts/1".to_string();
            let handle = state.rt_handle.clone();

            Task::perform(
                async move {
                    // Spawn the async request on the shared runtime and return the result via a oneshot channel.
                    // This avoids blocking the current task's thread.
                    let (tx, rx) = oneshot::channel::<Result<String, String>>();
                    // Build the async client task that performs the request and sends the result.
                    let client_task = async move {
                        let res = match HttpClient::default_config() {
                            Ok(client) => match client.get(&url).await {
                                Ok(body) => Ok(body),
                                Err(e) => Err(format!("{}", e)),
                            },
                            Err(e) => Err(format!("{}", e)),
                        };
                        // Ignore send errors (receiver may have been dropped).
                        let _ = tx.send(res);
                    };
                    // Spawn the client task on the shared runtime handle.
                    handle.spawn(client_task);
                    // Await the oneshot receiver to get the request result.
                    match rx.await {
                        Ok(res) => res,
                        Err(e) => Err(format!("oneshot recv error: {}", e)),
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

fn main() -> iced::Result {
    // Start with default prompt; the user can press the Fetch button to load data.
    let response = String::from("Press Fetch to load data.");

    // Create a single Tokio runtime and keep it alive for the app lifetime.
    // Keep the runtime alive in this scope for the program lifetime by binding it to `_rt`.
    // Because `main` is synchronous the runtime will only be dropped after `run()` returns.
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _rt = rt;
    let handle = _rt.handle().clone();

    // Provide the `new`, `update`, and `view` functions to the iced application.
    iced::application(
        move || new_state_from_response(response.clone(), handle.clone()),
        update,
        view,
    )
    .run()
}
