use std::fs::File;

fn warn_gitignore_exists() -> Result<(), i32> {
    println!("⛔😱 Sorry, but a `.gitignore` already exists in this folder!");
    Err(1)
}

fn create_gitignore() -> Result<(), i32> {
    let f = File::create(".gitignore");
    match f {
        Ok(_) => {
            println!("✨ 🍰 ✨ Created your gitignore!");
            Ok(())
        },
        Err(_) => {
            println!("⛔😱 Oh no! `.gitignore` could not be created!");
            Err(1)
        }
    }
}

fn main() -> Result<(), i32> {
    let f = File::open(".gitignore");
    match f {
        Ok(_) => warn_gitignore_exists(),
        Err(_) => create_gitignore(),
    }
}
