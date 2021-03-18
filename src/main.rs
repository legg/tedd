#[cfg(feature = "sheep")]
struct Sheep { naked: bool, name: &'static str }
#[cfg(feature = "pig")]
struct Pig { weight: u32, name: &'static str }
struct AnimalFactory {}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> String;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

#[cfg(feature = "sheep")]
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

#[cfg(feature = "pig")]
impl Pig {
    fn eat(&mut self, grule: u32) {
        println!("{} eats {} pounds of grule", self.name, grule);

        self.weight = self.weight + grule;
    }
}

#[cfg(feature = "sheep")]
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
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

#[cfg(feature = "pig")]
impl Animal for Pig {
    fn new(name: &'static str) -> Pig {
        Pig { name, weight: 5 }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> String {
        let mut n = String::from("O");
        let amount = self.weight / 5;
        for _ in 1..amount {
            n.push_str("o");
        }
        n.push_str("ink!");
        n
    }
}

#[cfg(feature = "pig")]
impl AnimalFactory{
    fn new(name: &'static str) -> Pig {
        Pig::new(name)
    }

    fn action(p: &mut Pig) {
        p.eat(5);
        p.eat(5);
    }
}

#[cfg(feature = "sheep")]
impl AnimalFactory {
    fn new(name: &'static str) -> Sheep {
        Sheep::new(name)
    }

    fn action(s: &mut Sheep) {
        s.shear();
    }
}

fn main() {
    let mut babe = AnimalFactory::new("babe");
    babe.talk();
    AnimalFactory::action(&mut babe);
    babe.talk();
}
