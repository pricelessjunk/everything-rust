/// Using traits for composition.

fn main() {
    let animal_kingdom = AnimalKingdom {
        animals: vec![Dog {
            sound: String::from("Bark"),
        }],
    };

    animal_kingdom.call(); // Should print bark
}

// The trait
pub trait Animal {
    fn sound(&self);
}

// The parent of the trait
pub struct AnimalKingdom<T: Animal> {
    pub animals: Vec<T>,
}

// implementing a common function of the trait
impl<T> AnimalKingdom<T>
where
    T: Animal,
{
    pub fn call(&self) {
        for animal in self.animals.iter() {
            animal.sound();
        }
    }
}

// a normal struct
pub struct Dog {
    pub sound: String,
}

// The struct implementing the trait Animal
impl Animal for Dog {
    fn sound(&self) {
        println!("{}", self.sound);
    }
}
