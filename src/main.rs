use std::io;

use tui::Terminal;
use tui::backend::CrosstermBackend as Backend;

mod interface;

fn main() -> Result<(), io::Error> {
    let backend = Backend::new();
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut frame| {
        let size = frame.size();
        let main_view = interface::layouts::main(&frame);

        println!("{:?}", main_view);

        interface::elements::Drawer(&mut frame, main_view["left"]);
        interface::elements::ViewPort(&mut frame, main_view["right"]);

    })
}
