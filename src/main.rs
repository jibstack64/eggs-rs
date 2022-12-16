use std::{fs, process::exit};
use ansi_term::Colour::{Red, Green};

#[derive(Debug)]
enum EggType {
    Brown, White, Pale, Unspecified
}

#[derive(Debug)]
struct Egg {
    name: String,
    lifetime: i32,
    egg_type: EggType,
}

impl Egg {
    fn new() -> Egg {
        return Egg {
            name: "".to_string(),
            lifetime: -1,
            egg_type: EggType::Unspecified
        }
    }
}

fn pretty(text: &str, error: bool) -> String {
    return (if error { Red } else { Green }).paint(text).to_string();
}

fn main() {
    // parse eggies in
    let pr = fs::read_to_string("eggs.txt");
    if pr.is_err() {
        println!("{}", pretty("Failed to read 'egg.txt', where shall thy beautiful eggs lie!!!", true));
        exit(1);
    }
    let contents = pr.unwrap();
    let lines = contents.split("\n"); // readability?!
    let mut egg_arr = Vec::<Egg>::new();
    // no point!!!
    let mut creates = false;
    for l in lines.clone() {
        if l == "NEW" {
            creates = true;
        }
    }
    if !creates {
        println!("{}", pretty("No eggs to create!!!", true));
        exit(0);
    }
    let mut position = 0;
    for n in lines {
        if n == "NEW" {
            if egg_arr.len() == 0 {
                egg_arr.push(Egg::new()); // initial egg :)
                continue;
            } else {
                egg_arr.push(Egg::new());
                position += 1;
                continue;
            }
        }
        let tmp: Vec<&str> = n.split(":").collect();
        if tmp[0] == "NAME" {
            egg_arr[position].name = tmp[1].to_string();
        } else if tmp[0] == "LIFE" {
            egg_arr[position].lifetime = i32::from_str_radix(tmp[1], 10).expect("Failed to parse 'LIFE' - not a valid 32bit integer.");
        } else if tmp[0] == "TYPE" {
            let lwr = tmp[1].to_lowercase();
            egg_arr[position].egg_type = if lwr == "b" { EggType::Brown }
                    else if lwr == "w" { EggType::White } else if lwr == "p"
                    { EggType::Pale } else { EggType::Unspecified }
        } else {
            let tmp = format!("Undefined attribute '{}'.", tmp[0]);
            println!("{}", pretty(tmp.as_str(), true));
            exit(1);
        }
    }
    // print eggy data
    for egg in egg_arr {
        println!("{}", pretty(format!("{:?}", egg).as_str(), false));
    }
}
