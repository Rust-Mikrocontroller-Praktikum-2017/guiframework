pub mod BorderLayout;
//#[macro_use]
//extern crate collections;

//#![feature(collections)]
//use collections;

//#[derive(Debug)]
//use shape;

use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;

use util::layout_funcs::DrawArea;
use util::layout_funcs::AddForm;
 

pub struct FlowLayout {
    pub bounding_box: sizes::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
}

impl DrawArea for FlowLayout {
    fn draw_area(&self) -> bool {
        for i in &self.elements {
            i.draw();
        }
        true
    }
}
/*
impl AddForm for FlowLayout {
    fn add_form<T:Form + 'static>(&mut self, f:T) -> bool {
        self.elements.push(Box::new(f));
        true
    }
}*/

impl AddForm for FlowLayout {
    fn add_form(&mut self, f:Box<Form>) -> bool {
        self.elements.push(f);
        let len = self.elements.len();
        let el_width = self.bounding_box.width / len as u32;
        let mut n = 0;
        for i in &mut self.elements {
            let bb = sizes::BoundingBox
                    { x: self.bounding_box.x + n * el_width
                    , y: self.bounding_box.y
                    , width: el_width
                    , height: self.bounding_box.height };
            i.set_bounding_box(bb);
            n += 1;
        }
        true
    }
}