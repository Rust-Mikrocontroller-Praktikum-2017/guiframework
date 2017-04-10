use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;

use util::layout_funcs::DrawArea;
use util::layout_funcs::AddForm;

pub struct HorizontalLayout {
    pub bounding_box: bounding_box::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
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
    fn add_form(&mut self, f: Box<Form>) -> bool {
        self.elements.push(f);
        let len = self.elements.len() as i32;
        let el_width = self.bounding_box.width / len;
        let mut n = 0;
        for i in &mut self.elements {
            let bb = bounding_box::BoundingBox {
                x: self.bounding_box.x + n * el_width,
                y: self.bounding_box.y,
                width: el_width,
                height: self.bounding_box.height,
            };
            i.set_bounding_box(bb);
            n += 1;
        }
        true
    }
}

impl Form for HorizontalLayout {
    fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }
    fn get_border_width(&self) -> u32 {
        self.bounding_box.width
    }
    fn set_border_width(&mut self, width: u32) -> () {
        self.bounding_box.width = width;
    }
    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        let mut res: Vec<&'a mut Form> = Vec::new();

        for i in &self.elements {
            res.push(&mut **i);
        }

        Box::new(res.into_iter())

        //Box::new(self.elements.iter_mut().map(|x| &mut **x))
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
