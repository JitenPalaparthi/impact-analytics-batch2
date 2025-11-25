use std::{fs::File, io::Write};

fn main() {
    let file_result = File::create("demo.txt");

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let data = "Hello Impact Analytics!";
    match file.write_all(data.as_bytes()) {
        Ok(()) => println!("data is successfully wwritten to the file"),
        Err(e) => println!("There seems to be an error writing to the fille {}", e),
    }
}

// Result is a enum
// T
// E

// Result
// Ok(T)
// Err(E)

// main also can return Result