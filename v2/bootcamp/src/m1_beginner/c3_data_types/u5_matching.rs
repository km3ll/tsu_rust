pub fn matching() {
    println!("----------");
    println!("Matching");

    /**
     * Match
     * - Flow-control operator
     * - Allows to compare a value against the series of patterns (match arms)
     *   to determine which code path to execute
     * - Exhaustive on match arms
     */
    println!("Age");
    let age: i32 = 35;
    match age {
        // Literal value
        1 => println!(" > One!"),
        // Range
        13..=19 => println!(" > Teenager!"),
        // Match all binding value
        x => println!(" > Other: {}!", x),
        // Match all without binding value
        // _ => println!("Other age"),
    }

    println!("Command");
    let cmd1 = Command::Undo;
    let cmd2 = Command::Redo;
    let cmd3 = Command::AddText(String::from("Ferris"));
    let cmd4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b")
    };

    println!(" > {}", cmd1.serialize());
    println!(" > {}", cmd2.serialize());
    println!(" > {}", cmd3.serialize());
    println!(" > {}", cmd4.serialize());
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String
    }
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
            Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
            Command::AddText(text) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{text}\" \
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": \"{line}\", \
                        \"column\": \"{column}\"\
                    }}"
                )
            },
            Command::Replace { from, to} => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        }
    }
}