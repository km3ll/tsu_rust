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
        name: String::from("John Wick"),
        age: 30,
    };
    println!(" > person: {person:?}");
    let info = person.info();
    println!(" > info: {info:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_traits() {
        traits()
    }
}
