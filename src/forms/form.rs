use util::sizes::BoundingBox;
use collections::boxed::Box;
use collections::Vec;

pub trait Form {
    fn get_bounding_box(&self) -> &BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_children(&self) -> &Vec<Box<Form>>;
    fn set_child(&mut self, child: Box<Form>) -> ();
    fn is_interactive(&mut self) -> bool;
    fn draw(&self) -> ();
}

// pub trait FormActionClicked {
//     fn register_callback_on_click(&mut self, f: &Fn)
// }
