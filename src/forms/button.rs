use collections::boxed::Box;
use collections::Vec;
use forms::form::Form;
use util::sizes::BoundingBox;
use draw::draw_rectangle;

pub struct Button {
    bounding_box: BoundingBox,
    interactive: bool,
    children: Vec<Box<Form>>,
}

impl Button {
    pub fn new(bounding_box: BoundingBox) -> Button {
        Button {
            bounding_box: bounding_box,
            interactive: true,
            children: Vec::new(),
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

    fn get_children(&self) -> &Vec<Box<Form>> {
        &self.children
    }

    fn set_child(&mut self, child: Box<Form>) -> () {
        self.children.push(child);
    }

    fn is_interactive(&mut self) -> bool {
        self.interactive
    }

    fn draw(&self) -> () {
        draw_rectangle(self.bounding_box.x, self.bounding_box.y, self.bounding_box.width, self.bounding_box.height, 0b1_11111_00000_00000);
    }
}
