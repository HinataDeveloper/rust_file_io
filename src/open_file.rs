use std::{fs::File, io::Read};

fn main() {
    println!("\n");

    let file_name = String::from("temp.txt");

    let file_result = File::options()
        .append(false)
        .create(false)
        .read(true)
        .write(false)
        .open(file_name);

    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => panic!("Error: {err}"),
    };

    let mut buffer = String::new();

    let result = file.read_to_string(&mut buffer);
    match result {
        Ok(_) => (),
        Err(err) => panic!("Error: {}", err),
    }

    println!("file content is: {}", buffer);

    println!("\nThe End ...\n");
}
