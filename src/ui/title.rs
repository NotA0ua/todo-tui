use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw_title(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("TODO", Style::default().fg(Color::Magenta)))
        .centered()
        .block(block);

    frame.render_widget(title, area);
}
