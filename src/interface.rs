use tui::layout::Rect;
use tui::layout;
use tui::terminal::Frame as F;
use tui::widgets;
use super::Backend as B;

// type &F<B> = F<Backend>;



pub mod elements {
    use super::{F, B};
    use super::Rect;
    use super::widgets::{Widget, Block, Borders};

    pub fn Drawer(frame: &mut F<B>, constraint: Rect) {
        Block::default()
            .title("Drawer")
            .borders(Borders::ALL)
            .render(frame, constraint);
    }

    pub fn ViewPort(frame: &mut F<B>, constraint: Rect) {
        Block::default()
            .title("View Port")
            .borders(Borders::ALL)
            .render(frame, constraint);
    }
}

pub mod layouts {
    use super::{F, B, Rect};
    use super::layout::{Layout, Constraint, Direction};

    type LayoutChunks = std::collections::HashMap<String, Rect>;

    pub fn main(frame: &F<B>) -> LayoutChunks {
        make_constraints(&frame,
                         ["left".to_string(),"right".to_string()].as_ref(),
                         [20 as u16, 80 as u16].as_ref(),
                         Direction::Horizontal)
    }

    fn make_constraints(frame: &F<B>, keys: &[String], sizes: &[u16], direction: Direction) -> LayoutChunks {
        let mut res = LayoutChunks::new();
        let mut constraints = Vec::new();

        for s in sizes {
            constraints.push(Constraint::Percentage(*s))
        }

       for (i, c) in Layout::default()
            .direction(direction)
            .margin(1)
            .constraints(constraints)
            .split(frame.size()).iter().enumerate() {

            res.insert(keys[i].to_string(), *c);
        }

        println!("{:?}", res);
        res
    }
}
