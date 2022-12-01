use super::utils::{ get_data_by_line, max_nth };

const file_path:&str = "resources/day1.txt" ;
type lines = Vec<String> ;

pub fn part_one() -> u32 {
    
    let calories = get_data_by_line( file_path ).unwrap() ;        

    let grouped = calories.split( |x| x.is_empty() ) ;

    let sums = grouped.into_iter().map( |x| {
        x.iter().fold( 0, |accum, item| accum + item.parse::<u32>().unwrap() )
    })
    .collect::<Vec<u32>>() ;    
    
    *sums.iter().max().unwrap()    
}

pub fn part_two() -> u32 {
    
    let calories = get_data_by_line( file_path ).unwrap() ;        

    let grouped = calories.split( |x| x.is_empty() ) ;

    let sums = grouped.into_iter().map( |x| {
        x.iter().fold( 0, |accum, item| accum + item.parse::<u32>().unwrap() )
    })
    .collect::<Vec<u32>>() ;    

    max_nth( sums, 3 )
    .iter().sum::<u32>()    
}