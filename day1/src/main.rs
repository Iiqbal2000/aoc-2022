use std::{io::{self, Read}};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut calories:Vec<isize> = Vec::new();

    let mut calorie_per_elf: Vec<isize> = Vec::new();

    for str_val in input.split('\n') {
        if str_val.is_empty() {
            let total_current_calorie: isize = calorie_per_elf
                .iter_mut()
                .fold(0, |acc, x| acc + *x);
            calories.push(total_current_calorie);
            calorie_per_elf.clear();
            continue;
        }

        let i_calorie = str_val
            .parse::<isize>()
            .expect("cannot convert the callory to integer");
        calorie_per_elf.push(i_calorie);
    }

    calories.sort();

    let top_three_elves = &calories[calories.len()-3..].iter().sum::<isize>();

    let total_calories = calories
        .iter()
        .max()
        .expect("cannot get the greatest calorie value");

    println!("total Calories: {total_calories:?}");
    println!("total top three: {top_three_elves:?}");
}