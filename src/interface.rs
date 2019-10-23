use tui::layout;
use tui::terminal::Frame as F;
use tui::widgets;
use tui::style::{Style, Color};
use indexmap::IndexMap;
use super::Backend as B;

use std::ops::{Index, IndexMut};

// use typemap::{TypeMap, Key};

type LayoutChunks = std::collections::HashMap<String, layout::Rect>;


#[derive(Clone, Debug)]
pub struct WidgetMeta
{
    size: u16,
    colour_id: String,
    borders: widgets::Borders,
    style: Style,
    children: Option<ElementManager>,
}

impl WidgetMeta
{
    pub fn render( &self, name: &str, frame: &mut F<B>, constraint: &layout::Rect) {
        print!("{}: {:?}\n\r", name, constraint);
        // print!("{}: {:?}\n\r", name,self);
        frame.render(
           &mut widgets::Block::default()
            .title(name)
            .borders(self.borders.clone())
            .style(self.style.clone()),
            constraint.clone()
            );
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

type ElementList = IndexMap<String, WidgetMeta>;

#[derive(Clone, Debug)]
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

    fn add_widget(&mut self, name: &str,
                         size: u16,
                         direction: Option<layout::Direction>,
                         children: bool,
                         style: Style,
                         borders: widgets::Borders)
    {
        self.elements.insert(name.to_string(),
            WidgetMeta {
                size: size,
                borders: borders,
                style: style,
                colour_id: "".to_string(),
                children: if children {
                    Some(Self::new(direction.unwrap()))
                } else {
                    None
                }
            }
        );
        self.names.push(name.to_string());
    }

    pub fn remove_widget(&mut self, name: &str) {
        self.elements.shift_remove(name);
    }

    /// draw ui function
    pub fn draw_ui(&mut self, frame: &mut F<B>) {
        print!("frame size: {:?}\n\r", frame.size());
        &self.draw(self.elements.clone(), frame, self.constraints(frame.size()).clone());
    }

    /// Recursive draw ui function
    fn draw(&mut self, elements: ElementList, mut frame: &mut F<B>, constraints: LayoutChunks) {
        // print!("Contraints:\n\r");
        // print!("{:?}\n\r", constraints);
        for (name, mut element) in elements {
            print!("render: {}: {:?}\n\r", name, &constraints[&name.clone()]);
            element.render(name.as_ref(), &mut frame, &constraints[&name.clone()]);
            match &mut element.children {
                Some(c) => self.draw(c.elements.clone(),
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
            print!("constraint: {}: {:?}\n\r", self.names[i], c);
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
    em.add_widget("drawer", 20 as u16, None, false, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em.add_widget("view_port", 80 as u16, Some(Direction::Vertical), true, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em["view_port"].add_widget("buffers", 10 as u16, Some(Direction::Horizontal), false, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em["view_port"].add_widget("buffer", 80 as u16, Some(Direction::Horizontal), true, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em["view_port"].add_widget("input", 10 as u16, Some(Direction::Horizontal), false, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em["view_port"]["buffer"]
        .add_widget("thread", 80 as u16, Some(Direction::Vertical), false, Style::default().bg(Color::Black), widgets::Borders::ALL);

    em["view_port"]["buffer"]
    .add_widget("users", 20 as u16, Some(Direction::Vertical), false, Style::default().bg(Color::Black), widgets::Borders::ALL);
   em
}
