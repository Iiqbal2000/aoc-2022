use std::fs;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Operation {
    Double,         // old + old
    Add(i32),       // old + number
    Square,         // old * old
    Multiply(i32),  // old * number
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: Operation,
    divisible: i32,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn calculate(&self, old: i32) -> i32 {
        match self.operation {
            Operation::Double => old * 2,
            Operation::Add(number) => old + number,
            Operation::Square => old * old,
            Operation::Multiply(number) => old * number,
        }
    }
}

fn parse_operation(operation: &str) -> Operation {
    let parts: Vec<&str> = operation.split_whitespace().collect();

    match (parts[1], parts[2]) {
        ("*", "old") => Operation::Square,
        ("+", "old") => Operation::Double,
        (_, num_str) => {
            let number = num_str.parse::<i32>().expect("invalid operand 2!");
            match parts[1] {
                "*" => Operation::Multiply(number),
                "+" => Operation::Add(number),
                _ => panic!("invalid operation syntax!"),
            }
        }
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];

    for line in fs::read_to_string("/Users/Iqbal.alasqalani/Dev/aoc-2022/day11/input.txt")
        .expect("can't read file").lines() {
        let line = line.trim();

        if line.starts_with("Monkey") {
            let monkey_id = line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
            monkeys.push(Monkey {
                id: monkey_id,
                items: Vec::new(),
                operation: Operation::Double,  // Default operation
                divisible: 0,
                if_true: 0,
                if_false: 0,
                inspect_count: 0,
            });
        } else if line.starts_with("Starting items") {
            let items = line.split(": ").nth(1).unwrap().split(", ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            monkeys.last_mut().unwrap().items = items;

        } else if line.starts_with("Operation") {
            let operation_str = line.split(": ").nth(1).unwrap().split('=').nth(1).unwrap().trim();
            let operation = parse_operation(operation_str);
            monkeys.last_mut().unwrap().operation = operation;

        } else if line.starts_with("Test") {
            let divisible = line.split_whitespace().last().unwrap().parse().unwrap();
            monkeys.last_mut().unwrap().divisible = divisible;

        } else if line.starts_with("If true") {
            let if_true = line.split_whitespace().last().unwrap().parse().unwrap();
            monkeys.last_mut().unwrap().if_true = if_true;
            
        } else if line.starts_with("If false") {
            let if_false = line.split_whitespace().last().unwrap().parse().unwrap();
            monkeys.last_mut().unwrap().if_false = if_false;
        }
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let if_true = monkeys[i].if_true;
            let if_false = monkeys[i].if_false;

            for _ in 0..monkeys[i].items.len() {
                if monkeys[i].items.is_empty() {
                    break;
                }
                let current_item = monkeys[i].items.remove(0);
                let worry_level_result = monkeys[i].calculate(current_item) / 3;
                
                if (worry_level_result % monkeys[i].divisible) == 0 {
                    if let Some(monkey) = monkeys.iter_mut().find(|m: &&mut Monkey| m.id == if_true) {
                        monkey.items.push(worry_level_result);
                    }
                } else {
                    if let Some(monkey) = monkeys.iter_mut().find(|m: &&mut Monkey| m.id == if_false) {
                        monkey.items.push(worry_level_result);
                    }
                }
            
                monkeys[i].inspect_count += 1;
            }
    
        }
    
        monkeys.iter().for_each(|m| {
            println!("monkey {}: {:?}", m.id, m.items);
            println!("inspect count: {}", m.inspect_count);
        });
    }


    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    println!("result: {:?}", monkeys[0].inspect_count * monkeys[1].inspect_count)
}
