use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;
use util::bounding_box::BoundingBox;


use util::layout_funcs::DrawArea;
use util::layout_funcs::AddForm;

use draw;
use lcd::Color;

pub struct VerticalLayout {
    pub bounding_box: bounding_box::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
}

impl DrawArea for VerticalLayout {
    fn draw_area(&self) -> bool {
        for i in &self.elements {
            i.draw();
        }
        true
    }
}

impl AddForm for VerticalLayout {
    fn add_form(&mut self, f: Box<Form>) -> bool {
        self.elements.push(f);
        let len = self.elements.len() as i32;
        let el_height = self.bounding_box.height / len;
        let mut n = 0;
        for i in &mut self.elements {
            let bb = bounding_box::BoundingBox {
                x: self.bounding_box.x,
                y: self.bounding_box.y + n * el_height,
                width: self.bounding_box.width,
                height: el_height,
            };
            i.set_bounding_box(bb);
            n += 1;
        }
        true
    }
}

impl Form for VerticalLayout {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }
    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        let mut res: Vec<&'a mut Form> = Vec::new();

        for i in &self.elements {
            res.push(&mut **i);
        }

        Box::new(res.into_iter())

        //self.elements.map(|x| *x).iter()
    }
    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        None
    }
    fn is_movable(&mut self) -> Option<&mut Movable> {
        None
    }
    fn clear(&self) -> () {
        let color = Color::rgba(0, 0, 0, 0);
        draw::fill_rectangle(self.bounding_box.x,
                             self.bounding_box.y,
                             self.bounding_box.width,
                             self.bounding_box.height,
                             color);
    }
    fn draw(&self) -> () {
        self.draw_area();
    }
}

impl Movable for VerticalLayout {
    fn move_form(&mut self, dir_x: i32, dir_y: i32) {
        // make recursive!!
        let (moved_x, moved_y) = self.bounding_box.move_in_direction(dir_x, dir_y);
        
        for i in &self.elements {
            i.move_form(moved_x, moved_y);
        }
        
    }
}
