use csv;
use std::error::Error;

fn main() {
    if let Err(err) = read_csv("./customers.csv") {
        println!("error running example: {}", err);
        ::std::process::exit(1);
    }
}

fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}