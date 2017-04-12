use collections::boxed::Box;
use core::iter;
use stm32f7::lcd::Color;

use draw::draw_rectangle;
use draw::fill_rectangle;
use forms::form::Clickable;
use forms::form::Form;
use util::bounding_box::BoundingBox;
use util::sizes;


pub struct Button {
    bounding_box: BoundingBox,
    outer_bounding_box: BoundingBox,
    child: Option<Box<Form>>,
    on_click: Option<fn(form: &mut Button) -> ()>,
    movable: bool,
    border_width: i32,
    border_color: Color,
}

impl Button {
    pub fn new(bounding_box: BoundingBox, border_width: i32) -> Button {
        Button {
            bounding_box: bounding_box,
            outer_bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: sizes::RES_X,
                height: sizes::RES_Y,
            },
            child: None,
            on_click: None,
            movable: false,
            border_width: border_width,
            border_color: Color::from_hex(0xFFFFFF),
        }
    }

    pub fn set_action_on_click(&mut self, callback: fn(form: &mut Button) -> ()) -> () {
        self.on_click = Some(callback);
    }

    pub fn set_child(&mut self, mut child: Box<Form>) -> () {
        child
            .get_bounding_box()
            .rebase_to_outer_box(self.get_bounding_box());

        child.set_outer_bounding_box(self.bounding_box.clone());

        self.child = Some(child);
    }

    pub fn get_border_width(&self) -> i32 {
        self.border_width
    }

    pub fn set_border_width(&mut self, width: i32) -> () {
        if width < 0 {
            return;
        }

        self.border_width = width;
    }
}

impl Form for Button {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn set_outer_bounding_box(&mut self, bounding_box: BoundingBox) {
        self.outer_bounding_box = bounding_box;
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        match self.child {
            None => Box::new(iter::empty()),
            Some(ref mut child) => Box::new(iter::once::<&'a mut Form>(&mut **child)),
        }
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        match self.on_click {
            None => None,
            _ => Some(self),
        }
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
        for i in 0i32..self.border_width {
            draw_rectangle(self.bounding_box.x + i,
                           self.bounding_box.y + i,
                           self.bounding_box.width - (2 * i),
                           self.bounding_box.height - (2 * i),
                           self.border_color);
        }

        match self.child {
            None => (),
            Some(ref child) => child.draw(),
        }
    }

    fn move_form(&mut self, dir_x: i32, dir_y: i32, top: bool) {
        if top {
            self.clear();
        }

        let outer_if_top = if top {
            Some(&self.outer_bounding_box)
        } else {
            None
        };

        let (delta_x, delta_y) = self.bounding_box
            .move_in_direction(dir_x, dir_y, outer_if_top);

        if let Some(ref mut child) = self.child {
            child.set_outer_bounding_box(self.bounding_box.clone());
            child.move_form(delta_x, delta_y, false);
        }

        if top {
            self.draw();
        }
    }
}

impl Clickable for Button {
    fn click(&mut self) {
        match self.on_click {
            Some(func) => func(self),
            None => (),
        }
    }
}
