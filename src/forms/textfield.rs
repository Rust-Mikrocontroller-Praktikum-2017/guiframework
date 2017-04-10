use core::iter;
use collections::boxed::Box;
use stm32f7::lcd::Color;

use draw::fill_rectangle;
use forms::form::Clickable;
use forms::form::Movable;
use forms::form::Form;
use util::bounding_box::BoundingBox;

pub struct Textfield {
    bounding_box: BoundingBox,
    child: Option<Box<Form>>,
    movable: bool,
}

impl Textfield {
    pub fn new(bounding_box: BoundingBox) -> Textfield {
        Textfield {
            bounding_box: bounding_box,
            child: None,
            movable: false,
        }
    }
}

impl Form for Textfield {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        Box::new(iter::empty())
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        None
    }

    fn is_movable(&mut self) -> Option<&mut Movable> {
        None
    }

    fn clear(&self) -> () {
        fill_rectangle(self.bounding_box.x,
                       self.bounding_box.y,
                       self.bounding_box.width,
                       self.bounding_box.height,
                       Color::rgba(0, 0, 0, 0));
    }

    fn draw(&self) -> () {}
}
