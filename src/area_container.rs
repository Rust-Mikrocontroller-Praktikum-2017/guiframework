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
 

struct FlowLayout {
    bounding_box: sizes::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    elements: Vec<Box<Form>>,
}

trait AddForm {
    fn add_form(&mut self, f: Box<Form>) -> bool;
}

trait DrawArea {
    fn draw_area(&self) -> bool;
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
        true
    }
}