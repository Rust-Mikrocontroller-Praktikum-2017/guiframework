use collections::boxed::Box;
use core::iter;
use stm32f7;
use stm32f7::lcd::Color;

use draw;
use draw::fill_rectangle;
use forms::form::Clickable;
use forms::form::Form;
use forms::form::Movable;
use util::bounding_box::BoundingBox;

pub struct Label {
    bounding_box: BoundingBox,
    child: Option<Box<Form>>,
    movable: bool,
    text: &'static str,
}

impl Label {
    pub fn new(bounding_box: BoundingBox, text: &'static str) -> Label {
        Label {
            bounding_box: bounding_box,
            child: None,
            movable: false,
            text: text,
        }
    }
}

impl Form for Label {
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

    fn draw(&self) -> () {
        stm32f7::with_stdout(|stdout| {
            let (width, height) = stdout.width_height(self.text);
            let (x_center, y_center) = self.bounding_box.get_center();

            let x_offset = x_center - width as i32 / 2;
            let y_offset = y_center - height as i32 / 2;

            stdout.set_offset(x_offset as usize, y_offset as usize);
        });

        print!("{:?}", self.text);
    }
}
