use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use super::core;

pub fn draw_ui(curstate: &core::state::Game) -> Result<(), io::Error> {
 let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Log")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Input")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
}

