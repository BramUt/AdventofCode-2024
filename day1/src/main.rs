use std::fs;
use std::env;
use std::iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let file_content = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error)
    };

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in file_content.lines(){
        // let (num1, num2) = line.split_once("   ").map(|(n1, n2)|(n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())).unwrap();
        let (s1, s2) = line.split_once("   ").unwrap();
        let num1 = s1.parse::<i32>().unwrap();
        let num2 = s2.parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    // println!("Unsorted: {list1:?}");

    list1.sort();
    list2.sort();

    // println!("  Sorted: {list1:?}");

    // Part 1
    let mut total_distance: u32 = 0;
    for (num1, num2) in iter::zip(list1.clone(), list2.clone()){
        let difference = num1.abs_diff(num2);
        // println!("Difference: {difference} ({num1}, {num2})");
        total_distance += u32::try_from(difference).unwrap();
    }

    println!("Total distance: {total_distance}");

    // Part 2
    let mut similarity: u32 = 0;
    for num1 in list1 {
        let count= list2.iter().filter(|&&i| i.eq(&num1)).count();
        let sim_increase = num1 as u32 * count as u32;
        // println!("Number {num1} found {count} times. Similarity increase: {sim_increase}");
        similarity += sim_increase;
    }

    println!("Total similarity: {similarity}")


}
