use draw;

use forms::form::Form;
use util::sizes::BoundingBox;

/*trait Shape {
    fn position() -> (u32, u32);
    fn set_position(u32, u32) -> bool;
    fn color() -> u16;
    fn bounding_box() -> (u32, u32, u32, u32);
    fn draw() -> bool;
}*/

struct Point {
    x: u32, 
    y: u32
}

struct Size {
    width: u32,
    height: u32
}

struct Rectangular {
    position: Point,
    size: Size,
    bounding_box: BoundingBox,
    color: u16
}

impl Rectangular {
    pub fn new(x_pos: u32, y_pos:u32, width: u32, height:u32, color: u16) -> Rectangular {
        Rectangular {
            position: Point{x:x_pos, y: y_pos},
            size:Size{width:width, height:height},
            color:color,
            bounding_box: BoundingBox{x:x_pos, y:y_pos, width:width, height:height},
        }
    }
}

impl Form for Rectangular {
    fn get_bounding_box(&self) -> &BoundingBox {
    }
    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {

    }
    fn get_clickable(&self) -> bool {

    }
    fn set_clickable(&mut self, value: bool) -> () {

    }
    fn get_child(&self) -> Option<&Form> {
        
    }
    fn draw(&self) -> bool {
        draw_rectangle(self.position.x, self.position.y, self.size.width, self.size.height, self.color);
    }
}