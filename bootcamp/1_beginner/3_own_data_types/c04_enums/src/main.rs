#!(allow[unused])

struct Product {
    name: String,
    category: Category,
    price: f32,
    in_stock: bool
}

// Enums allow us to define a type by enumerating its variance
enum Category {
    Books, 
    Clothing,
    Electronics
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
        let json_string = match self {
            Command::Undo => {
                String::from("{ \"cmd\": \"undo\" }")
            },
            Command::Redo => {
                String::from("{ \"cmd\": \"redo\" }")
            },
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            },
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };
        json_string
    }

}

fn main() {

    let category: Category = Category::Electronics;
    let product = Product {
        name: String::from("TV"),
        category: category,
        price: 200.98,
        in_stock: true
    };

    let cmd_1 = Command::Undo;
    let cmd_2 = Command::AddText(String::from("Hello"));
    let cmd_3 = Command::MoveCursor(22, 0);
    let cmd_4 = Command::Replace {
        from: String::from("Hello"),
        to: String::from("Hello, pod!")
    };

    println!("{}", cmd_1.serialize());
    println!("{}", cmd_2.serialize());
    println!("{}", cmd_3.serialize());
    println!("{}", cmd_4.serialize());

}