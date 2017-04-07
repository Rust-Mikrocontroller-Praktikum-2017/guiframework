use collections::boxed::Box;
use collections::Vec;

use util::sizes::BoundingBox;

pub trait Form {
    fn get_bounding_box(&self) -> &BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_border_width(&self) -> u32;
    fn set_border_width(&mut self, width: u32) -> ();
    fn get_children(&mut self) -> &mut Vec<Box<Form>>;
    fn set_child(&mut self, child: Box<Form>) -> ();
    fn is_clickable(&mut self) -> Option<&mut Clickable>;
    fn draw(&self) -> ();
}

pub trait Clickable {
    fn click(&mut self) -> ();
}
