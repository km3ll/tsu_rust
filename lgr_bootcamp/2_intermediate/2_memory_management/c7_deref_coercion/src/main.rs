#![allow(unused)]
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {

    println!("Hello, world!");
    let s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );

    // A reference to a Box smart pointer is being coerced to a reference of
    // String, which is coerced to a string slice
    // &Box -> &String -> &str

    // It only works with types that implement the Deref trait.
    // - Deref
    // - Target
    // - ?Sized (constant size known at compile time)

    // &MySmartPointer -> &Box -> &String -> &str
    let s1: &MySmartPointer<Box<String>> = &(s);
    let s2: &Box<String> = (&*s); // one deref operator
    let s3: &String = (&**s); // two deref operators
    let s4: &str = (&***s); // three deref operators

    // Every time we use the deref operator, the deref method is called on that type.
    // The compiler will generate this sequence of calls implicitly

    print(&s);

    let mut mut_s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );
    print(&mut mut_s);

}

// This call worls because of the Implicit Deref Coercion
// For convenience this happens when references are passed to functions or methods
fn print(s: &str) {
    println!("{s}");
}