use collections::boxed::Box;
use core::iter;
use stm32f7;
use stm32f7::lcd::Color;
use stm32f7::lcd::TextWriter;

use draw;
use draw::draw_pixel_on_text_layer;
use forms::form::Clickable;
use forms::form::Form;
use util::bounding_box::BoundingBox;

pub struct Label {
    bounding_box: BoundingBox,
    outer_bounding_box: BoundingBox,
    child: Option<Box<Form>>,
    movable: bool,
    text: &'static str,
}

impl Label {
    pub fn new(bounding_box: BoundingBox, text: &'static str) -> Label {
        let x = bounding_box.x;
        let y = bounding_box.y;
        let width = bounding_box.width;
        let height = bounding_box.height;

        Label {
            bounding_box: bounding_box,
            outer_bounding_box: BoundingBox {
                x: x,
                y: y,
                width: width,
                height: height,
            },
            child: None,
            movable: false,
            text: text,
        }
    }
}

impl Form for Label {
    fn get_bounding_box(&mut self) -> &mut BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn set_outer_bounding_box(&mut self, bounding_box: BoundingBox) {
        self.outer_bounding_box = bounding_box.clone();
        self.set_bounding_box(bounding_box);
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        Box::new(iter::empty())
    }

    fn is_clickable(&mut self) -> Option<&mut Clickable> {
        None
    }

    fn is_movable(&mut self) -> bool {
        self.movable
    }

    fn set_movable(&mut self, value: bool) -> () {
        self.movable = value;
    }

    fn clear(&self) -> () {
        let extra = 20;
        for x in self.bounding_box.x - extra..self.bounding_box.x + self.bounding_box.width + extra {
            for y in self.bounding_box.y - extra..self.bounding_box.y + self.bounding_box.height + extra {
                draw_pixel_on_text_layer(x, y, Color::rgba(0, 0, 0, 0));
            }
        }
    }

    fn draw(&self) -> () {
        stm32f7::with_stdout(|stdout| {
            let (width, height) = stdout.width_height(self.text);
            let (x_center, y_center) = self.bounding_box.get_center();

            let x_offset = x_center - width as i32 / 2;
            let y_offset = y_center - height as i32 / 2;

            if x_offset < 0 || y_offset < 0 {
                return;
            }

            stdout.set_offset(x_offset as usize, y_offset as usize);
            stdout.print_str(self.text);
        });
    }

    fn move_form(&mut self, dir_x: i32, dir_y: i32, top: bool) {
        self.clear();

        let outer_if_top = if top {
            Some(&self.outer_bounding_box)
        } else {
            None
        };

        self.bounding_box
            .move_in_direction(dir_x, dir_y, outer_if_top);

        self.draw();
    }
}
