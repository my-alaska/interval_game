use std::io::{self, Write};


static STARTING_KEYS : &[&str] = &["Gb", "Db", "Ab", "Eb", "Bb", "F", "C", "G", "D", "A", "E", "B", "F#"];
static IONIAN_INTERVALS  : &[u32] = &[0,2,4,5,7,9,11];
static C_MAJOR_NOTES  : &[&str] = &["C", "D", "E", "F", "G", "A", "B"];

fn get_random_key<'a>() -> &'a str{
    "asd"
}

fn get_interval<'a>() -> (&'a str, &'a str, &'a str){
    let note = get_random_key();
    
    return ("C", "Gb", "b5")
}

fn get_input(input: &mut String) -> &str{
    input.clear();
    
    print!("Enter your input: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(input).expect("Error reading input");
    
    input.trim_end()
}

pub fn game_loop() -> Result<(), &'static str>{
    let mut input = String::new();
    loop {
        
        
        let trimmed = get_input(&mut input); 
        if trimmed == "exit"{
            println!("Thanks for playing!\n");
            return Ok(())
        }
        
        println!("Your input: \"{}\"\n", trimmed);
        
    }
}



