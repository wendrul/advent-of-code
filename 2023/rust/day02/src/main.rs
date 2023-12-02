use std::{error::Error, collections::HashMap, fs};

fn main() -> Result<(), Box<dyn Error>>{
    let part1_input = fs::read_to_string("part1_input.txt").expect("The puzzle input exists.");
    let result = get_possible_games(&part1_input)?;
     
    println!("{:?}\nThe sum is {}\n\n", result, result.iter().sum::<usize>());

    let result_part2 = get_game_minimum_cubeset_powers(&part1_input)?;
    println!("{:?}\nThe sum is {}\n\n", result_part2, result_part2.iter().sum::<usize>());
    Ok(())
}

fn get_possible_games(input: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let amounts = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut ret: Vec<usize> = Vec::new();


    for line in input.lines() {
        let mut line = line.split(':');
        let id = &line.next().unwrap()[5..].parse::<usize>()?;
        ret.push(id.clone());
        for cube_set in line.next().unwrap().split(';') {
            for color_cubes in cube_set.split(',') {
                let mut color_cubes = color_cubes.split(' ');
                color_cubes.next();
                let num: usize = color_cubes.next().unwrap().parse::<usize>()?;
                let color = color_cubes.next().unwrap();
                match amounts.get(color) {
                    Some(n) if n >= &num => (),
                    Some(_) => {ret.pop(); break;},
                    None => return Err("not a color".into()),
                }
                // println!("Game {}: There are {} of {} cubes", id, num, color);
            }
            if !ret.contains(id) {
                break;
            }

        }


    } 
    Ok(ret)
}

fn get_game_minimum_cubeset_powers(input: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut ret: Vec<usize> = Vec::new();


    for line in input.lines() {
        let mut line = line.split(':');
        let _id = &line.next().unwrap()[5..].parse::<usize>()?;
        let mut amounts = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        for cube_set in line.next().unwrap().split(';') {
            for color_cubes in cube_set.split(',') {
                let mut color_cubes = color_cubes.split(' ');
                color_cubes.next();
                let num: usize = color_cubes.next().unwrap().parse::<usize>()?;
                let color = color_cubes.next().unwrap();
                match amounts.get(color) {
                    Some(n) if n < &num => {amounts.insert(color, num);},
                    Some(_) => (),
                    None => return Err("not a color".into()),
                }
            }
        }
        println!("Game {} needs {:?}", _id, amounts);
        ret.push(amounts["red"] * amounts["green"] * amounts["blue"]);


    } 
    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
