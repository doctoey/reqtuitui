mod app;
mod engine;
mod models;
mod parser;
mod storage;
mod ui;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use engine::HttpManager;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Duration;
use std::{io, sync::Arc};
use tokio::sync::mpsc;

use crate::app::CurrentScreen;
use crate::{
    app::{App, UiMessage, WorkMessage},
    storage::StorageManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Storage and Mock Data
    let storage = StorageManager::new(".requestui_db")?;

    // For demonstration, pull from Sled or fallback to a default mock vector
    let mock_requests = storage.get_all_requests().unwrap_or_default();
    let mut app = App::new(if mock_requests.is_empty() {
        vec![models::ApiRequest {
            id: "1".into(),
            name: "Get JSON data".into(),
            url: "https://jsonplaceholder.typicode.com/posts/1".into(),
            method: models::HttpMethod::GET,
            headers: std::collections::HashMap::new(),
            query_params: std::collections::HashMap::new(),
            body: models::RequestBody {
                body_type: models::BodyType::None,
                content: None,
            },
        }]
    } else {
        mock_requests
    });

    // 2. Setup Channels for Inter-Thread Communication
    let (tx_worker, mut rx_worker) = mpsc::channel::<WorkMessage>(100);
    let (tx_ui, mut rx_ui) = mpsc::channel::<UiMessage>(100);

    // 3. Spawn Background Network Worker
    let http_manager = Arc::new(HttpManager::new());
    tokio::spawn(async move {
        while let Some(message) = rx_worker.recv().await {
            match message {
                WorkMessage::RunRequest(req) => {
                    let _ = tx_ui.send(UiMessage::RequestStarted).await;
                    // Execute with no active environment mapping for now
                    match http_manager
                        .execute(req, None)
                        .await
                        .map_err(|e| e.to_string())
                    {
                        Ok(resp) => {
                            let _ = tx_ui.send(UiMessage::RequestCompleted(Ok(resp))).await;
                        }
                        Err(err_str) => {
                            let _ = tx_ui.send(UiMessage::RequestCompleted(Err(err_str))).await;
                        }
                    }
                }
            }
        }
    });

    // 4. Initialize Terminal TUI Canvas
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 5. Main TUI Event Loop
    loop {
        terminal.draw(|f| ui::render(f, &app))?;
        // Non-blocking poll for user input events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    KeyCode::Enter => {
                        let active_req = app.requests[app.selected_request_idx].clone();
                        tx_worker.send(WorkMessage::RunRequest(active_req)).await?;
                    }
                    _ => {}
                }
            }
        }

        // Process any incoming messages from our network worker thread
        while let Ok(msg) = rx_ui.try_recv() {
            match msg {
                UiMessage::RequestStarted => {
                    app.is_loading = true;
                }
                UiMessage::RequestCompleted(res) => {
                    app.is_loading = false;
                    if let Ok(resp) = res {
                        app.active_response = Some(resp);
                    }
                }
            }
        }

        if let CurrentScreen::Exiting = app.current_screen {
            break;
        }
    }

    // 6. Restore Terminal Context on exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
