use std::collections::HashMap;
use std::ops::Not;
use std::io;
use std::cmp;
use rand::Rng;

const DIRECTIONS:[char; 4] = ['U', 'D', 'L', 'R'];

pub fn header() -> () {
  println!(r" Welcome to battleships!");
}

pub fn create_grid() -> HashMap<String, String> {
  let letters: Vec<char> = (b'A'..=b'H').map(|c| c as char).collect();
  let mut grid = HashMap::new();

  for letter in letters {
    for number in 1..=8 {
      let label: String = format!("{}{}", letter, number);
      grid.insert(label, " ".to_string());
    }
  }
  grid
}

pub fn print_grid(player_grid: &HashMap<String, String>, cpu_actual_grid: &HashMap<String, String>) -> () { 
  let mut cpu_grid = cpu_actual_grid.clone();
  for (key, value) in cpu_actual_grid.iter() {
    if *value == String::from("B") { 
      cpu_grid.insert(String::from(key), String::from(" "));
    }
  }
  println!(" 
  PLAYER                                CPU
 __ __ __ __ __ __ __ __              __ __ __ __ __ __ __ __
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 1          |__|__|__|__|__|__|__|__| 1
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 2          |__|__|__|__|__|__|__|__| 2
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 3          |__|__|__|__|__|__|__|__| 3
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 4          |__|__|__|__|__|__|__|__| 4
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 5          |__|__|__|__|__|__|__|__| 5
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 6          |__|__|__|__|__|__|__|__| 6
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 7          |__|__|__|__|__|__|__|__| 7
|{} |{} |{} |{} |{} |{} |{} |{} |            |{} |{} |{} |{} |{} |{} |{} |{} |
|__|__|__|__|__|__|__|__| 8          |__|__|__|__|__|__|__|__| 8
 A  B  C  D  E  F  G  H                A  B  C  D  E  F  G  H
", 
player_grid["A1"], player_grid["B1"], player_grid["C1"], player_grid["D1"], player_grid["E1"], player_grid["F1"], player_grid["G1"], player_grid["H1"],
cpu_grid["A1"], cpu_grid["B1"], cpu_grid["C1"], cpu_grid["D1"], cpu_grid["E1"], cpu_grid["F1"], cpu_grid["G1"], cpu_grid["H1"],
player_grid["A2"], player_grid["B2"], player_grid["C2"], player_grid["D2"], player_grid["E2"], player_grid["F2"], player_grid["G2"], player_grid["H2"],
cpu_grid["A2"], cpu_grid["B2"], cpu_grid["C2"], cpu_grid["D2"], cpu_grid["E2"], cpu_grid["F2"], cpu_grid["G2"], cpu_grid["H2"],
player_grid["A3"], player_grid["B3"], player_grid["C3"], player_grid["D3"], player_grid["E3"], player_grid["F3"], player_grid["G3"], player_grid["H3"],
cpu_grid["A3"], cpu_grid["B3"], cpu_grid["C3"], cpu_grid["D3"], cpu_grid["E3"], cpu_grid["F3"], cpu_grid["G3"], cpu_grid["H3"],
player_grid["A4"], player_grid["B4"], player_grid["C4"], player_grid["D4"], player_grid["E4"], player_grid["F4"], player_grid["G4"], player_grid["H4"],
cpu_grid["A4"], cpu_grid["B4"], cpu_grid["C4"], cpu_grid["D4"], cpu_grid["E4"], cpu_grid["F4"], cpu_grid["G4"], cpu_grid["H4"],
player_grid["A5"], player_grid["B5"], player_grid["C5"], player_grid["D5"], player_grid["E5"], player_grid["F5"], player_grid["G5"], player_grid["H5"],
cpu_grid["A5"], cpu_grid["B5"], cpu_grid["C5"], cpu_grid["D5"], cpu_grid["E5"], cpu_grid["F5"], cpu_grid["G5"], cpu_grid["H5"],
player_grid["A6"], player_grid["B6"], player_grid["C6"], player_grid["D6"], player_grid["E6"], player_grid["F6"], player_grid["G6"], player_grid["H6"],
cpu_grid["A6"], cpu_grid["B6"], cpu_grid["C6"], cpu_grid["D6"], cpu_grid["E6"], cpu_grid["F6"], cpu_grid["G6"], cpu_grid["H6"],
player_grid["A7"], player_grid["B7"], player_grid["C7"], player_grid["D7"], player_grid["E7"], player_grid["F7"], player_grid["G7"], player_grid["H7"],
cpu_grid["A7"], cpu_grid["B7"], cpu_grid["C7"], cpu_grid["D7"], cpu_grid["E7"], cpu_grid["F7"], cpu_grid["G7"], cpu_grid["H7"],
player_grid["A8"], player_grid["B8"], player_grid["C8"], player_grid["D8"], player_grid["E8"], player_grid["F8"], player_grid["G8"], player_grid["H8"],
cpu_grid["A8"], cpu_grid["B8"], cpu_grid["C8"], cpu_grid["D8"], cpu_grid["E8"], cpu_grid["F8"], cpu_grid["G8"], cpu_grid["H8"]
);
}

