// Item is defined as an associated type, which are similar to generics except
// that they are restricted to one concrete type per trait implementation.
trait IteratorWithAssociatedType {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct StructWithAssociatedType {}

impl IteratorWithAssociatedType for StructWithAssociatedType {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// Using generics
trait IteratorWithGenerics<Item> {
    fn next(&mut self) -> Option<Item>;
}

struct StructWithGenerics {}

impl IteratorWithGenerics<String> for StructWithGenerics {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl IteratorWithGenerics<i32> for StructWithGenerics {
    fn next(&mut self) -> Option<i32> {
        None
    }
}

// Another common trait
// For example, a vector is a collection that you can turn into an iterator
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

fn main() {

    // The iterator design pattern allows different types to have a commmon interface
    // for iteration, while abstracting away how that iteration is implemented.
    println!("Hello, world!");

    // Error: cannot infer type for type parameter 'Item'
    // We have two implementations of IteratorWithGenerics
    let mut struct_with_generics = StructWithGenerics {};
    //let item1 = struct_with_generics.next();

    // We can define which result type we expect
    // However, it doesn't makes sense to iterate and sometimes get an String and
    // sometime get an integer.
    let item2: Option<i32> = struct_with_generics.next();

    // We can remove the type
    let mut struct_with_associated_type = StructWithAssociatedType {};
    let item3 = struct_with_associated_type.next();

}