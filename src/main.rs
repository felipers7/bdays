use std::env::args;
use chrono;
use csv;
use std::process;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct Record{
    date: String,
    person: String 
}

fn main() -> std::io::Result<()>{

    let input = args().collect::<Vec<String>>();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("bdays.csv")
        .unwrap();

    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    

/*
    for record in reader.deserialize(){
        let record: Record = record?;
        println!("{} - {}",record.date, record.person);

    }
*/

    let mut today = chrono::offset::Local::now().to_string();
    today = (&today[5..11]).to_string();


    if input.len() < 2{
        println!("yoy");
        process::exit(0);
    }  


    for argument in input.iter().skip(1){


        match argument.as_str(){
            "-m" =>{

                for record in reader.deserialize(){
                    let record: Record = record?;

                    if &record.date[0..2] == &today[0..2]{
                        println!("{} on {}",record.person, record.date);
                    }
                }
            }
            "-w" => { 

            }
            "-nm" => {

            }
            "new" => {
                println!("Type the name of the person:");
                let mut name_buffer = String::new();
                io::stdin().read_line(&mut name_buffer)?;

                println!("Type birthday's month (00 format):");
                let mut month_buffer = String::new();
                io::stdin().read_line(&mut month_buffer)?;

                println!("Type birthday's day (00 format):");
                let mut day_buffer = String::new();
                io::stdin().read_line(&mut day_buffer)?;
                
                day_buffer = String::from(day_buffer.trim());
                month_buffer = String::from(month_buffer.trim());
                name_buffer = String::from(name_buffer.trim());

                if let Err(e) = writeln!(file, "{}-{},{}",month_buffer, day_buffer, name_buffer){
                    eprintln!("Couldn't write in file {}", e);
                }
                else{
                    println!("Register was successfully added.");
                }


            }
            &_ => {
                println!("{argument} is not recognized as a command for bdays");
                
            }
        }
    }

    Ok(())

}
