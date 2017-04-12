use application::view::View;
use forms::form::Form;

pub struct App {
    active_view: View,
}

impl App {
    pub fn new(view: View) -> App {
        App { active_view: view }
    }

    pub fn show_view(&mut self, view: View) {
        self.active_view.clear();
        self.active_view = view;
        self.active_view.draw();
    }

    pub fn get_active_view(&mut self) -> &mut View {
        &mut self.active_view
    }
}
