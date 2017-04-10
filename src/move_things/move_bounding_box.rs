use core::num;
use collections::VecDeque;
use forms::form::Form;
use collections::Vec;
use util::sizes::BoundingBox;
use util::sizes;

pub struct TouchHistory {
    // x_pos, y_pos, #ticks (round, in which touch occurred)
    cur_touches : VecDeque<(i32, i32, usize)>,
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
        TouchHistory{last_touches: VecDeque::new()}
    }

    pub fn update(&self, cur_ticks: usize, new_touches: Vec<(u16, u16)>) {
        let mut old = true;
        // pop old touches
        while old {
            let cur_el = self.cur_touches.get(0);
            if cur_ticks - cur_el[2] > 500 { // 1000ms could be made adaptable later:)
                cur_el.pop_front();
            } else {
                old = false;
            }
        }
        // push new touches
        for i in &new_touches {
            self.cur_touches.push_back((i[0], i[1], cur_ticks));
        }
    }

    pub fn check_for_object_moves(&self, &mut movable_objects: Vec<&Form>) {
        //let mut moves = Vec::new();
        let mut movements: Vec<Vec<(i32, i32, usize)>> = Vec::new();
        
        for i in &self.cur_touches {
            let mut found_match = false;
            // currently takes the first that is good enough...
            for (j, value) in &movements.enumerate() {
                let length = value.len();
                // call update and clear old touches first, but you need to know the last one...
                if get_square_distance(value[length - 1][0], value[length - 1][1], i[0], i[1]) < 7.09 {
                    movements[j].push(i);
                    found_match = true;
                }
            }
            if !found_match {
                movements.push(vec![i]);
            }
        }

        //let mut results: Vec<(&Form, i32, i32)> = Vec::new();
        for i in movements {
            let res_check = check_for_hit(movable_objects, i[0], i[1]);
            match res_check {
                Some(form) => {
                    form.move_obj(i[i.len() - 1][0], i[i.len() - 1][1]);
                    // or return this and let the caller do the actual movement...
                    //results.push((form, i[i.len() - 1][0], i[i.len() - 1][1]));
                },
                None => {},
            }
        }
    }

    pub fn check_for_directions(&self) {

    }
}

fn check_for_hit<'a>(movable_objects: &'a Vec<&Form>, x: i32, y: i32) -> Option<&'a Form> {
    for i in &movable_objects {
        if i.is_in_bound(x, y) {
            return i;
        }
    }
    None
}

fn get_square_distance(x1: u16, y1: u16, x2: u16, y2: u16) -> f32 {
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    let x2 = x2 as i32;
    let y2 = y2 as i32;

    let x = (x1 - x2) * (x1-x2);
    let y = (y1 - y2) * (y1-y2);

    let tmp = (x + y) as f32;
    //sqrt(tmp)
    tmp.sqrt()
}

fn move_bb(bb: &BoundingBox, dir_x: i32, dir_y: i32) {
    let pos_x_new = bb.x + dir_x;
    let pos_y_new = bb.y + dir_y;

    if pos_x_new < 0 {
        bb.x = 0;
    } else if pos_x_new + bb.width > sizes::MAX_X {
        bb.x = sizes::MAX_X - bb.width;
    } else {
        bb.x = pos_x_new;
    }
    if pos_y_new < 0 {
        bb.y = 0;
    } else if pos_y_new + bb.height > sizes::MAX_Y {
        bb.y = sizes::MAX_Y - bb.height;
    } else {
        bb.y = pos_y_new;
    }
}

// fn to_move mit letzten x Touches --> determine if bounding_box of first touch is to be moved.

// fn check_overlap --> check for other elements