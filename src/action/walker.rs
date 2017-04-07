use core::ops::DerefMut;

use forms::form::Form;

pub fn walk(root: &mut Form, x: u16, y: u16) {
    for child in root.get_children() {
        walk(child.deref_mut(), x, y);
    }
    
    if root.get_bounding_box().is_in_bound(x as u32, y as u32) {
        match root.is_clickable() {
            None => (),
            Some(clickable) => clickable.click(),
        }
    }
}
