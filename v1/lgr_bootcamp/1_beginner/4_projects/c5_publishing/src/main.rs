
// Publishing a package into crates.io
fn main() {
    /**
     * 1. Login to crates.io with github account
     * 2. Generate an API token
     * 3. Run cargo login <token>
     * 4. Run cargo publish
     *    - changes must be commited to github
     *    - required metadata inside Cargo.toml: description and license
     *    - the name of the package must be unique
     *    - publishing a package is permanent, forever
     * 5. Versions can be yanked/unyanked
     *    - cargo yank --vers 0.1.0
     *    - cargo yank --vers 0.1.0 --undo
     * */
    println!("Hello, world!");
}