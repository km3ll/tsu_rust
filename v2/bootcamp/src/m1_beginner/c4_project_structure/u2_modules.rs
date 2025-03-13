pub fn modules() {
    println!("----------");
    println!("Modules");

    /**
     * Modules
     * - Contain items: functions, structs, enums, traits, etc
     * - Explicitly defined using the 'mod' keyword
     * - Not mapped to the file system
     * - A single file could have multiple modules
     * - Allow conditional compilation
     */
    println!(" > #![allow(unused)]");

    /**
     * Sub-Modules
     * - Must be declared within the parent module
     */
    println!(" > mod database (database.rs)");
}