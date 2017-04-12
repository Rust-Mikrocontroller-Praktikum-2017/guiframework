use collections::boxed::Box;
use collections::Vec;

use draw;
use forms::form::*;
use lcd::Color;
use util::*;
use util::bounding_box::BoundingBox;

pub struct VerticalLayout {
    pub bounding_box: bounding_box::BoundingBox,
    outer_bounding_box: bounding_box::BoundingBox,
    pub elements: Vec<Box<Form>>,
    movable: bool,
    pub proportions: Vec<i32>,
}

impl VerticalLayout {
    pub fn new(bounding_box: BoundingBox) -> VerticalLayout {
        VerticalLayout {
            bounding_box: bounding_box,
            outer_bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: sizes::RES_X,
                height: sizes::RES_Y,
            },
            elements: Vec::new(),
            movable: false,
            proportions: Vec::new(),
        }
    }

    pub fn draw_area(&self) -> bool {
        for i in &self.elements {
            i.draw();
        }

        true
    }

    pub fn add_form(&mut self, f: Box<Form>) -> bool {
        // f.set_outer_bounding_box(self.bounding_box.clone());
        self.elements.push(f);
        self.proportions.push(1);
        self.update_proportions();
        true
    }
    
    fn update_proportions(&mut self) -> bool {
        if self.proportions.len() == 0 {
            return false;
        }
        let mut sum = 0;
        for i in &self.proportions {
            sum = sum + i;
        }
        let proportions = &mut self.proportions;
        let width = self.bounding_box.width;
        let height = self.bounding_box.height;
        let mut cur_y = self.bounding_box.y;
        let mut added_height = 0;
        for i in 0..&self.elements.len() - 1 {
            let next_height = (proportions[i] * height) / sum;
            added_height += next_height;
            let next_y = cur_y;
            cur_y += next_height;
            let bb = BoundingBox {
                x: self.bounding_box.x,
                y: next_y,
                width: self.bounding_box.width,
                height: next_height,
            };

            self.elements[i].set_bounding_box(bb.clone());
            self.elements[i].set_outer_bounding_box(bb);
        }
        let bb = BoundingBox {
            x: self.bounding_box.x,
            y: cur_y,
            width: self.bounding_box.width,
            height: self.bounding_box.height - added_height,
        };
        self.elements[proportions.len() - 1].set_bounding_box(bb);
        self.elements[proportions.len() - 1].set_outer_bounding_box(self.bounding_box.clone());
        true
    }

    pub fn set_proportions(&mut self, proportions: Vec<i32>) -> bool {
        if proportions.len() != self.elements.len() {
            return false;
        }
        self.proportions = proportions;
        self.update_proportions()
    }
}

impl Form for VerticalLayout {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
        self.update_proportions();
        // for el in &mut self.elements {
            // el.set_outer_bounding_box(self.bounding_box.clone());
        // }
    }

    fn set_outer_bounding_box(&mut self, bounding_box: BoundingBox) {
        self.outer_bounding_box = bounding_box;
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        let mut res: Vec<&'a mut Form> = Vec::new();

        for i in &mut self.elements {
            res.push(&mut **i);
        }

        Box::new(res.into_iter())
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        None
    }

    fn is_movable(&mut self) -> bool {
        self.movable
    }

    fn set_movable(&mut self, value: bool) -> () {
        self.movable = value;
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

    fn move_form(&mut self, dir_x: i32, dir_y: i32, top: bool) {
        self.clear();

        let outer_if_top = if top {
            Some(&self.outer_bounding_box)
        } else {
            None
        };

        let (moved_x, moved_y) = self.bounding_box
            .move_in_direction(dir_x, dir_y, outer_if_top);

        for i in &mut self.elements {
            i.set_outer_bounding_box(self.bounding_box.clone());
            i.move_form(moved_x, moved_y, false);
        }

        self.draw();
    }
}
