pub fn memory_regions() {
    println!("----------");
    println!("Memory Regions");

    /**
     * Static
     * - Program's binary
     * - Static variables
     * - String literals
     * - Size: fixed size
     * - Lifetime: lifetime of program
     * - Cleanup: automatic when program terminates
     */
    println!("Static");

    /**
     * Heap
     * - Values that live beyond a function's lifetime
     * - Values accessed by multiple threads
     * - Large values
     * - Unknown size at compile time
     * - Size: dynamic
     * - Lifetime: determined by programmer
     * - Cleanup: manual
     */
    println!("Heap");

    /**
     * Stack
     * - Function arguments
     * - Local variables
     * - Known size at compile time
     * - Size: dynamic / fixed upper limit
     * - Lifetime: lifetime of function
     * - Cleanup: automatic when function returns
     */
    println!("Stack");

    /**
     * Threads
     * - Each thread has its own stack
     * - All threads share the same heap
     */
    println!("Threads")


}