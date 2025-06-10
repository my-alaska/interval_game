use std::io::{self, Write};
use rand::{self,Rng};

static C_MAJOR : &[&str] = &["C", "D", "E", "F", "G", "A", "B"];
static IONIAN_INTERVALS  : &[u32] = &[0,2,4,5,7,9,11];

fn get_interval<'a, R: Rng + ?Sized>(rng: &mut R) -> (&'a str, &'a str, i32){
    let root_idx: usize = rng.random_range(0..=6);
    let targ_idx: usize = rng.random_range(0..=6);
    
    let root_note = &C_MAJOR.get(root_idx).unwrap();
    let targ_note = &C_MAJOR.get(targ_idx).unwrap();

//    let note_dist = (targ_idx - root_idx) % 8;
    let interval_dist: i32 = (IONIAN_INTERVALS[targ_idx]  as i32 - IONIAN_INTERVALS[root_idx]  as i32).rem_euclid(12);

    return (root_note, targ_note, interval_dist)
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
    let mut rng = rand::rng();
    loop {
        let (root, targ, interval) : (&str, &str, i32) = get_interval(&mut rng);
        
        println!("{} - {} - {}", root, targ, interval);
        
        let trimmed = get_input(&mut input); 
        if trimmed == "exit"{
            println!("Thanks for playing!\n");
            return Ok(())
        }
        
        println!("Your input: \"{}\"\n", trimmed);
        
    }
}



