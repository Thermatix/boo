use std::io;

use tui::Terminal;
use tui::backend::CrosstermBackend as Backend;

use crossterm::{input, RawScreen};

mod interface;
mod events;

fn main() -> Result<(), io::Error> {
    let backend = Backend::new();
    let mut terminal = Terminal::new(backend)?;
    let mut i  = interface::create_ui();
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();
        let mut stdin = input.read_sync();

        input.enable_mouse_mode().unwrap();

        loop {
            terminal.draw(|mut frame| {
                // i.draw_ui(&mut frame, frame.size())
                i.draw_ui(&mut frame)
            });
            if let Some(key_event) = stdin.next() {
               match events::process_input_event(key_event) {
                   events::Event::Keyboard(e) => {
                       match e {
                           events::KeyboardAction::Close => break (),
                           _ => (),
                       };
                   },
                   _ => (),
               }
            }
        };
        input.disable_mouse_mode().unwrap();
        Result::Ok(())
    } else {
        Result::Ok(())
    }
}
