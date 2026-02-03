//! # Memory Regions

pub fn memory_stack() {
    let n1 = r#"
	pod: Stack Contents
	- Function arguments
	- Local variables
	- Known size at compile time
	---
	pod: Stack Properties
	- Size: dynamic, fixed upper limit
	- Lifecycle: lifetime of a function
	- Cleanup: automatic when function returns
	---"#;
    println!("{n1}");
}

pub fn memory_heap() {
    let n1 = r#"
	pod: Heap Contents
	- Values that live beyond a function's lifetime
	- Values accessed by multiple threads
	- Large values
	- Unknown size at compile time
	---
	pod: Heap Properties
	- Size: dynamic
	- Lifecycle: determined by programmer
	- Cleanup: manual
	---"#;
    println!("{n1}");
}

pub fn memory_static() {
    let n1 = r#"
	pod: Static Memory Contents
    - Program's binary
  	- Static variables
	- String literals
	---
	pod: Static Memory Properties
	- Size: fixed
	- Lifecycle: lifetime of programm
	- Cleanup: automatic when program terminates
	---"#;
    println!("{n1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_memory_stack() {
        memory_stack();
    }

    #[test]
    fn run_memory_heap() {
        memory_heap();
    }

    #[test]
    fn run_memory_static() {
        memory_static();
    }
}
