use rand::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::thread;

fn main() {
    let mut collection = vec![4, 5, 2, 8, 6, 1, 3, 9, 7];

    println!("{:?}", quicksort(collection));
}

fn quicksort(mut collection_to_sort: Vec<u8>) -> Vec<u8> {
    if collection_to_sort.len() < 2 {
        return collection_to_sort;
    }

    let mut rng = thread_rng();

    let mut pivot = collection_to_sort.iter().choose(&mut rng).unwrap();

    //Get the items less than pivot
    let mut less_than = collection_to_sort.iter().cloned().filter(|x| x < &pivot).collect::<Vec<_>>();

    //Get the items greater than the pivot
    let mut greater_than = collection_to_sort.iter().cloned().filter(|x| x > &pivot).collect::<Vec<_>>();

    return [quicksort(less_than), vec![*pivot], quicksort(greater_than)].concat();
}

fn algorithm_binary_search() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Enter a value");
    std::io::stdout().flush();

    let places = get_places();
    let answer = binary_search(String::from(buf), places);
}

fn get_places() -> Vec<String> {
    let places = File::open("/home/bolt/Repos/random-name/places.txt").unwrap();
    let br: Vec<String> = BufReader::new(places)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    br
}

fn binary_search(place_name: String, places: Vec<String>) -> usize {
    let mut low = 0;
    let mut high = places.len() - 1;

    loop {
        let mid = (low + high) / 2;
        println!("{} - {}", mid, places[mid]);

        if place_name.trim() == places[mid] {
            return mid;
        } else if place_name < places[mid] {
            high = mid;
        } else {
            low = mid;
        }

        if low > high {
            return 0;
        }
    }
}
