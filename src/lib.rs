use rand::{Rng, rng};
use std::io::{self, Write};

static C_MAJOR: &[&str] = &["C", "D", "E", "F", "G", "A", "B"];
static IONIAN_INTERVALS: &[u32] = &[0, 2, 4, 5, 7, 9, 11];

fn get_interval<R: Rng + ?Sized>(rng: &mut R) -> (&'static str, &'static str, i32) {
    let root_idx = rng.random_range(0..=6);
    let targ_idx = rng.random_range(0..=6);

    let root_note = C_MAJOR[root_idx];
    let targ_note = C_MAJOR[targ_idx];

    let interval_dist =
        (IONIAN_INTERVALS[targ_idx] as i32 - IONIAN_INTERVALS[root_idx] as i32).rem_euclid(12);

    (root_note, targ_note, interval_dist)
}

fn get_input(input: &mut String) -> io::Result<&str> {
    input.clear();
    print!("Enter your input: ");
    io::stdout().flush()?;
    io::stdin().read_line(input)?;
    Ok(input.trim_end())
}

pub fn game_loop() -> io::Result<()> {
    let mut input = String::new();
    let mut rng = rng();

    loop {
        let (root, targ, interval) = get_interval(&mut rng);
        println!("{} - {} - {}", root, targ, interval);

        match get_input(&mut input)? {
            "exit" => {
                println!("Thanks for playing!");
                break;
            }
            trimmed => {
                println!("Your input: \"{}\"\n", trimmed);
            }
        }
    }

    Ok(())
}
