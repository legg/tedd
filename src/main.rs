struct Sheep { naked: bool, name: &'static str }

struct Pig { weight: u32, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> String;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Pig {
    fn eat(&mut self, grule: u32) {
        self.weight = self.weight + grule;
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
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

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

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

fn main() {
    // Type annotation is necessary in this case.


    let mut babe: Sheep = Animal::new("Babe");
    // let mut babe: Pig = Animal::new("Babe");
    // TODO ^ Try removing the type annotations.

    babe.talk();
    babe.shear();
    // babe.eat(5);
    // babe.eat(5);
    babe.talk();
}