pub fn grid_check(input: &str) -> bool {
  let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
  let numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
  
  for (i, c) in input.chars().enumerate() {
    match i {
      0 => {
        if letters.iter().any(|&i| i==c).not() {
          println!("NO {}", c);
          return false
        }
      },
      1 => {
        if numbers.iter().any(|&i| i==c).not() {
          println!("NO {}", c);
          return false
        }
      },
      _ => {
        return false
      },
    }
  }
  return true
}

pub fn read_input(prompt: String) -> String {
  loop {
    println!("Please input a valid cell {}", prompt);
    let mut input: String = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("This is not a value");
    let coord: String = input.trim().parse().expect("Not a value.");
    if grid_check(&coord) {
      return coord
    }
  }
}

fn check_boat(start: &str, end: &str, boat: u8, grid: &mut HashMap<String, String>) -> bool {
  // Finds the distance between 2 points to check size. 
  let mut boat_locations: Vec<String> = Vec::new();
  let mut dist: u8 = 0;

  for n in 0..2 {
    let single_dist: u8 = (end.chars().nth(n).unwrap() as i8 - start.chars().nth(n).unwrap() as i8).abs().try_into().unwrap(); 
    dist = dist + single_dist;
  }

  if dist + 1 == boat {  
    let start_col: char = start.chars().nth(0).unwrap();
    let end_col: char = end.chars().nth(0).unwrap();
    let start_row: u8 = start.chars().nth(1).unwrap() as u8;
    let end_row: u8 = end.chars().nth(1).unwrap() as u8;

    if start_col == end_col {
      // Loop through the row in the same column. 
      for row in cmp::min(start_row, end_row)..=cmp::max(start_row, end_row) {
        let grid_ref = String::from(format!("{}{}", start_col, row-48));
        let value = grid.get(&grid_ref).clone().unwrap();
        if *value != String::from(" ") {
          println!("Something is here");
          return false
        }
        boat_locations.push(grid_ref);
      }
    } else if start_row == end_row {
        // println!("{} {}", start_col, end_col);
        for col in cmp::min(start_col, end_col)..=cmp::max(start_col, end_col) {
          let grid_ref = String::from(format!("{}{}", col, start_row-48));
          let value = grid.get(&grid_ref).clone().unwrap();
          if *value != String::from(" ") {
            println!("Something is here");
            return false
          }
          boat_locations.push(grid_ref);
        }
    } 
  } else {
    println!("Incorrect size");
    return false
  }
  fill_boat(&boat_locations, grid);
  return true
}

