//! # Iterators - Part 1

use std::iter::Rev;
use std::slice::Iter;

fn iterators() {
    let n1 = r#"
    ---
    pod: Iterators
    - Objects that produce sequences of values
    - Their execution is lazy
    - Double references to get values `&&`
    ---"#;
    println!("{n1}");

    println!("Iterators");

    let mut vec1 = vec![1, 2, 3];
    println!(" > vec1: {vec1:?}");

    let mut iter1 = vec1.iter();
    println!(" > iter1: {iter1:?}");

    println!(" > next: {:?}", iter1.next());
    println!(" > vec1: {vec1:?}");
}

fn iterators_any() {
    println!("Iterators");

    let vec2 = vec![0, 1, 2, 3, 4, 5];
    println!(" > vec2: {vec2:?}");

    let res2 = vec2.iter().any(|&x| x > 10);
    println!(" > any(greater than 10): res2: {res2}");
}

fn iterators_all() {
    println!("Iterators");

    let vec3 = vec![2, 4, 6, 8, 10];
    println!(" > vec3: {vec3:?}");

    let res3 = vec3.iter().all(|&x| x % 2 == 0);
    println!(" > all(are even) res3: {res3:?}");
}

fn iterators_find() {
    println!("Iterators");

    let vec4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    println!(" > vec4: {vec4:?}");

    let res4 = vec4.iter().find(|&&x| x == 7);
    println!(" > find(7): {res4:?}");
}

fn iterators_position() {
    println!("Iterators");

    let vec5 = vec![10, 2, 7, 24, 76, 43, 0];
    println!(" > vec5: {vec5:?}");

    let res5 = vec5.iter().position(|&x| x == 43);
    println!(" > position(of 43): {res5:?}");
}

fn iterators_reverse_position() {
    println!("Iterators");

    let vec6 = vec![10, 2, 7, 24, 76, 43, 0];
    println!(" > vec6: {vec6:?}");

    let res6 = vec6.iter().rposition(|&x| x == 43);
    println!(" > reverse position(of 43): {res6:?}");
}

fn iterators_max() {
    println!("Iterators");

    let vec7 = vec![610, 298, 781, 243, 776, 493, 120];
    println!(" > vec7: {vec7:?}");

    let res7 = vec7.iter().max();
    println!(" > max(): {res7:?}");
}

fn iterators_min() {
    println!("Iterators");

    let vec8 = vec![610, 298, 781, 243, 776, 493, 120];
    println!(" > vec8: {vec8:?}");

    let res8 = vec8.iter().min();
    println!(" > min(): {res8:?}");
}

fn iterators_reverse() {
    println!("Iterators");

    let vec9 = vec![61, 29, 81, 23, 76, 93, 20];
    println!(" > vec9: Vec<i32>: {vec9:?}");

    let res9: Rev<Iter<i32>> = vec9.iter().rev();
    println!(" > reverse iterator direction rev(): vec9: Rev<Vec<i32>>: {vec9:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iterators() {
        iterators()
    }

    #[test]
    fn run_iterators_any() {
        iterators_any()
    }

    #[test]
    fn run_iterators_all() {
        iterators_all()
    }

    #[test]
    fn run_iterators_find() {
        iterators_find()
    }

    #[test]
    fn run_iterators_position() {
        iterators_position()
    }

    #[test]
    fn run_iterators_reverse_position() {
        iterators_reverse_position()
    }

    #[test]
    fn run_iterators_max() {
        iterators_max()
    }

    #[test]
    fn run_iterators_min() {
        iterators_min()
    }

    #[test]
    fn run_iterators_reverse() {
        iterators_reverse()
    }
}
