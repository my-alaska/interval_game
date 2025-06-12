use rand::{Rng, rng};
use std::io::{self, Write};
use std::collections::HashMap;

static C_MAJOR: &[&str] = &["C", "D", "E", "F", "G", "A", "B"];
static IONIAN_INTERVALS: &[u32] = &[0, 2, 4, 5, 7, 9, 11];

fn build_interval_map() -> HashMap<(i32, i32), &'static str> {
    let mut interval_map = HashMap::new();

    interval_map.insert((0, 0), "1");

    interval_map.insert((1, 1), "b2");
    interval_map.insert((1, 2), "2");
    interval_map.insert((1, 3), "#2");

    interval_map.insert((2, 3), "b3");
    interval_map.insert((2, 4), "3");
    interval_map.insert((2, 5), "#3");

    interval_map.insert((3, 5), "4");
    interval_map.insert((3, 6), "#4");

    interval_map.insert((4, 6), "b5");
    interval_map.insert((4, 7), "5");
    interval_map.insert((4, 8), "#5");

    interval_map.insert((5, 8), "b6");
    interval_map.insert((5, 9), "6");
    interval_map.insert((5, 10), "#6");

    interval_map.insert((6, 10), "b7");
    interval_map.insert((6, 11), "7");

    interval_map
}

fn get_interval<R: Rng + ?Sized>(rng: &mut R, map: &HashMap<(i32, i32), &'static str>) -> (&'static str, &'static str, &'static str) {
    let root_idx = rng.random_range(0..=6);
    let targ_idx = rng.random_range(0..=6);

    let root_note = C_MAJOR[root_idx];
    let targ_note = C_MAJOR[targ_idx];

    let interval_dist =
        (IONIAN_INTERVALS[targ_idx] as i32 - IONIAN_INTERVALS[root_idx] as i32).rem_euclid(12);
       
    let note_dist = (targ_idx as i32 - root_idx as i32).rem_euclid(7);

    let interval = map.get(&(note_dist, interval_dist)).unwrap();

    (root_note, targ_note, interval)
}

fn get_input(input: &mut String) -> io::Result<&str> {
    input.clear();
    print!("Enter your input: ");
    io::stdout().flush()?;
    io::stdin().read_line(input)?;
    Ok(input.trim_end())
}

fn wait_for_enter() -> io::Result<()> {
    let mut dummy = String::new();
    print!("Press Enter to continue...");
    io::stdout().flush()?; // Ensure the message is displayed before waiting
    io::stdin().read_line(&mut dummy)?; // Waits for user to press enter
    Ok(())
}

pub fn game_loop() -> io::Result<()> {
    let mut input = String::new();
    let mut rng = rng();
    let map = build_interval_map();

    loop {
        let (root, target, interval) = get_interval(&mut rng, &map);
        println!("\n===================");
        println!("Your root : {}", root);
        println!("Interval  : {}", interval);
        println!("What is the note?");

        let trimmed = get_input(&mut input)?; 
        if trimmed == "exit"{
            println!("Thanks for playing!\n");
            break;
        }
        
        if trimmed.to_lowercase() == target.to_lowercase(){
            println!("Correct answer!")
        } else {
            println!("Wrong. Correct answer is {}",target);
        }
        println!("===================\n");
        wait_for_enter()?;
    }

    Ok(())
}
