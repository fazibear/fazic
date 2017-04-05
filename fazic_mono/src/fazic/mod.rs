pub mod screen;

pub struct Fazic {
    pub screen: screen::Screen
}

impl Fazic {
    pub fn new() -> Fazic {
        Fazic {
            screen: screen::Screen::new()
        }
    }
}
