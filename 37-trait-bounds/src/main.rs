fn main() {
    let animal = Animal {
        name: Box::new(Dog),
    };
    animal.name.speak();
    let animal = Animal {
        name: Box::new(Cat),
    };
    animal.name.speak();

    let d: Dog = Dog;
    let d1 = &d;
    let animal = Animal {
        name: Box::new(*d1),
    };
    animal.name.speak();

    let sound: &'static str = "Silent"; // 'static lifetime
    let pg = PugDog { sound: &sound };
    let animal = Animal { name: Box::new(pg) };
    animal.name.speak();

    let sound: String = "Silent".to_string(); // &sound it is not a static reference
    let pg = GoldenRetriverDog { sound: &sound };
    let animal = Animal { name: Box::new(pg) };
    animal.name.speak();
}

trait Speak {
    // Trait
    fn speak(&self); // def called speak
}

struct Animal<'a> {
    name: Box<dyn Speak+ 'a >,           // Composition, owned values .. not the borrowd values
                                    // dog:Box<Dog>,
                                    // cat:Box<Cat>,




  // name1: Box<dyn Speak>, // static lifetime
}

#[derive(Copy, Clone)]
struct Dog; // MakerStruct
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Dog is barking!")
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Cat is Meaw!")
    }
}

struct PugDog<'a> {
    sound: &'a str, //reference
}

impl<'a> Speak for PugDog<'a> {
    fn speak(&self) {
        println!("Pug dog is making some sound {}!", self.sound);
    }
}

struct GoldenRetriverDog<'a> {
    sound: &'a str, //reference
}

impl<'a> Speak for GoldenRetriverDog<'a> {
    fn speak(&self) {
        println!(
            "Pug dog is making Log of  sound but now it is {}!",
            self.sound
        );
    }
}
