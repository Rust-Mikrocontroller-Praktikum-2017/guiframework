use collections::boxed::Box;
use core::iter;

use draw::draw_rectangle;
use draw::fill_rectangle;
use forms::form::Clickable;
use forms::form::Movable;
use forms::form::Form;
use util::bounding_box::BoundingBox;
use stm32f7::lcd::Color;


pub struct Button {
    bounding_box: BoundingBox,
    child: Option<Box<Form>>,
    on_click: Option<fn(form: &mut Button) -> ()>,
    movable: bool,
    border_width: i32,
}

impl Button {
    pub fn new(bounding_box: BoundingBox, border_width: i32) -> Button {
        Button {
            bounding_box: bounding_box,
            child: None,
            on_click: None,
            movable: false,
            border_width: border_width,
        }
    }

    pub fn set_action_on_click(&mut self, callback: fn(form: &mut Button) -> ()) -> () {
        self.on_click = Some(callback);
    }

    pub fn set_child(&mut self, child: Box<Form>) -> () {
        self.child = Some(child);
    }
}

impl Form for Button {
    fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn get_border_width(&self) -> i32 {
        self.border_width
    }

    fn set_border_width(&mut self, width: i32) -> () {
        if width < 0 {
            return;
        }

        self.border_width = width;
    }


    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        match self.child {
            None => Box::new(iter::empty()),
            Some(ref mut child) => Box::new(iter::once::<&'a mut Form>(&mut **child)),
        }
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        match self.on_click {
            None => None,
            _ => Some(self),
        }
    }

    fn is_movable(&mut self) -> Option<&mut Movable> {
        match self.movable {
            true => None,
            false => Some(self),
        }
    }

    fn clear(&self) -> () {
        fill_rectangle(self.bounding_box.x,
                       self.bounding_box.y,
                       self.bounding_box.width,
                       self.bounding_box.height,
                       Color::rgba(0, 0, 0, 0));
    }

    fn draw(&self) -> () {
        for i in 0i32..self.border_width {
            draw_rectangle(self.bounding_box.x + i,
                           self.bounding_box.y + i,
                           self.bounding_box.width - (2 * i),
                           self.bounding_box.height - (2 * i),
                           Color::rgb(0b00110, 0b00110, 0b11010));
        }

        match self.child {
            None => (),
            Some(ref child) => child.draw(),
        }
    }
}

impl Clickable for Button {
    fn click(&mut self) {
        match self.on_click {
            Some(func) => func(self),
            None => (),
        }
    }
}

impl Movable for Button {
    fn move_form(&mut self, dir_x: i32, dir_y: i32) {
        self.bounding_box.move_in_direction(dir_x, dir_y);
    }
}
