mod hands;
mod game;


fn main() {
    let mut computer_score = 0;
    let mut player_score = 0;

    loop {
        let computer = hands::Hand::get_random();
        let player = hands::Hand::user_choice();

        let round = game::GameRound::new(computer, player);
        let result = round.play_game();

        match result {
            game::GameOutcome::Win => player_score += 1,
            game::GameOutcome::Lose => computer_score += 1,
            game::GameOutcome::Draw => (),
        }

        println!("==========ROUND===========");
        println!("Computer played: {computer:?}");
        println!("You played: {player:?}");
        println!();
        println!("{result}");
        println!("=========SCORES===========");
        println!("Computer: {computer_score}");
        println!("You: {player_score}");
    }
}
