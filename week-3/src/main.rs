enum Animal {
    Dog(Dog),
    Cat(Cat)
}

struct Dog{}
struct Cat{}

trait Say {
    fn say(&self);
}

impl Say for Animal {
    fn say(&self) {
        match self {
            Animal::Dog(dog) => dog.say(),
            Animal::Cat(cat) => cat.say()
        }
    }
}

impl Say for Dog {
    fn say(&self) {
        println!("멍멍")
    }
}

impl Say for Cat {
    fn say(&self) {
        println!("야옹")
    }
}

fn main() {
    let animals = vec![Animal::Dog(Dog {}), Animal::Cat(Cat {})];
    for animal in animals {
        animal.say();
    }
}
