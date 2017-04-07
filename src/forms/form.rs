use util::sizes::BoundingBox;
use collections::boxed::Box;
use collections::Vec;

pub trait Form {
    fn get_bounding_box(&self) -> &BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_children(&mut self) -> &mut Vec<Box<Form>>;
    fn set_child(&mut self, child: Box<Form>) -> ();
    fn is_clickable(&mut self) -> Option<&mut Clickable>;
    fn draw(&self) -> ();
}

pub trait Clickable {
    fn set_action_on_click(&mut self, callback: fn() -> ()) -> ();
    fn click(&mut self) -> ();
}
