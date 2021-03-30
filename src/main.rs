mod animals;
#[cfg(feature = "pig")]
mod pig;
#[cfg(feature = "sheep")]
mod sheep;

use animals::*;

fn main() {
    let mut babe = AnimalFactory::new("babe");
    babe.talk();
    AnimalFactory::action(&mut babe);
    babe.talk();
}
