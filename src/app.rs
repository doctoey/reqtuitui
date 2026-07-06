use crate::models::{ApiRequest, ApiResponse};
use tui_input::Input;

#[derive(PartialEq)]
pub enum Focus {
    Sidebar,
    UrlBar,
}

pub enum CurrentScreen {
    Sidebar,
    RequestPanel,
    Exiting,
}

/// Messages sent from the TUI to the background HTTP worker
pub enum WorkMessage {
    RunRequest(ApiRequest),
}

/// Messages sent from the background HTTP worker back to the TUI
pub enum UiMessage {
    RequestStarted,
    RequestCompleted(Result<ApiResponse, String>),
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub focus: Focus,
    pub requests: Vec<ApiRequest>,
    pub selected_request_idx: usize,
    pub url_input: Input,
    pub active_response: Option<ApiResponse>,
    pub is_loading: bool,
}

impl App {
    pub fn new(initial_requests: Vec<ApiRequest>) -> Self {
        // Pre-fill the input with the first request's URL
        let initial_url = initial_requests
            .first()
            .map(|r| r.url.clone())
            .unwrap_or_default();
        Self {
            current_screen: CurrentScreen::Sidebar,
            focus: Focus::Sidebar,
            requests: initial_requests,
            selected_request_idx: 0,
            url_input: Input::default().with_value(initial_url),
            active_response: None,
            is_loading: false,
        }
    }
}
