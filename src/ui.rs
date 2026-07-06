use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, Focus};

pub fn render(f: &mut Frame, app: &App) {
    // Split the screen horizontally into Sidebar (25%) and Main Panel (75%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(f.area());

    render_sidebar(f, app, main_chunks[0]);
    render_main_panel(f, app, main_chunks[1]);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .requests
        .iter()
        .enumerate()
        .map(|(i, req)| {
            let style = if i == app.selected_request_idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("[{:?}] {}", req.method, req.name)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" History / Collections ")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn render_main_panel(f: &mut Frame, app: &App, area: Rect) {
    // Split right panel vertically into Request Info (top) and Response Info (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let active_req = &app.requests[app.selected_request_idx];

    let url_border_style = if app.focus == Focus::UrlBar {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    // Top: Request URL Bar
    let url_block = Paragraph::new(app.url_input.value()).block(
        Block::default()
            .title(format!(" URL: {} (Press 'e' to edit) ", active_req.name))
            .borders(Borders::ALL)
            .border_style(url_border_style),
    );
    f.render_widget(url_block, chunks[0]);

    if app.focus == Focus::UrlBar {
        // We calculate where the cursor should sit based on the chunk coordinates
        // chunks[0].x + 1 accounts for the border thickness
        f.set_cursor_position(Position {
            x: chunks[0].x + 1 + app.url_input.visual_cursor() as u16,
            y: chunks[0].y + 1,
        });
    }

    // Bottom: Response Area
    let response_content = if app.is_loading {
        "Sending request...".to_string()
    } else if let Some(resp) = &app.active_response {
        format!(
            "Status: {}\nTime: {}ms\n\nBody:\n{}",
            resp.status_code, resp.duration_ms, resp.body
        )
    } else {
        "Press <Enter> to dispatch request".to_string()
    };

    let response_block = Paragraph::new(response_content)
        .block(Block::default().title(" Response ").borders(Borders::ALL));
    f.render_widget(response_block, chunks[1]);
}
