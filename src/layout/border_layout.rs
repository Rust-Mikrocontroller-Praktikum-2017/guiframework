use forms::form::*;
use util::bounding_box::BoundingBox;
use util::*;
use collections::boxed::Box;
use collections::Vec;

use util::layout_funcs::BorderArea;
use util::sizes;

use draw;
use lcd::Color;


pub struct BorderLayout {
    pub bounding_box: bounding_box::BoundingBox,
    outer_bounding_box: bounding_box::BoundingBox,
    pub top_element: Option<Box<Form>>,
    pub bottom_element: Option<Box<Form>>,
    pub left_element: Option<Box<Form>>,
    pub right_element: Option<Box<Form>>,
    pub center_element: Option<Box<Form>>,
    movable: bool,
}

impl BorderLayout {
    pub fn new(bb: bounding_box::BoundingBox) -> BorderLayout {
        BorderLayout {
            bounding_box: bb,
            outer_bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: sizes::RES_X,
                height: sizes::RES_Y,
            },
            top_element: None,
            bottom_element: None,
            left_element: None,
            right_element: None,
            center_element: None,
            movable: false,
        }
    }

    pub fn draw_area(&self) -> bool {
        let all_el = [&self.top_element,
                      &self.bottom_element,
                      &self.left_element,
                      &self.right_element,
                      &self.center_element];
        //for i in all_el.into_iter() {
        for i in all_el.into_iter() {
            match i {
                // why two references??? thought into_iter would not create another?? very strange??
                // dereferencing is prohibited due to moving... this is also weird??
                &&Some(ref el) => el.draw(),
                &&None => {}
            }
            //i.draw();
        }
        true
    }

    pub fn add_form(&mut self, mut f: Box<Form>, pos: BorderArea) -> bool {
        match pos {
            BorderArea::Top => {
                let width = self.bounding_box.width;
                let height = self.bounding_box.height / 3;
                let bb = bounding_box::BoundingBox {
                    x: self.bounding_box.x,
                    y: self.bounding_box.y,
                    width: width,
                    height: height,
                };
                f.set_bounding_box(bb);
                self.top_element = Some(f);
            }
            BorderArea::Bottom => {
                let width = self.bounding_box.width;
                let height = self.bounding_box.height / 3;
                let bb = bounding_box::BoundingBox {
                    x: self.bounding_box.x,
                    y: self.bounding_box.y + height * 2,
                    width: width,
                    height: height,
                };
                f.set_bounding_box(bb);
                self.bottom_element = Some(f);
            }
            BorderArea::Left => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = bounding_box::BoundingBox {
                    x: self.bounding_box.x,
                    y: self.bounding_box.y + height,
                    width: width,
                    height: height,
                };
                f.set_bounding_box(bb);
                self.left_element = Some(f);
            }
            BorderArea::Right => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = bounding_box::BoundingBox {
                    x: self.bounding_box.x + 2 * width + 1,
                    y: self.bounding_box.y + height,
                    width: width,
                    height: height,
                };
                f.set_bounding_box(bb);
                self.right_element = Some(f);
            }
            BorderArea::Center => {
                let width = self.bounding_box.width / 3;
                let height = self.bounding_box.height / 3; // ?
                let bb = bounding_box::BoundingBox {
                    x: self.bounding_box.x + width + 1,
                    y: self.bounding_box.y + height,
                    width: width,
                    height: height,
                };
                f.set_bounding_box(bb);
                self.center_element = Some(f);
            }
        }

        true
    }
}

impl Form for BorderLayout {
    fn get_bounding_box(&mut self) -> &mut bounding_box::BoundingBox {
        &mut self.bounding_box
    }

    fn set_bounding_box(&mut self, bounding_box: bounding_box::BoundingBox) -> () {
        self.bounding_box = bounding_box;
    }

    fn set_outer_bounding_box(&mut self, bounding_box: BoundingBox) {
        self.outer_bounding_box = bounding_box;
    }

    fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a> {
        // change interface, actually if these are None, they can be ignored, I guess?
        let opts = vec![&mut self.top_element,
                        &mut self.bottom_element,
                        &mut self.left_element,
                        &mut self.right_element,
                        &mut self.center_element];
        let mut res: Vec<&'a mut Form> = Vec::new();
        for i in opts {
            match *i {
                Some(ref mut el) => res.push(&mut **el),
                None => {}
            }
        }

        Box::new(res.into_iter())
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

    fn draw(&self) -> () {
        self.draw_area();
    }

    fn clear(&self) -> () {
        let color = Color::rgba(0, 0, 0, 0);
        draw::fill_rectangle(self.bounding_box.x,
                             self.bounding_box.y,
                             self.bounding_box.width,
                             self.bounding_box.height,
                             color);
    }

    fn move_form(&mut self, dir_x: i32, dir_y: i32, top: bool) {
        self.clear();

        let outer_if_top = if top {
            Some(&self.outer_bounding_box)
        } else {
            None
        };

        let (moved_x, moved_y) = self.bounding_box
            .move_in_direction(dir_x, dir_y, outer_if_top);

        {
            let opts = vec![&mut self.top_element,
                            &mut self.bottom_element,
                            &mut self.left_element,
                            &mut self.right_element,
                            &mut self.center_element];

            for i in opts {
                if let &mut Some(ref mut form) = i {
                    form.move_form(moved_x, moved_y, false);
                }
            }
        }

        self.draw();
    }
}
