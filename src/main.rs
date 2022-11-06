use std::env::args;
use chrono;
use csv;
use std::process;
use std::env;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct Record{
    date: String,
    person: String 
}

fn main() -> std::io::Result<()>{

    let input = args().collect::<Vec<String>>();



    //All paths are paths to hell.

    let mut path = env::current_exe()?.display().to_string();
    path = (path[0..&path.len()-10]).to_string();
    path.push_str("\\bdays.csv");
    
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(Path::new(&path))
        .unwrap();

    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    


    let mut today = chrono::offset::Local::now().to_string();
    today = (&today[5..11]).to_string();


    if input.len() < 2{

        println!("Birthdays this month: ");
        for record in reader.deserialize(){
              let record: Record = record?;
                if &record.date[0..2] == &today[0..2]{
                  println!("{} on {}",record.person, record.date);
            }
        }
            
        process::exit(0);
    }  


    for argument in input.iter().skip(1){


        match argument.as_str(){
            "-all" => { 
                println!("All birthdays registered: ");
                
                for record in reader.deserialize(){
                    let record: Record = record?;
                    println!("{} on {}", record.person, record.date);
                }  
            }
            "-nm" => {
                process::exit(0);
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
            "help" =>{
                print_help();
            }
            "-h" => {
                print_help();
            }
            "-help" =>{
                print_help();
            }
            "-version"=>{
                print_version();
            }
            "version" =>{
                print_version();
            }
            "-v" =>{
                print_version();
            }
            &_ => {
               print_help(); 
            }
        }
    }

    Ok(())

}

fn print_help(){
    println!("Thank you for using bdays, you can use the following commands:");
    println!("");
    println!("-h | help | -help");
    println!("These commands will open this help menu.");
    println!("");
    println!("-version version -v");
    println!("These commands will tell you the current version running.");
    println!("");
    println!("new");
    println!("This comman will open an assistant to enter new birthdays.");
    println!("You can always just edit the csv directly, found on the folder of the project.");
    println!("");
    println!("-all");
    println!("This command will show all the birthdays registered");
    println!("");
    println!("No commands");
    println!("Bdays will show you all the birthdays this month.");

    
}

fn print_version(){
    println!("version 1.0");
}
