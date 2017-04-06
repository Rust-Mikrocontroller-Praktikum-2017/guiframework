use util::sizes::BoundingBox;
use collections::boxed::Box;

pub trait Form {
    fn get_bounding_box(&self) -> &BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_clickable(&self) -> bool;
    fn set_clickable(&mut self, value: bool) -> ();
    fn get_child(&self) -> &Option<Box<Form>>;
    fn set_child(&mut self, child: Box<Form>) -> ();
    fn draw(&self) -> bool;
}
