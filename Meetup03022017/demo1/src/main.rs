#![allow(dead_code)]
#![allow(unused)]

extern crate rand;

use rand::{thread_rng, Rng};

#[derive(Debug)]
#[derive(PartialEq)]
enum Personality {
    Tyhma,
    Fiksu,
}

#[derive(Debug)]
struct Person {
    name: String,
    personality: Personality,
}

fn print(message: String) {
    println!("\n{}\n", message);
}

fn inc_by_1(number: i32) -> i32 {
    number + 1 // last expression returns
}

fn evil_finder() -> Option<i32> {
    if thread_rng().choose(&[0, 1]) == Some(&0) {
        Some(666)
    } else {
        None
    }
}

fn count_pairs_in_a_string(text: String) -> usize {
    text.chars().zip(text.chars().skip(1)).filter(|&(x, y)| x == y).count()
}

fn main() {
    let person1 = Person {
        name: "Trump".to_string(),
        personality: Personality::Tyhma,
    }; //This person is real stupid

    // person1 = Person{ name: "Trump".to_string(), personality: Personality::Tyhma }; //error[E0384]: re-assignment of immutable variable `person1`

    if person1.personality == Personality::Tyhma {
        print(person1.name + " is history's biggest mistake!");
    }

    print(format!("1 + 1 => {}", inc_by_1(1)));

    // Arrays
    let array: [i32; 3] = [1, 2, 3]; //[0; 3]

    for item in array.iter() {
        print(item.to_string());
    }

    // Vectors
    let vectx = vec![1, 2, 3];
    println!("{:?}", vectx);

    let vecty: Vec<i32> = (1..10).collect();
    println!("{:?}", vecty);


    // Functional programming aspects

    // Pattern matching

    match person1.personality {
        Personality::Tyhma => println!("USA is doomed!"),
        _ => println!("USA still doomed!"),
    }

    match evil_finder() {
        Some(666) => println!("You are evil!"),
        Some(_) | None => print!("You are a saint!"),
    }

    // closures
    let paired_string = "sshhiittt!!";
    let pairs_count = count_pairs_in_a_string(paired_string.to_string());
    println!("\nTotal count: {:?}\n", pairs_count);
}

//Tests

#[test]
fn test_paired_string() {
    let text = "sshhiittt!!";
    assert_eq!(6, count_pairs_in_a_string(text.to_string()));
}

#[test]
fn test_paired_looong_string() {
    let text = "fuuuuuuuuuuuuuccccccckkkkkkkkk!!";
    assert_eq!(6, count_pairs_in_a_string(text.to_string()));
}

