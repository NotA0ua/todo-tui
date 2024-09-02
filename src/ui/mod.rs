use footer::draw_footer;
use main::draw_main;
use ratatui::{prelude::*, Frame};
use title::draw_title;

use crate::app::App;

mod footer;
mod main;
mod title;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_title(frame, chunks[0]);
    draw_main(frame, chunks[1], app);
    draw_footer(frame, chunks[2]);
}
