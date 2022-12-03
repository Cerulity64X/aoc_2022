/*
 * --- Day 1: Calorie Counting ---
 * Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas.
 * For that, their favorite snack is a special type of star fruit that only grows deep in the jungle.
 * The Elves have brought you on their annual expedition to the grove where the fruit grows.
 * 
 * To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th.
 * Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
 * 
 * Collect stars by solving puzzles.
 * Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.
 * Each puzzle grants one star. Good luck!
 * 
 * The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
 * the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies.
 * One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
 * 
 * The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc.
 * that they've brought with them, one item per line.
 * Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
 */

pub fn calories() {
    let inventories: Vec<String> =
    include_str!("assets/input_day1.txt")
        .split("\n\n")
        .map(|s|s.trim().to_owned()) // convert to owned String; easier to manage
        .collect();

    let mut inventories_split: Vec<Vec<usize>> = vec![];
    for i in inventories {
        let st = i.split("\n")
            .map(|s|s.parse::<usize>().unwrap()).collect(); // parse to int
        inventories_split.push(st);
    }

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
