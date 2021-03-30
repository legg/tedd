use crate::animals::*;

pub struct Sheep { naked: bool, name: &'static str }

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    pub fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> String {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }.to_string()
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl AnimalFactory {
    pub fn new(name: &'static str) -> Sheep {
        Sheep::new(name)
    }

    pub fn action(s: &mut Sheep) {
        s.shear();
    }
}
