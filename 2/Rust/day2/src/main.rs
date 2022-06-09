use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
enum Movement{
    Up,
    Down,
    Forward,
}


impl FromStr for Movement{
    
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();
        match lowercase.as_str() {
            "up"      => Ok(Movement::Up), 
            "down"    => Ok(Movement::Down), 
            "forward" => Ok(Movement::Forward),
            _         => Err(()), 
        }
    }
}



fn part1(buf_reader: BufReader<File>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in buf_reader.lines(){
        let tmp_line = match line{
            Ok(l) => l,
            Err(_error) => panic!("problem with line"),
        };
        let line_result : Vec<&str>= tmp_line.split(" ").collect();
        // println!("{:?}", line_result);
        let line_enum: Movement = line_result[0].parse().unwrap();
        // println!("as enum {:?}", line_enum );
        let movement: u32 = line_result[1].parse().expect("Not a number"); 

        match line_enum {
            Movement::Up        => depth = depth - movement, 
            Movement::Down      => depth = depth + movement, 
            Movement::Forward   => horizontal = horizontal + movement
        };
        // println!("depth {:?}, horizontal {:?}", depth, horizontal );
    }
    
    // println!("Multiplied Result : {:?}", depth * horizontal);
    return depth * horizontal;
}


fn part2(buf_reader: BufReader<File>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in buf_reader.lines(){
        let tmp_line = match line{
            Ok(l) => l,
            Err(_error) => panic!("problem with line"),
        };
        let line_result : Vec<&str>= tmp_line.split(" ").collect();
        // println!("{:?}", line_result);
        let line_enum: Movement = line_result[0].parse().unwrap();
        // println!("as enum {:?}", line_enum );
        let movement: u32 = line_result[1].parse().expect("Not a number"); 

        match line_enum {
            Movement::Up        => aim = aim - movement, 
            Movement::Down      => aim = aim + movement, 
            Movement::Forward   => {
                                        horizontal = horizontal + movement;
                                        depth = depth + (aim * movement);
                                   }
        };
        // println!("depth {:?}, horizontal {:?} aim{:?}", depth, horizontal, aim );
    }
    
    // println!("Multiplied Result : {:?}", depth * horizontal);
    return depth * horizontal;
}



fn main() -> std::io::Result<()> {



    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    println!("Part1 answer {:?}", part1(buf_reader));

    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    println!("Part2 answer {:?}", part2(buf_reader));


    Ok(())


}