use once_cell::sync::Lazy;
use rand::{Rng, rng};
use std::collections::HashMap;
use std::io::{self, Write};

static INTERVAL_MAP: Lazy<HashMap<(i32, i32), &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert((0, 0), "1");

    map.insert((1, 1), "b2");
    map.insert((1, 2), "2");
    map.insert((1, 3), "#2");

    map.insert((2, 3), "b3");
    map.insert((2, 4), "3");
    map.insert((2, 5), "#3");

    map.insert((3, 5), "4");
    map.insert((3, 6), "#4");

    map.insert((4, 6), "b5");
    map.insert((4, 7), "5");
    map.insert((4, 8), "#5");

    map.insert((5, 8), "b6");
    map.insert((5, 9), "6");
    map.insert((5, 10), "#6");

    map.insert((6, 10), "b7");
    map.insert((6, 11), "7");

    map
});

// Static array of all major scales from Gb to F#
static MAJOR_SCALES: &[&[&str]] = &[
    &["Gb", "Ab", "Bb", "Cb", "Db", "Eb", "F"], //Gb major
    &["Db", "Eb", "F", "Gb", "Ab", "Bb", "C"],  //Db major
    &["Ab", "Bb", "C", "Db", "Eb", "F", "G"],   //Ab major
    &["Eb", "F", "G", "Ab", "Bb", "C", "D"],    //Eb major
    &["Bb", "C", "D", "Eb", "F", "G", "A"],     //Bb major
    &["F", "G", "A", "Bb", "C", "D", "E"],      //F major
    &["C", "D", "E", "F", "G", "A", "B"],       //C major
    &["G", "A", "B", "C", "D", "E", "F#"],      //G major
    &["D", "E", "F#", "G", "A", "B", "C#"],     //D major
    &["A", "B", "C#", "D", "E", "F#", "G#"],    //A major
    &["E", "F#", "G#", "A", "B", "C#", "D#"],   //E major
    &["B", "C#", "D#", "E", "F#", "G#", "A#"],  //B major
    &["F#", "G#", "A#", "B", "C#", "D#", "E#"], //F# major
];

static IONIAN_INTERVALS: &[u32] = &[0, 2, 4, 5, 7, 9, 11];

fn get_random_scale() -> &'static [&'static str] {
    let mut rng = rng();
    let random_index = rng.random_range(0..=12);

    MAJOR_SCALES[random_index]
}

fn get_interval() -> (&'static str, &'static str, &'static str) {
    let mut rng = rng();

    // Genet a random major scale
    let major_scale = get_random_scale();
    println!("Your scale: {} major", major_scale[0]);

    // Random index of a degree of the major scale
    let root_idx = rng.random_range(0..=6);

    // Select a different note to compare with
    let targ_idx = (root_idx + rng.random_range(1..=6)) % 7;

    // Extract the notes as text
    let root_note = major_scale[root_idx];
    let targ_note = major_scale[targ_idx];

    // Get distance between notes in semitones
    let interval_dist =
        (IONIAN_INTERVALS[targ_idx] as i32 - IONIAN_INTERVALS[root_idx] as i32).rem_euclid(12);

    // Get the distance between the scale degrees
    let note_dist = (targ_idx as i32 - root_idx as i32).rem_euclid(7);

    // Extract the interval as text
    let interval = INTERVAL_MAP.get(&(note_dist, interval_dist)).unwrap();

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

    loop {
        println!("\n===================");

        // Get two notes and interval between them to make a question
        let (root, target, interval) = get_interval();

        // Print the question
        println!("Your root : {}", root);
        println!("Interval  : {}", interval);
        println!("What is the note?");

        // Get user input
        let trimmed_input = get_input(&mut input)?;
        if trimmed_input == "exit" {
            break;
        }

        // Wait for he
        if trimmed_input.to_lowercase() == target.to_lowercase() {
            println!("Correct answer!")
        } else {
            println!("Wrong. Correct answer is {}", target);
        }
        println!("===================\n");

        // Give the user some time to evaluate the correct answer
        wait_for_enter()?;
    }

    println!("Thanks for playing!\n");
    Ok(())
}
