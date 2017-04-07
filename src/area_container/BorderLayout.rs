use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;

use util::layout_funcs::AddFormBorder;
use util::layout_funcs::DrawArea;

use util::layout_funcs::BorderArea;


pub struct BorderLayout {
    pub bounding_box: sizes::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub top_element: Box<Form>,
    pub bottom_element: Box<Form>,
    pub left_element: Box<Form>,
    pub right_element: Box<Form>,
    pub center_element: Box<Form>,
}

impl DrawArea for BorderLayout {
    fn draw_area(&self) -> bool {
        let all_el = [&self.top_element, &self.bottom_element, &self.left_element, &self.right_element, &self.center_element];
        for i in all_el.iter() {
            i.draw();
        }
        true
    }
}

impl AddFormBorder for BorderLayout {
    fn add_form(&mut self, mut f:Box<Form>, pos:BorderArea) -> bool {
        match pos {
            BorderArea::Top => {
                let width = self.bounding_box.width;
                let height = self.bounding_box.height / 3;
                let bb = sizes::BoundingBox{x:self.bounding_box.x, y:self.bounding_box.y, width:width, height:height};
                f.set_bounding_box(bb);
                self.top_element = f;
            }
            BorderArea::Bottom => {
                let width = self.bounding_box.width;
                let height = self.bounding_box.height / 3;
                let bb = sizes::BoundingBox{x:self.bounding_box.x, y:self.bounding_box.y + height * 2, width:width, height:height};
                f.set_bounding_box(bb);
                self.bottom_element = f;
            }, 
            BorderArea::Left => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = sizes::BoundingBox{x:self.bounding_box.x, y:self.bounding_box.y + height, width:width, height:height};
                f.set_bounding_box(bb);
                self.left_element = f;
            },
            BorderArea::Right => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = sizes::BoundingBox{x:self.bounding_box.x, y:self.bounding_box.y + height, width:width, height:height};
                f.set_bounding_box(bb);
                self.right_element = f;
            },
            BorderArea::Center => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = sizes::BoundingBox{x:self.bounding_box.x + width, y:self.bounding_box.y + height, width:width, height:height};
                f.set_bounding_box(bb);
                self.center_element = f;
            },
        }
        return true;

        // self.elements.push(f);
        // let len = self.elements.len();
        // let el_width = self.bounding_box.width / len as u32;
        // let mut n = 0;
        // for i in &mut self.elements {
        //     let bb = sizes::BoundingBox
        //             { x: self.bounding_box.x + n * el_width
        //             , y: self.bounding_box.y
        //             , width: el_width
        //             , height: self.bounding_box.height };
        //     i.set_bounding_box(bb);
        //     n += 1;
        // }
        // true
    }
}

impl Form for BorderLayout {
    fn get_bounding_box(&self) -> &sizes::BoundingBox {
        &self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: sizes::BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }
    fn get_children(&self) -> &Vec<Box<Form>> {
        
    }
    fn set_child(&mut self, child: Box<Form>) -> () {
        
    }
    fn is_interactive(&mut self) -> bool {
        false
    }
    fn draw(&self) -> () {
        // should we draw sth explicitly here?
    }
}