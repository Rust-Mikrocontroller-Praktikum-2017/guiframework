use core::ops::DerefMut;

use forms::form::Form;
use application::app::App;

pub fn walk(root: &mut Form, x: i32, y: i32) {
    for mut child in root.get_children() {
        walk(child.deref_mut(), x, y);
    }

    if root.get_bounding_box().is_in_bound(x, y) {
        match root.is_clickable() {
            None => (),
            Some(clickable) => clickable.click(),
        }
    }
}
