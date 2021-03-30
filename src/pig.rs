use crate::animals::*;

pub struct Pig { weight: u32, name: &'static str }

impl Pig {
    fn eat(&mut self, grule: u32) {
        println!("{} eats {} pounds of grule", self.name, grule);
        self.weight = self.weight + grule;
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

impl AnimalFactory {
    pub fn new(name: &'static str) -> Pig {
        Pig::new(name)
    }

    pub fn action(p: &mut Pig) {
        p.eat(5);
        p.eat(5);
    }
}

