use std::fs::File;
use std::io::Write;

const TEMPLATE: &str = r#"
# This is a general gitignore template
debug*/
target*/
build*/
release*/

# Operating system generated files
.DS_Store
*.temp
*.cache
*.pdb
*.tmp

# Artifacts from development
*.swp
Session.vim
__pycache__
node_modules*/
"#;

fn warn_gitignore_exists() -> Result<(), i32> {
    println!("\u{26D4} Sorry, but a `.gitignore` already exists in this folder!");
    Err(1)
}

fn create_gitignore() -> Result<(), i32> {
    let f = File::create(".gitignore");
    match f {
        Ok(mut f) => {
            let w = write!(&mut f, "{}", TEMPLATE);
            match w {
                Ok(_) => {
                    println!("\u{2728} Created your gitignore!");
                    Ok(())
                },
                Err(_) => {
                    println!("\u{26D4} Oh no! `.gitignore` could not be created!");
                    Err(1)
                }
            }
        },
        Err(_) => {
            println!("\u{26D4} Oh no! `.gitignore` could not be created!");
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