fn fill_boat(locations: &Vec<String>, grid: &mut HashMap<String, String>) -> () {
  let boat_marker: String = String::from("B");
  for location in locations {
    grid.insert(location.to_string(), boat_marker.clone());
  }

}

pub fn place_boats(mut grid: HashMap<String, String>, user: bool) -> HashMap<String, String> {
  let boat_sizes : [u8; 4] = [2, 2, 3, 4];
  if user {
    for boat in boat_sizes {
      loop {
        let prompt: String = format!("to {} a boat of length {}", "start", boat);
        let start_coord = read_input(prompt.to_string());
      
        let prompt: String = format!("to {} a boat of length {}", "finish", boat);
        let end_coord = read_input(prompt.to_string());
      
        if (start_coord.chars().nth(0).unwrap() == end_coord.chars().nth(0).unwrap()) | (start_coord.chars().nth(1).unwrap() == end_coord.chars().nth(1).unwrap()){
          if check_boat(&start_coord, &end_coord, boat, &mut grid) {
            break;
          }
        } else {
          panic!("Attempted diagonal boat. Cannot be done.");
        }
      }
    }
  } else {
    for boat in boat_sizes {
      loop{
        let (start_col, start_row) = random_location();
        let start_location = format!(
          "{}{}", 
          start_col, 
          start_row,
        );
        match DIRECTIONS[rand::thread_rng().gen_range(0..DIRECTIONS.len())] {
          'L' => {
                let end_col = start_col as u8 - (boat-1);
                if end_col > 64 {
                  if check_boat(&start_location, &format!("{}{}", end_col as char, start_row), boat, &mut grid) {
                    break
                  }
                }
              },
          'R' => {
            let end_col = start_col as u8 + (boat-1);
            if end_col < 73 {
              if check_boat(&start_location, &format!("{}{}", end_col as char, start_row), boat, &mut grid) {
                break
              }
            }
          },
          'U' => {
            let end_row: i8 = (start_row as i8 - (boat as i8 -1)).try_into().unwrap();
            if end_row > 0 {
              if check_boat(&start_location, &format!("{}{}", start_col, end_row), boat, &mut grid) {
                break
              }
            }
          },
          'D' => {
            let end_row = start_row as u8 + (boat-1);
            if end_row < 9 {
              if check_boat(&start_location, &format!("{}{}", start_col, end_row), boat, &mut grid) {
                break
              }
            }
          },
          _ => panic!("How did I get here!?"),
        }
      }
    }
        
  }
  return grid
}

pub fn random_location() -> (char, i8) {
  let start_col = rand::thread_rng().gen_range('A'..='E');
  let start_row = rand::thread_rng().gen_range(1..=8);
  (start_col, start_row)
}


pub fn make_guess(guess: String, grid: &mut HashMap<String, String>) -> String {
  let value = grid.get(&guess).clone().unwrap();
  if (*value == String::from("B")) | (*value == String::from("X")) {
    grid.insert(guess, String::from("X"));
    return String::from("X")
  } else {
    grid.insert(guess, String::from("O"));
    return String::from("O")
  }
}

pub fn check_win(grid: &HashMap<String, String>) -> bool {
  return grid.values().any(|x| *x == String::from("B")).not()
}

fn cpu_guess_check(previous_guesses:  &mut HashMap<String, String>) -> (char, i8) {
  loop {
    let (guess_col, guess_row) = random_location();
    let location = format!("{}{}", guess_col, guess_row);
    if !previous_guesses.contains_key(&location) {
      return (guess_col, guess_row)
    }
  }
}


pub fn cpu_guess(grid: &mut HashMap<String, String>, previous_guesses: &mut HashMap<String, String>) -> () {

  let (guess_col, guess_row) = cpu_guess_check(previous_guesses);
  let result = make_guess(format!("{}{}", guess_col, guess_row), grid);
  previous_guesses.insert(format!("{}{}", guess_col, guess_row).to_string(), result);

}