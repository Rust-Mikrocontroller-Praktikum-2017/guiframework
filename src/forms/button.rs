use collections::boxed::Box;
use collections::Vec;
use forms::form::Form;
use forms::form::Clickable;
use util::sizes::BoundingBox;
use draw::draw_rectangle;

pub struct Button {
    bounding_box: BoundingBox,
    children: Vec<Box<Form>>,
    on_click: Option<fn(form: &mut Form) -> ()>,
    border_width: u32,
}

impl Button {
    pub fn new(bounding_box: BoundingBox, border_width: u32) -> Button {
        Button {
            bounding_box: bounding_box,
            children: Vec::new(),
            on_click: None,
            border_width: border_width,
        }
    }
}

impl Form for Button {
    fn get_bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn get_border_width(&self) -> u32 {
        self.border_width
    }

    fn set_border_width(&mut self, width: u32) -> () {
        self.border_width = width;
    }

    fn get_children(&mut self) -> &mut Vec<Box<Form>> {
        &mut self.children
    }

    fn set_child(&mut self, child: Box<Form>) -> () {
        self.children.push(child);
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        match self.on_click {
            None => None,
            _ => Some(self),
        }
    }

    fn draw(&self) -> () {
        for i in 0..self.border_width {
            draw_rectangle(self.bounding_box.x + i,
                           self.bounding_box.y + i,
                           self.bounding_box.width - (2 * i),
                           self.bounding_box.height - (2 * i),
                           0b1_00110_00110_11010);
        }

        for child in &self.children {
            child.draw();
        }
    }
}

impl Clickable for Button where {
    fn set_action_on_click(&mut self, callback: fn(form: &mut Form) -> ()) {
        self.on_click = Some(callback);
    }

    fn click(&mut self) {
        match self.on_click {
            Some(func) => func(self),
            None => (),
        }

        self.draw();
    }
}
