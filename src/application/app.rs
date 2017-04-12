use application::view::View;

pub struct App {
    active_view: View,
}

impl App {
    pub fn new(view: View) -> App {
        App { active_view: view }
    }

    pub fn set_active_view(&mut self, view: View) {
        self.active_view = view;
    }

    pub fn get_active_view(&mut self) -> &mut View {
        &mut self.active_view
    }
}
