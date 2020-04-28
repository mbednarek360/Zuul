use std::io::{self, Read};
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






        // basic layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Percentage(93),
                Constraint::Percentage(7),
            ].as_ref()).split(f.size());






        // message buffer
        let block = Block::default()
             .title("Log")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);






        // command input
        let block = Block::default()
             .title("Input")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);






    })
}

