use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;

use util::layout_funcs::DrawArea;
use util::layout_funcs::AddForm;

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
