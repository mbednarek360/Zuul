use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction, self};
use super::core;

// draw layout with current state
pub fn draw_ui(curstate: &core::state::Game) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut xpos = 0u16;
    terminal.draw(|mut f| {
    
        // stats
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
            ].as_ref()).split(split_rect(f.size(), 20, &mut xpos));

        // statistics
        let mut stat_buffer: Vec<Text> = Vec::new();
        format_stats(curstate, &mut stat_buffer);
        let block = Paragraph::new(stat_buffer.iter())
            .block(Block::default()
            .title("Stats")
            .borders(Borders::ALL));
        f.render_widget(block, chunks[0]);

        // message and command
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ].as_ref()).split(split_rect(f.size(), 80, &mut xpos));

        // message buffer
        let block = Paragraph::new(curstate.get_buffer().iter())
             .block(Block::default()
             .title("Log")
             .borders(Borders::ALL));
        f.render_widget(block, chunks[0]);

        // command input
        let block = Block::default()
             .title("Input")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;

    // set cursor to input block
    terminal.show_cursor()?;
    terminal.set_cursor((
        terminal.size()?.width as f32 * 0.2 /*stats ratio*/ ) as u16 + 1,
        terminal.size()?.height - 2)?;
    Ok(())
}

// create horizontal rec for split because idk how to use the layout lib
fn split_rect(main: layout::Rect, percent: u16, xpos: &mut u16) -> layout::Rect {
    let width = (main.width as f32 * (percent as f32 / 100f32)) as u16;
    let rect = layout::Rect::new(*xpos, 0, width, main.height);
    *xpos += width;
    rect
}

// prettify stats from gamestate
fn format_stats(curstate: &core::state::Game, buffer: &mut Vec<Text>) {

}