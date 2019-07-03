use std::io;

use tui::Terminal;
use tui::backend::CrosstermBackend as Backend;

mod interface;

fn main() -> Result<(), io::Error> {
    let backend = Backend::new();
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut frame| interface::draw_ui(&mut frame) )
}
