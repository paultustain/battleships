use battleships;
use std::ops::Not;
use std::collections::HashMap;


fn main() {
    battleships::header();
    let mut player_grid = battleships::create_grid();
    let mut cpu_grid = battleships::create_grid();
    let mut previous_guesses: HashMap<String, String> = HashMap::new();

    player_grid = battleships::place_boats(player_grid, true); 
    cpu_grid = battleships::place_boats(cpu_grid, false);

    battleships::print_grid(&player_grid, &cpu_grid);
    let mut user = true; 
    loop {
        if user {
            let guess = battleships::read_input("Please enter your guess: ".to_string());
            battleships::make_guess(guess, &mut cpu_grid);
            if battleships::check_win(&cpu_grid) {
                println!("You win");
                break
            } 
        } else {
            battleships::cpu_guess(&mut player_grid, &mut previous_guesses);
            if battleships::check_win(&player_grid) {
                println!("You lose");
                break
            }
        }
        battleships::print_grid(&player_grid, &cpu_grid);
        user = user.not()
    }
    battleships::print_grid(&player_grid, &cpu_grid);
}

