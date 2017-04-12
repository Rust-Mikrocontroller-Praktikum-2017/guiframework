use collections::boxed::Box;

use layout::HorizontalLayout;
use layout::VerticalLayout;
use layout::MoveBox;
use forms::form::Form;
use forms::button::Button;
use forms::label::Label;
use util::bounding_box::BoundingBox;

fn bb(x: i32, y:i32, width: i32, height: i32) -> BoundingBox {
    BoundingBox {
        x: x,
        y: y,
        width: width,
        height: height,
    }
}

fn bb_def() -> BoundingBox {
    bb(0,0,0,0)
}

fn empty() -> Box<HorizontalLayout> {
    Box::new(HorizontalLayout::new(bb_def()))
}

fn button(string: &'static str) -> Box<Button> {
    let label = Label::new(bb_def(), string);
    let mut button = Button::new(bb_def());
    button.set_child(Box::new(label));
    Box::new(button)
}

fn middle(middle_form: Box<Form>) -> HorizontalLayout {
    let mut f = HorizontalLayout::new(bb(0,0,480,200));
    
    f.add_form(empty());
    f.add_form(middle_form);
    f.add_form(empty());
    
    f.set_proportions(vec!(1,2,1));
    f
}

pub fn form_languages() -> HorizontalLayout {
    let mut f = VerticalLayout::new(bb_def());
    f.add_form(button("English"));
    f.add_form(button("Français"));
    f.add_form(button("Deutsch"));
    f.add_form(button("Italiano"));
    f.add_form(button("Português"));
    f.add_form(button("Rust"));
    middle(Box::new(f))
}

pub fn form_skins() -> HorizontalLayout {
    let mut f = VerticalLayout::new(bb_def());
    f.add_form(button("Standard _Dark_ Theme"));
    f.add_form(button("Standard _Light_ Theme"));
    f.add_form(button("Rusty Safety Theme"));
    f.add_form(button("Rusty Zero Cost Theme "));
    f.add_form(button("Clippy Theme "));
    middle(Box::new(f))
}

pub fn form_settings() -> HorizontalLayout {
    let mut f = VerticalLayout::new(bb_def());
    f.add_form(button("Auto-capitalization"));
    f.add_form(button("Double-space period"));
    f.add_form(button("Sound on keypress"));
    f.add_form(button("Popup on keypress"));
    middle(Box::new(f))
}

pub fn form_about() -> HorizontalLayout {
    let mut f = VerticalLayout::new(bb_def());
    f.add_form(button("------------"));
    f.add_form(button("Didong"));
    f.add_form(button("Drexler"));
    f.add_form(button("Speitelsbach"));
    f.add_form(button("------------"));
    middle(Box::new(f))
}

