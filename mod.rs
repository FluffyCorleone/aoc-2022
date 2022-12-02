use super::utils::{ get_data_by_line, eval_player_shape, eval_outcome, eval_outcome2 };

const real_file:&str = "resources/day2.txt" ;
const test_file:&str = "resources/day2_test.txt" ;

pub fn part_one() -> u32 {

    let game_rounds = get_data_by_line( real_file ) ;    

    let score = game_rounds.unwrap()
        .iter()
        .map( |round| {
            let mut split = round.split_whitespace() ;

            let enemy = split.next().unwrap() ;
            let player = split.next().unwrap() ;

            let player_shape_eval = eval_player_shape( player ) ;            
            let outcome = eval_outcome( enemy, player ) ;            

            player_shape_eval + outcome
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>() ;

    score
}

pub fn part_two() -> u32 {

    let game_rounds = get_data_by_line( real_file ) ;    

    let score = game_rounds.unwrap()
        .iter()
        .map( |round| {
            let mut split = round.split_whitespace() ;

            let enemy = split.next().unwrap() ;
            let desired_outcome = split.next().unwrap() ;
            
            eval_outcome2( enemy, desired_outcome )
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>() ;

    score
}