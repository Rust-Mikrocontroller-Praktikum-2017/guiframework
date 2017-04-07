use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;

pub enum BorderArea {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

pub trait AddForm {
    fn add_form(&mut self, f: Box<Form>) -> bool;
}

pub trait DrawArea {
    fn draw_area(&self) -> bool;
}

pub trait AddFormBorder {
    fn add_form(&mut self, f: Box<Form>, pos: BorderArea) -> bool;
}

// pub trait DrawAreaBorder {
//     fn draw_area(&self) -> bool;
// }
