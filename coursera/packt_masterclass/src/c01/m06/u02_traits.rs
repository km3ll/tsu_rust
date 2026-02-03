//! # Traits and Default Implementations

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u32);

    fn greet(&self) -> () {
        println!(" > Hello, Someone!");
    }
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u32) {
        (self.name.as_str(), self.age)
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u32) {
        (self.name.as_str(), self.age)
    }

    fn greet(&self) -> () {
        println!(" > Hello, {}!", self.name);
    }
}

fn traits() {
    let n1 = r#"
    pod: Trait
    - Abstract definition of shared behavior amongst different types
    - A type's behavior consists of the function we can call on that type
    - Different types share the same behavior if we can call the same function on them
    ---"#;
    println!("{n1}");

    println!("Traits");
    let person = Person {
        name: String::from("John"),
        age: 30,
    };
    println!(" > person: {person:?}");
    let info = person.info();
    println!(" > info: {info:?}");
}

fn traits_defaults() {
    println!("Traits: default implementation");
    let person = Person {
        name: String::from("John"),
        age: 30,
    };
    person.greet();

    let student = Student {
        name: String::from("Spike"),
        age: 35,
    };
    student.greet();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_traits() {
        traits()
    }

    #[test]
    fn run_traits_defaults() {
        traits_defaults()
    }
}
