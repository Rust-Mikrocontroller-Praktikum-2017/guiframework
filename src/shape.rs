use draw;

use forms::form;
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

/*impl Shape for Rectangular {
    fn position(&self) -> (u32, u32) {
        (self.position.x, self.position.y)
    }
    fn set_position(&self, x:u32, y:u32) {
        // is this possible, or let (x, y) = position ...
        self.position.x = x;
        self.position.y = y;

        // bounding box position changes accordingly
        self.bounding_box.1 = x;
        self.bounding_box.2 = y;
    }
    fn color(&self) -> u16 {
        self.color
    }
    fn bounding_box(&self) -> (u32, u32, u32, u32) {
        self.bounding_box
    }
    fn draw(&self) -> bool {
        draw_rectangle(self.position.x, self.position.y, self.size.width, self.size.height, self.color);
    }
}*/