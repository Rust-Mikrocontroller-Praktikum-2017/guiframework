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

pub struct HorizontalLayout {
    pub bounding_box: sizes::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
}

pub struct VerticalLayout {
    pub bounding_box: sizes::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
}

pub trait AddForm {
    fn add_form(&mut self, f: Box<Form>) -> bool;
}

pub trait DrawArea {
    fn draw_area(&self) -> bool;
}

impl DrawArea for HorizontalLayout {
    fn draw_area(&self) -> bool {
        for i in &self.elements {
            i.draw();
        }
        true
    }
}

impl AddForm for HorizontalLayout {
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

impl AddForm for VerticalLayoutLayout {
    fn add_form(&mut self, f:Box<Form>) -> bool {
        self.elements.push(f);
        let len = self.elements.len();
        let el_height = self.bounding_box.height / len as u32;
        let mut n = 0;
        for i in &mut self.elements {
            let bb = sizes::BoundingBox
                    { x: self.bounding_box.x
                    , y: self.bounding_box.y + n * el_height
                    , width: self.bounding_box.weight
                    , height: el_height };
            i.set_bounding_box(bb);
            n += 1;
        }
        true
    }
}