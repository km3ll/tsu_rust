//! # Iterators - Part 2

fn iterators() {
    let n1 = r#"
    pod: Iterators
    - The collect function transforms an iterator into a collection (arrays, vectors)
    ---"#;
    println!("{n1}");
}

fn iterators_collect() {
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res1: Vec<&i32> = vec1.iter().filter(|&x| *x % 2 > 0).collect::<Vec<&i32>>();
    println!("Iterators");
    println!(" > filter > collect: {:?}", res1);
}

fn iterator_into_iter() {
    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res2: Vec<i32> = vec2
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>();
    println!("Iterators");
    println!(" > into_iter > collect: {:?}", res2);
}

fn iterator_clone() {
    let vec3: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec4: Vec<i32> = vec3.clone();
    println!("Iterators");
    println!(" > vec3 {:?}, clone vec4: {:?}", vec3, vec4);
}

fn iterator_map() {
    println!("Iterators");
    let vec5: Vec<i32> = vec![65, 97, 43, 29, 88];
    let vec6 = vec5.iter().map(|x| x * 11).collect::<Vec<i32>>();
    println!(" > map vec5: {:?}", vec5);
    println!(" > map vec6: {:?}", vec6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iterators() {
        iterators()
    }

    #[test]
    fn run_iterators_collect() {
        iterators_collect()
    }

    #[test]
    fn run_iterator_into_iter() {
        iterator_into_iter()
    }

    #[test]
    fn run_iterator_clone() {
        iterator_clone()
    }

    #[test]
    fn run_iterator_map() {
        iterator_map()
    }
}
