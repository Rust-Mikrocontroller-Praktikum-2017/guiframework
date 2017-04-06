#[macro_use]
extern crate collections;



//#[derive(Debug)]
use shape

struct FlowLayout {
    bounding_box: (x_min: u32, y_min: u32, width: u32, height: u32);
    elements: Vec<Box<Shape>>;
}

trait AddForm {
    fn add_form() -> bool;
}

trait DrawArea {
    fn draw_area() -> bool;
}

impl DrawArea for FlowLayout {
    fn draw_area() -> bool {
        for i in elements {
            elements.draw();
        }
    }
}

impl AddForm for FlowLayout {
    fn add_form(Shape f) -> bool {
        elements.push(f);
    }
}