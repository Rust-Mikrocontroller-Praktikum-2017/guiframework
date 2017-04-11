use forms::form::*;
use util::*;
use collections::boxed::Box;
use collections::Vec;
use util::bounding_box::BoundingBox;

use util::layout_funcs::BorderArea;

use draw;
use lcd::Color;

pub struct MoveBox {
    pub bounding_box: bounding_box::BoundingBox,
    // Boxen direkt annehmen, also Nutzer allokiert ne Box und den Zeiger darauf füge ich hinzu
    pub elements: Vec<Box<Form>>,
    movable: bool,
    border_color: Color,
}

impl MoveBox {
    pub fn new(bounding_box: BoundingBox, movable: bool) -> MoveBox {
        MoveBox{elements: Vec::<Box<Form>>::new(), bounding_box: bounding_box, movable: movable, border_color: Color::from_hex(0xFFFFFF)}
    }
    pub fn draw_area(&self) -> bool {
        draw::draw_rectangle(self.bounding_box.x, self.bounding_box.y, self.bounding_box.width, self.bounding_box.height, self.border_color);
        for i in &self.elements {
            i.draw();
        }
        true
    }

    pub fn add_form(&mut self, f: Box<Form>) -> bool {
        self.elements.push(f);
        let index = self.elements.len() - 1;
        self.elements[index].get_bounding_box().rebase_to_outer_box(&self.bounding_box);
        true
    }
}



impl Form for MoveBox {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }
    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        let mut res: Vec<&'a mut Form> = Vec::new();

        for i in &mut self.elements {
            res.push(&mut **i);
        }

        Box::new(res.into_iter())

        //Box::new(self.elements.iter_mut().map(|x| &mut **x))
    }
    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        None
    }
    fn is_movable(&mut self) -> bool {
        self.movable
    }
    fn set_movable(&mut self, movable: bool) -> () {
        self.movable = movable;
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

    fn move_form(&mut self, dir_x: i32, dir_y: i32) {
        // make recursive!!
        let (moved_x, moved_y) = self.bounding_box.move_in_direction(dir_x, dir_y);
        
        for i in &mut self.elements {
            i.move_form(moved_x, moved_y);
        }
        
    }
}