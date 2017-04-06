pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub trait Form {
    fn get_bounding_box(&self) -> &BoundingBox;
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> ();
    fn get_clickable(&self) -> bool;
    fn set_clickable(&mut self, value: bool) -> ();
    fn draw(&self) -> bool;
}
