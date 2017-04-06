use util::sizes::BoundingBox;
use forms::form::Form;

pub struct Button {
    bounding_box: BoundingBox,
    clickable: bool,
}

impl Button {
    pub fn new(bounding_box: BoundingBox) -> Button {
        Button {
            bounding_box: bounding_box,
            clickable: true,
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

    fn get_clickable(&self) -> bool {
        self.clickable
    }

    fn set_clickable(&mut self, value: bool) -> () {
        self.clickable = value;
    }

    fn draw(&self) -> bool {
        true
    }
}
