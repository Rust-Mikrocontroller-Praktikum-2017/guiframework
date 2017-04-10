use collections::boxed::Box;
use collections::Vec;

use util::bounding_box::BoundingBox;

pub trait Form {
    fn get_bounding_box(&mut self) -> &mut BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_border_width(&self) -> i32;
    fn set_border_width(&mut self, width: i32) -> ();
    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a>;
    fn is_clickable(&mut self) -> Option<&mut Clickable>;
    fn is_movable(&mut self) -> Option<&mut Movable>;
    fn clear(&self) -> ();
    fn draw(&self) -> ();
}

pub trait Movable {
    fn move_form(&mut self, dir_x: i32, dir_y: i32);
}

pub trait Clickable {
    fn click(&mut self) -> ();
}
