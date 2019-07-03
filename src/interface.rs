use tui::layout::Rect as Size;
use tui::layout;
use tui::terminal::Frame as F;
use tui::widgets;
use super::Backend;

type Frame = F<Backend>;



pub mod elements {
    use super::Size;
    use super::Frame;
    use super::widgets::{Widget, Block, Borders};

    pub fn Drawer(mut frame: Frame, constraint: super::Size) {
        Block::default()
            .title("Drawer")
            .borders(Borders::ALL)
            .render(&mut frame, constraint);
    }

    pub fn ViewPort(mut frame: Frame, constraint: Size) {
        Block::default()
            .title("View Port")
            .borders(Borders::ALL)
            .render(&mut frame, constraint);
    }
}

pub mod layouts {
    use super::Frame;
    use super::layout::{Layout, Constraint, Direction};

    type LayoutChunks = std::collections::HashMap<String, super::Size>;

    pub fn main(frame: &Frame) -> LayoutChunks {
        make_constraints(&frame, ["left".to_owned(), "right".to_owned()].as_ref(), [20 as u16, 80 as u16].as_ref())
    }

    fn make_constraints(frame: &Frame, keys: &[String], sizes: &[u16]) -> LayoutChunks {
        let res = LayoutChunks::new();
        for (i, constraint) in  Layout::default()
                                .direction(Direction::Vertical)
                                .margin(1)
                                .constraints(sizes.iter().map(|s| Constraint::Percentage(*s)))
                                .split(frame.size()) {
            res.insert(keys[i].to_owned(), constraint);
        }
        res
    }
}
