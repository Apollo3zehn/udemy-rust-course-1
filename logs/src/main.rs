use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())

    // let text = fs::read_to_string("logs.txt")
    //     .expect("failed to read logs.txt");

    // let error_logs = extract_errors(text.as_str());

    // fs::write("errors.txt", error_logs.join("\n"))
    //     .expect("failed to write errors.txt");

    // println!("Wrote errors.txt")

    // match fs::read_to_string("logs.txt") {
    //     Ok(text_that_was_read) => {
    //         let error_logs = extract_errors(text_that_was_read.as_str());
            
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(reason_write_failed) => {
    //                 println!("Writing of erros.txt failed: {}", reason_write_failed)
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read file: {}", why_this_failed)
    //     }
    // }
}