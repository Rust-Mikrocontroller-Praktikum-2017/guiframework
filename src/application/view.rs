use collections::boxed::Box;
use core::iter;
use stm32f7::lcd::Color;

use draw::fill_rectangle;
use forms::form::Form;
use forms::form::Clickable;
use util::bounding_box::BoundingBox;
use util::sizes;

pub struct View {
    bounding_box: BoundingBox,
    outer_bounding_box: BoundingBox,
    child: Box<Form>,
    movable: bool,
}

impl View {
    pub fn new(child: Box<Form>) -> View {
        View {
            bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: sizes::RES_X,
                height: sizes::RES_Y,
            },
            outer_bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: sizes::RES_X,
                height: sizes::RES_Y,
            },
            child: child,
            movable: false,
        }
    }
}

impl Form for View {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn set_outer_bounding_box(&mut self, bounding_box: BoundingBox) {
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        Box::new(iter::once::<&'a mut Form>(&mut *self.child))
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
        fill_rectangle(self.bounding_box.x,
                       self.bounding_box.y,
                       self.bounding_box.width,
                       self.bounding_box.height,
                       Color::rgba(0, 0, 0, 0));
    }

    fn draw(&self) -> () {
        self.child.draw();
    }

    fn move_form(&mut self, dir_x: i32, dir_y: i32, top: bool) {
        self.clear();

        let outer_if_top = if top {
            Some(&self.outer_bounding_box)
        } else {
            None
        };

        let (delta_x, delta_y) = self.bounding_box
            .move_in_direction(dir_x, dir_y, outer_if_top);

        self.child.set_outer_bounding_box(self.bounding_box.clone());
        self.child.move_form(delta_x, delta_y, false);

        self.draw();
    }
}
