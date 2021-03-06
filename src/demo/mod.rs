use collections::boxed::Box;

use layout::HorizontalLayout;
use layout::VerticalLayout;
use layout::MoveBox;
use forms::form::Form;
use forms::button::Button;
use forms::label::Label;
use util::bounding_box::BoundingBox;
use util::sizes;
use application::view::View;

fn bb(x: i32, y:i32, width: i32, height: i32) -> BoundingBox {
    BoundingBox {
        x: x,
        y: y,
        width: width,
        height: height,
    }
}

fn bb_def() -> BoundingBox {
    bb(0,0,10,10)
}

fn bb_screen() -> BoundingBox {
    bb(0, 0, sizes::RES_X, sizes::RES_Y)
}

fn empty() -> Box<HorizontalLayout> {
    Box::new(HorizontalLayout::new(bb_def()))
}


fn clicked(form: &mut Button) {
        let label = Label::new(BoundingBox{
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }, "clicked!");

    for child in form.get_children() {
        child.clear();
        //child.draw();
    }
    form.set_child(Box::new(label));

    form.clear();
    form.draw();
}

fn button(string: &'static str) -> Box<Button> {
    let label = Label::new(bb_def(), string);
    let mut button = Button::new(bb_def());
    button.set_child(Box::new(label));
    //button.set_action_on_click(clicked);
    Box::new(button)
}

fn middle(middle_form: Box<Form>) -> View {
    let mut f = HorizontalLayout::new(bb_screen());
    
    f.add_form(empty());
    f.add_form(middle_form);
    f.add_form(empty());
    
    f.set_proportions(vec!(1,2,1));
    View::new(Box::new(f))
}

pub fn view_languages() -> View {
    let mut f = VerticalLayout::new(bb_screen());
    f.add_form(button("English"));
    f.add_form(button("Français"));
    f.add_form(button("Deutsch"));
    f.add_form(button("Italiano"));
    f.add_form(button("Português"));
    f.add_form(button("Rust"));
    middle(Box::new(f))
}

pub fn view_skins() -> View {
    let mut f = VerticalLayout::new(bb_screen());
    f.add_form(button("Standard Dark Theme"));
    f.add_form(button("Standard Light Theme"));
    f.add_form(button("Rusty Safety Theme"));
    f.add_form(button("Rusty Zero Cost Theme "));
    f.add_form(button("Clippy Theme "));
    middle(Box::new(f))
}

pub fn view_settings() -> View {
    let mut f = VerticalLayout::new(bb_screen());
    f.add_form(button("Auto-capitalization"));
    f.add_form(button("Double-space period"));
    f.add_form(button("Sound on keypress"));
    f.add_form(button("Popup on keypress"));
    middle(Box::new(f))
}

pub fn view_about() -> View {
    let mut f = VerticalLayout::new(bb_screen());
    f.add_form(button("------------"));
    f.add_form(button("Didong"));
    f.add_form(button("Drexler"));
    f.add_form(button("Speitelsbach"));
    f.add_form(button("------------"));
    middle(Box::new(f))
}

