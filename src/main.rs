use std::fs;

#[derive(Debug)]
enum EggType {
    Brown, White, Pale
}

#[derive(Debug)]
struct Egg {
    name: String,
    lifetime: i32,
    egg_type: EggType,
}

fn main() {
    let contents = fs::read_to_string("egg.txt").expect("Failed to find 'egg.txt'.");
    let mut egg_arr: [Egg; 10] = [];
    let mut position = 0;
    for n in contents.split("\n") {
        if position == egg_arr.len() {
            break;
        }
        if n != "STOP" {
            let tmp: Vec<&str> = n.split(":").collect();
            egg_arr[position].name = tmp[1].to_string();
        } else {
            position += 1;
        }
    }
    let my_egg = Egg {
        name: "Jeffery".to_string(),
        lifetime: 12,
        egg_type: EggType::Pale
    };

    println!("{:?}", my_egg);
}
