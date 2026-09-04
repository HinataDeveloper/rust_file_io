use std::fs;

fn main() {
    println!("\n");

    let source_file = "raphael.txt";
    let destination_file = "temp.txt";

    let result = fs::rename(source_file, destination_file);
    match result {
        Ok(_) => println!("file was renamed successfully ..."),
        Err(err) => eprintln!("Error: {}", err),
    }

    println!("\nThe End ...\n")
}
