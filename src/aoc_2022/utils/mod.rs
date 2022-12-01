use std::fs::File ;
use std::io::{prelude::*, BufReader } ;

pub fn max_nth( input:Vec<u32>, n:usize ) -> Vec<u32> {

    let mut ret = input ;

    ret.sort() ;
    ret.reverse() ;

    ret[0..n].to_vec()    
}

pub fn get_data_by_line( file_path:&str ) -> std::io::Result<Vec<String>> {

    let file = File::open( file_path )? ;    
    let buffer = BufReader::new( file ) ;

    let mut lines = Vec::new() ;

    for line in buffer.lines() {
        lines.push( line? ) ;
    }

    Ok(lines)
}

pub fn get_data_to_string( file_path:&str ) -> std::io::Result<String> {

    let mut file = File::open( file_path )? ;    
    let mut buffer = String::new() ;

    file.read_to_string( &mut buffer )? ;        

    Ok(buffer)
}