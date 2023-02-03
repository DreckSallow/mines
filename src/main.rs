mod lib;
mod list_methods;
use list_methods::{AppList, ResultAppList};
use std::{fs, io, path::Path};

fn main() {
    let max_init = 100;
    let numbers = get_numbers("./numbers.txt").expect("Ocurred an error");

    if numbers.len() <= max_init {
        println!("The mines are smaller than: {}", max_init);
        return;
    }
    println!("Length of mines: {}", numbers.len());
    let mut first_numbers: Vec<u128> = numbers[0..max_init].iter().map(|n| n.to_owned()).collect();
    let rest_numbers: Vec<u128> = numbers[max_init..].iter().map(|n| n.to_owned()).collect();

    let mut app_list = AppList::new();
    app_list.add_insecure_list(rest_numbers);
    app_list.add_safe_list(first_numbers.clone());
    first_numbers.sort(); // Sort the array to improve the search!
    app_list.add_sorted_list(first_numbers);

    // Run the app finding a insecure mine
    let (node_count, result) = app_list.run();
    println!("Read mines {node_count} + {max_init} (safe mines)");
    match result {
        ResultAppList::Insecure(n) => {
            println!("Mine: {} is insecure", n);
        }
        ResultAppList::Secure => {
            println!("The list of mines are safe");
        }
        ResultAppList::EmptyList => {
            println!("The list of mines are empty");
        }
        ResultAppList::Missing => {
            println!("Missing fields in list");
        }
    }
}

pub fn get_numbers<P: AsRef<Path>>(path: P) -> io::Result<Vec<u128>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<u128> = content
        .lines()
        .map(|l| {
            l.to_owned()
                .parse::<u128>()
                .expect(&format!("Cannot parse the number: {}", l))
        })
        .collect();

    Ok(lines)
}
