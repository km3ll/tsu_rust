//! # Iterators - Part 1

fn iterators() {
    let n1 = r#"
    pod: Iterators
    - Objects that produce sequences of values
    - Their execution is lazy
    - Double references to get values `&&`
    ---"#;
    println!("{n1}");

    println!("Iterators");
    let mut vec1 = vec![1, 2, 3];
    let mut iter1 = vec1.iter();
    println!(" > iter: {iter1:?}");

    println!(" > next: {:?}", iter1.next());
    println!(" > next: {:?}", iter1.next());
    println!(" > next: {:?}", iter1.next());
    println!(" > next: {:?}", iter1.next());
}

fn iterators_any() {
    let vec2 = vec![0, 1, 2, 3, 4, 5];
    let res2 = vec2.iter().any(|&x| x > 10);
    println!("Iterators");
    println!(" > vec2: {:?}", vec2);
    println!(" > any (greater than 10): {res2}");
}

fn iterators_all() {
    let vec3 = vec![2, 4, 6, 8, 10];
    let res3 = vec3.iter().all(|&x| x % 2 == 0);
    println!("Iterators");
    println!(" > vec3: {:?}", vec3);
    println!(" > all (even): {}", res3);
}

fn iterators_find() {
    let vec4 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let res4 = vec4.iter().find(|&&x| x == 7);
    println!("Iterators");
    println!(" > vec4: {:?}", vec4);
    println!(" > find: {:?}", res4);
}

fn iterators_position() {
    let vec5 = vec![10, 2, 7, 24, 76, 43, 0];
    let res5 = vec5.iter().position(|&x| x == 43);
    println!("Iterators");
    println!(" > vec5: {:?}", vec5);
    println!(" > position: {:?}", res5);
}

fn iterators_reverse_position() {
    let vec6 = vec![10, 2, 7, 24, 76, 43, 0];
    let res6 = vec6.iter().rposition(|&x| x == 43);
    println!("Iterators");
    println!(" > vec6: {:?}", vec6);
    println!(" > rposition: {:?}", res6);
}

fn iterators_max() {
    let vec7 = vec![610, 298, 781, 243, 776, 493, 120];
    let res7 = vec7.iter().max();
    println!("Iterators");
    println!(" > vec7: {:?}", vec7);
    println!(" > max: {:?}", res7);
}

fn iterators_min() {
    let vec8 = vec![610, 298, 781, 243, 776, 493, 120];
    let res8 = vec8.iter().min();
    println!("Iterators");
    println!(" > vec8: {:?}", vec8);
    println!(" > min: {:?}", res8);
}

fn iterators_reverse() {
    let vec9 = vec![61, 29, 81, 23, 76, 93, 20];
    let res9 = vec9.iter().rev();
    println!("Iterators");
    println!(" > vec9: {:?}", vec9);
    println!(" > reverse: {:?}", res9);
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
