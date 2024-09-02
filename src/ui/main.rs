use std::iter::{self, Enumerate};

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use style::Styled;

use crate::app::App;

// TODO: Number, backlight, check

pub fn draw_main(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let mut constraints: Vec<Constraint> = Vec::new();

    for _ in 0..app.todos.len() {
        constraints.push(Constraint::Length(3));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(block.inner(area));

    for (i, todo) in app.todos.iter_mut().enumerate() {
        let mut style = match todo.completed {
            true => Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::CROSSED_OUT),
            false => Style::default().fg(Color::Green),
        };

        if app.current_todo == i {
            style = style.bg(Color::White);
        }

        let todo_paragraph =
            Paragraph::new(Text::styled(format!("{}. {}", i + 1, &todo.text), style))
                .block(block.clone());

        frame.render_widget(todo_paragraph, chunks[i])
    }

    frame.render_widget(block, area);
}
