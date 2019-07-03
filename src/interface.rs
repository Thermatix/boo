use tui::layout::Rect;
use tui::layout;
use tui::terminal::Frame as F;
use tui::widgets;
use tui::style::{Style, Color};
use super::Backend as B;

type LayoutChunks = std::collections::HashMap<String, Rect>;

pub fn draw_ui(mut frame: &mut F<B>) {
    let main_view = layouts::main(
        &frame.size(),
        &["drawer".to_string(), "view_port".to_string()]
    );
    let view_port = layouts::viewPortInterior(
        &main_view["view_port"],
        &["buffers".to_string(), "buffer".to_string(), "input".to_string()]
     );

    let buffer_view = layouts::bufferView(
        &view_port["buffer"],
        &["thread".to_string(), "users".to_string()]
    );

    elements::drawer(&mut frame, &main_view, "drawer");
    elements::viewPort(&mut frame, &main_view, "view_port");
    elements::buffer_control(&mut frame, &view_port, "buffers");
    elements::buffer_view(&mut frame, &buffer_view, "thread");
    elements::chat_users(&mut frame, &buffer_view, "users");
    elements::chat_input(&mut frame, &view_port, "input");
}

mod elements {
    use super::{F, B, LayoutChunks, Rect, Style, Color};
    use super::widgets::{Widget, Block, Borders};

     pub  fn drawer(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }

    pub fn viewPort(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }

    pub fn buffer_control(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }

    pub fn buffer_view(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }
    pub fn chat_input(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }
    pub fn chat_users(frame: &mut F<B>, constraints: &LayoutChunks, name: &str) {
        common_features::<Block>(&constraints[name])
            .title(name.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black))
            .render(frame, constraints[name]);
    }

    fn common_features<W>(constraint: &Rect) -> W
        where W: Widget + Default
    {
        W::default()
    }

}

mod layouts {
    use super::{Rect, LayoutChunks};
    use super::layout::{Layout, Constraint, Direction};


    pub fn main(size: &Rect, keys: &[String]) -> LayoutChunks {
        make_constraints(&size,
                         keys,
                         &[20 as u16, 80 as u16],
                         Direction::Horizontal)
    }

    pub fn viewPortInterior(size: &Rect, keys: &[String]) -> LayoutChunks {
        make_constraints(&size,
                         keys,
                         &[10 as u16, 80 as u16, 10 as u16 ],
                         Direction::Vertical)
    }

    pub  fn bufferView(size: &Rect, keys: &[String]) -> LayoutChunks {
        make_constraints(&size,
                         keys,
                         &[80 as u16, 20 as u16],
                         Direction::Horizontal)
    }

    pub fn make_constraints(size: &Rect, keys: &[String], sizes: &[u16], direction: Direction) -> LayoutChunks {
        let mut res = LayoutChunks::new();
        let mut constraints = Vec::new();

        for s in sizes {
            constraints.push(Constraint::Percentage(*s))
        }

       for (i, c) in Layout::default()
            .direction(direction)
            .margin(1)
            .constraints(constraints)
            .split(*size).iter().enumerate() {

            res.insert(keys[i].to_string(), *c);
        }

        println!("{:?}", res);
        res
    }
}
