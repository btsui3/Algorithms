use core::panic;

use algorithms::bubble_sort::bubble_sort;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
    .map(|s|s.as_str())
    .unwrap_or("None");

    let v = vec![1, 34, 6, 12, 8, 100, 320, 66, 90, 2000, 45, 65, 120];

    println!("v = {:?}", v);

    let result = match problem {
        "bubble_sort" => bubble_sort(&mut v)
    };
    println!("{}", result); 
}