use core::ops::DerefMut;

use forms::form::Form;
use application::app::App;

pub fn walk(root: &mut Form, x: i32, y: i32, context: &mut App) {
    for mut child in root.get_children() {
        walk(child.deref_mut(), x, y, context);
    }

    if root.get_bounding_box().is_in_bound(x, y) {
        match root.is_clickable() {
            None => (),
            Some(clickable) => clickable.click(context),
        }
    }
}
