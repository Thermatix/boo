use tui::layout;
use tui::terminal::Frame as F;
use tui::widgets;
use tui::style::{Style, Color};
use super::Backend as B;

use std::ops::{Index, IndexMut};

// use typemap::{TypeMap, Key};

type LayoutChunks = std::collections::HashMap<String, layout::Rect>;


pub struct WidgetMeta
{
    ui: dyn FnMut(&str) -> dyn widgets::Widget,
    size: u16,
    colourId: String,
    children: Option<ElementManager>,
}

impl WidgetMeta
{
    pub fn render(&mut self, name: &str, mut frame: &mut F<B>, constraint: &layout::Rect) {
        frame.render(&mut (self.ui)(name), constraint.clone());
    }
}


impl Index<&str> for WidgetMeta
{
    type Output = WidgetMeta;

    fn index(&self, name: &str) -> &Self::Output {
        &self.children.as_ref().unwrap().elements[&name.to_string()]
    }
}

impl IndexMut<&str> for WidgetMeta
{

    fn index_mut(&mut self, name: &str) -> &mut WidgetMeta {
        self.children.as_mut().unwrap().elements.get_mut(&name.to_string()).unwrap()
    }
}

type ElementList = std::collections::HashMap<String, WidgetMeta>;

pub struct ElementManager {
    elements: ElementList,
    colour_grid: Vec<String>,
    names: Vec<String>,
    direction: layout::Direction
}

impl ElementManager {

    fn new(root_direction: layout::Direction) -> Self {
        Self {
            elements: ElementList::new(),
            colour_grid: Vec::new(),
            names: Vec::new(),
            direction: root_direction,
        }
    }

    fn add_widget<WW>(&mut self, name: &str,
                         size: u16,
                         direction: Option<layout::Direction>,
                         children: bool,
                         ui_element: dyn FnMut(&str)  -> (dyn widgets::Widget) + 'static)
    {
        self.elements.insert(name.to_string(),
            WidgetMeta {
                ui: ui_element,
                size: size,
                colourId: "".to_string(),
                children: if children {
                    Some(Self::new(direction.unwrap()))
                } else {
                    None
                }
            }
        );
        self.names.push(name.to_string());
    }

    /// draw ui function
    pub fn draw_ui(&mut self, mut frame: &mut F<B>) {
        &self.draw(&mut self.elements, &mut frame, self.constraints(frame.size()).clone());
    }

    /// Recursive draw ui function
    fn draw(&mut self, elements: &mut ElementList, mut frame: &mut F<B>, constraints: LayoutChunks) {
        for (name, element) in elements {
            element.render(name.clone(), &mut frame, &constraints[&name.clone()]);
            match &mut element.children {
                Some(c) => self.draw(&mut c.elements,
                                       &mut frame,
                                       c.constraints(constraints[&name.clone()])),
                None => ()
            }
        };
    }

    /// accepts a size: tui::layout::Rect
    pub fn constraints(&self, size: layout::Rect ) -> LayoutChunks {
        let mut res = LayoutChunks::new();
        let mut constraints = Vec::new();

        for (_, element) in &self.elements {
            constraints.push(layout::Constraint::Percentage(element.size.clone()))
        }

       for (i, c) in layout::Layout::default()
            .direction(self.direction.clone())
            .constraints(constraints)
            .split(size.clone()).iter().enumerate() {

            res.insert(self.names[i].clone(), *c);
        }

        // println!("{:?}", res);
        res
    }
}

impl Index<&str> for ElementManager {
    type Output = ElementManager;

    fn index(&self, name: &str) -> &Self::Output {
        &self.elements[&name.to_string()].children.as_ref().unwrap()
    }
}

impl IndexMut<&str> for ElementManager {

    fn index_mut(&mut self, name: &str) -> &mut ElementManager {
        self.elements.get_mut(&name.to_string()).unwrap().children.as_mut().unwrap()
    }
}


pub fn create_ui() -> ElementManager {
    use layout::Direction;

    let mut em = ElementManager::new(Direction::Horizontal);
    em.add_widget("drawer", 20 as u16, None, false, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em.add_widget("view_port", 80 as u16, Some(Direction::Vertical), true, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em["view_port"].add_widget("buffers", 10 as u16, Some(Direction::Vertical), false, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em["view_port"].add_widget("buffer", 80 as u16, Some(Direction::Vertical), true, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em["view_port"].add_widget("input", 10 as u16, Some(Direction::Vertical), false, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em["view_port"]["buffer"]
        .add_widget("thread", 80 as u16, Some(Direction::Vertical), false, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });

    em["view_port"]["buffer"]
    .add_widget("users", 20 as u16, Some(Direction::Vertical), false, |name| {
       Box::new(widgets::Block::default()
            // .title(name)
            .borders(widgets::Borders::ALL)
            .style(Style::default().bg(Color::Black)))
    });
   em
}
