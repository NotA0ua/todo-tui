use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw_footer(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let line = Line::from(vec![
        "Quit ".into(),
        "[Q]".red().into(),
        " | Create new TODO ".into(),
        "[N]".red().into(),
        " | Delete TODO ".into(),
        "[D]".red().into(),
        " | Check completed ".into(),
        "[SPACE]".red().into(),
        " | Edit TODO ".into(),
        "[ENTER]".red().into(),
        " | Scroll TODOs ".into(),
        "[UP & DOWN]".red().into(),
    ]);

    let footer = Paragraph::new(Text::from(line)).centered().block(block);

    frame.render_widget(footer, area);
}
