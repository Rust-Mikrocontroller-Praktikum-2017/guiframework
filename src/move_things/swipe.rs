use core::num;
use core::iter;
use collections::VecDeque;
use forms::form::Form;
use collections::Vec;
use util::bounding_box::BoundingBox;
use util::sizes;
use util::math::isqrt;
use collections::boxed::Box;

//use arrayvec::ArrayVec;

//use font_rs::float_impls::FloatImpls;

pub struct TouchHistory {
    // x_pos, y_pos, #ticks (round, in which touch occurred)
    cur_touches: VecDeque<(i32, i32, usize)>,
}
// struct Movement {
//     source_x : i32,
//     source_y : i32,
//     dir_x : i32,
//     dir_y : i32
// }
// struct ObjectMovement {
//     //object : &'a Form,
//     source_x : i32,
//     source_y : i32,
//     dir_x : i32,
//     dir_y : i32
// }
impl TouchHistory {
    // add code here
    pub fn new() -> TouchHistory {
        TouchHistory { cur_touches: VecDeque::new() }
    }

    //pub fn update(&mut self, cur_ticks: usize, new_touches: ArrayVec<(i32, i32)>) {
    pub fn update(&mut self, cur_ticks: usize, new_touches: Vec<(i32, i32)>) {
        let mut old = true;
        // pop old touches
        while old && !self.cur_touches.is_empty() {
            //let mut cur_el = self.cur_touches.get(0).unwrap();
            if cur_ticks - self.cur_touches.get(0).unwrap().2 > 300 {
                // 1000ms could be made adaptable later:)
                self.cur_touches.pop_front();
            } else {
                old = false;
            }
        }
        // push new touches
        for i in &new_touches {
            self.cur_touches.push_back((i.0, i.1, cur_ticks));
        }

        /*print!("{}-", &self.cur_touches.len());
        for i in &self.cur_touches {
            print!("{}-", i.0);
        }
        print!("---");*/
    }

    pub fn check_for_object_moves(&self, root: &mut Form) {
        //let mut moves = Vec::new();
        let mut movements: Vec<Vec<(i32, i32, usize)>> = Vec::new();

        for i in &self.cur_touches {
            let mut found_match = false;
            // currently takes the first that is good enough...
            // 0..movements.len()
            for j in 0..movements.len() {
                let value = &mut movements[j];
                let length = value.len();
                // call update and clear old touches first, but you need to know the last one...
                if get_square_distance(value[length - 1].0, value[length - 1].1, i.0, i.1) < 40 {
                    value.push(*i);
                    found_match = true;
                }
            }
            if !found_match {
                movements.push(vec![*i]);
            }
        }

        /*for i in &movements {
            let len = i.len();
            print!("--{},{}-{},{}--", i[0].0, i[0].1, i[len - 1].0, i[len - 1].1);
        }*/

        //let mut results: Vec<(&Form, i32, i32)> = Vec::new();
        for i in movements {
            //let res_check = check_for_hit(Box::new(iter::once(root)), i[0].0, i[0].1);
            let res_check = check_for_hit(root.get_children(), i[0].0, i[0].1);
            match res_check {
                Some(form) => {
                    // draw recursively from current note.
                    /*
                     * 1) clear() at parent
                     * 2) draw() parent, // move!
                     */
















                    let delta_x = i[i.len() - 1].0 - i[0].0;
                    let delta_y = i[i.len() - 1].1 - i[0].1;

                    //form.move_form(i[i.len() - 1].0, i[i.len() - 1].1);
                    form.move_form(delta_x, delta_y, true);
                }
                None => {}
            }
        }
    }
}

fn check_for_hit<'a>(objects: Box<Iterator<Item = &'a mut Form> + 'a>,
                     x: i32,
                     y: i32)
                     -> Option<&'a mut Form> {
    // recursively search for the movable object matched most precisely, move subtree from this
    // object.
    let mut last_mov_form: Option<&'a mut Form> = None;
    //let mut found_el = false;
    for i in objects {
        let in_bb = i.get_bounding_box().is_in_bound(x, y);
        let movable = i.is_movable();
        if in_bb {
            //let ret: &'a mut Form = i;

            //iter::once::<&'a mut Form>(&mut **child)
            //fn get_children<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Form> + 'a>;
            if check_for_hit(i.get_children(), x, y).is_none() {
                if movable {
                    last_mov_form = Some(i);
                    break;
                }
            } else {
                last_mov_form = check_for_hit(i.get_children(), x, y);
            }
            //break;
        }
    }
    last_mov_form
}

/*
fn draw_recursively(note: &mut Form) {
    let move_trait = note.is_movable();
    match move_trait {
        Some(T) => {
            T.move_form(i[i.len() - 1].0, i[i.len() - 1].1);
        }
        None => {
            note.get_bounding_box()
        }
    }
}*/

fn get_square_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    // let x1 = x1 as i32;
    // let y1 = y1 as i32;
    // let x2 = x2 as i32;
    // let y2 = y2 as i32;

    let x = (x1 - x2) * (x1 - x2);
    let y = (y1 - y2) * (y1 - y2);

    let tmp = (x + y) as u32;
    //sqrt(tmp)
    isqrt(tmp) as i32
}

// fn to_move mit letzten x Touches --> determine if bounding_box of first touch is to be moved.

// fn check_overlap --> check for other elements
