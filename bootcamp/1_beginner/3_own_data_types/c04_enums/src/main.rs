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
    Repace {
        from: String, 
        to: String
    }
}

impl Command {

    fn serialize(&self) -> String {
        match self {
            Command::Undo => {
                String::from("Undo")
            },
            Command::Redo => {
                String::from("Redo")
            },
            Command::AddText(text) => {
                String::from("AddText")
            },
            Command::MoveCursor(x, y) => {
                String::from("MoveCursor")
            },
            Command::Repace { from, to } => {
                String::from("Replace")
            }
        }
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

    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("Hello"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Repace {
        from: String::from("Hello"),
        to: String::from("Hello, pod!")
    };

    let json_String = cmd3.serialize();
    println!("JSON string: {:?}", json_String);

}