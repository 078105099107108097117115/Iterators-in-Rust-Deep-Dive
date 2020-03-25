#![allow(unconditional_panic)]
use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let mut populations: HashMap<&str, u32> = HashMap::new();
    populations.insert("Nairobi", 3_000_000);
    populations.insert("Kisumu", 599_468);
    populations.insert("Tokyo", 32_000_000);
    populations.insert("Dar es Salaam", 4_000_000);
    populations.insert("London", 9_000_000);
    populations.insert("New York", 8_175_000);

    let output = get_max(&populations);
    println!("{}", output);

    println!("***********************************************************************");
    let pop_1 = match populations
        .iter()
        .find(|&(_city, pop)| pop > &(1_000_000 as u32))
    {
        Some(val) => format!("{:?}", val),
        None => "No such value".into(),
    };
    println!("{:?}", pop_1);
    let pop_2 = match populations
        .iter()
        .find(|&(_city, pop)| pop.lt(&(500_000 as u32)))
    {
        Some(val) => format!("{:?}", val),
        None => "Error: No such value!".into(),
    };

    println!("{:?}", pop_2);
    println!("**********************************************************************");

    let squares = (0..10)
        .map(|i| (i as isize).pow(i as u32))
        .collect::<Vec<isize>>();
    println!("{:?}", squares);
    println!("Let us test out the std::iter::Iterator::{{le,gt,ge,lt}} methods for lexicographic comparisons");
    let x = "Emperor Napoleon Bonaparte of Corsica";
    println!("x == {}", x);
    let y = "Archimedes of Syracuse";
    println!("y == {}", y);
    let z = "  Tyrant Hiero of Syracuse";
    println!("z == {}", z);
    let n = "Daimyo and Shogun Tokugawa Leyasu";
    println!("n == {}", n);
    println!("Is z > x ? ==> {}", z.gt(x));

    let a: [i32; 0] = [];
    let b = [1, 34, 565, 0, -23, 12, 5, 18, 81, 27, 256];
    println!("a == {:?}", a);
    println!("b == {:?}", b);
    println!("Get the maximum value in the array b");
    let max_b = b
        .iter()
        .fold(b[0], |accum, &elem| std::cmp::max(accum, elem));
    println!("maximum value in array b == {}", max_b);

    let words = vec![
        "Pack",
        "your",
        "bags",
        "immediately",
        ".",
        "Do",
        "you",
        "hear",
        "me",
        "?",
    ];
    let sentence = words.iter().fold(String::new(), |mut accum, &elem| {
        let elem = elem.trim();
        accum.push_str(elem);
        accum.push(' ');
        accum
    });
    println!("{}", sentence);
    let auction = vec!["once", "twice", "chicken soup with bismati rice"];
    println!("auction vec == {:?}", auction);
    for item in auction.iter().take(2).enumerate() {
        println!("{:?}", item);
    }
    let proclammation: Vec<_> = repeat("going").zip(auction.iter().take(2)).collect();
    let end_proclammation: Vec<_> = repeat("sold to the gentleman in the fedora")
        .zip(auction.iter().skip(2))
        .collect();
    println!("Proclammation == {:?}", proclammation);
    println!("end_proclammation == {:?}", end_proclammation);
    let final_proclammation = proclammation
        .into_iter()
        .chain(end_proclammation)
        .collect::<Vec<_>>();
    println!("Final == {:?}", final_proclammation);

    let things = vec![
        "protozoan",
        "stone",
        "centipede",
        "lion",
        "bear",
        "doorknob",
        "grapefruit",
    ];
    println!("*********************************************************************");
    println!(
        "{:?}",
        &things
            .iter()
            .map(|thing| thing.as_bytes())
            .collect::<Vec<_>>()
    );
    println!("");
    println!("");
    println!(
        "{:?}",
        &things
            .iter()
            .map(|thing| thing.as_bytes().iter().map(|x| x & 1).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    println!("");
    println!("");
    println!(
        "{:?}",
        &things
            .iter()
            .map(|thing| thing.as_bytes()[0] & 1 != 0)
            .collect::<Vec<_>>()
    );
    println!("");
    println!("");
    let (living, non_living): (Vec<&str>, Vec<&str>) =
        things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);
    println!("living == {:?} ; non-living == {:?}.", living, non_living);
}

fn get_max(x: &HashMap<&str, u32>) -> String {
    x.iter()
        .max_by_key(|&(_k, v)| v)
        .map(|(&k, &v)| format!(" The city of {} has the greatest population of {}", &k, &v))
        .unwrap_or_else(|| "Error".into())
}
