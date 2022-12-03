/*
 * Challenge:
 * Inventories are lists of numbers, seperated by newlines. Inventories are seperated by double newlines.
 * Output the highest calorie count, and the 3 highest calorie counts, summed.
 */

pub fn calories() {
    // Seperate inventories
    let inventories: Vec<String> =
    include_str!("assets/input_day1.txt")
        .split("\n\n")
        .map(|s|s.trim().to_owned()) // convert to owned String; easier to manage
        .collect();

    // Split up inventories into seperate entries for each calorie count
    let mut inventories_split: Vec<Vec<usize>> = vec![];
    for i in inventories {
        let st = i.split("\n")
            .map(|s|s.parse::<usize>().unwrap()).collect(); // parse to int
        inventories_split.push(st);
    }

    // Sum each inventory
    let mut inventories_summed: Vec<usize> = vec![];
    for i in inventories_split {
        let mut total = 0;
        for n in i {
            total += n;
        }
        inventories_summed.push(total);
    }
    
    inventories_summed.sort();
    inventories_summed.reverse();
    println!("Biggest calorie count: {}", inventories_summed[0]);
    println!("Top 3 calorie count: {}", inventories_summed[0] + inventories_summed[1] + inventories_summed[2]);
}
