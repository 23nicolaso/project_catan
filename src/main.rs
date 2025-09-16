// singleplayer catan
// every hex with rolled number produces resources if player has settlement on its edge. A settlement earns resources based on the type of the settlement, the quality of the settlement, randomized conditions (good weather, bad weather, each condition lasts 4 hours). 
// if a 7 is rolled, if you have > 7 resource cards, half are consumed to "pay taxes".
// if you connect to each of the ports you can trade on the exchanges, to buy/sell resource islands, land, stocks in top companies. You can also connect to the contracts market to get short term contracts (EMPLOYMENT, LOANS, ETC). 
// ON EACH TURN YOU DO THE FOLLOWING:
// 1. Roll 2 six-sided dice, get resources accordingly. 
// 2. trade 
// 3. build / upgrade properties if you can afford it

use rand::prelude::*;
use std::{io, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Resource {
    Brick,
    Wood,
    Wheat,
    Sheep,
    Ore,
    None
}

const RESOURCES: [Resource; RESOURCE_COUNT] = [
    Resource::Brick,
    Resource::Wood,
    Resource::Wheat,
    Resource::Sheep,
    Resource::Ore,
    Resource::None
];

struct Tile {
    resource: Resource,
    num: i8
}

struct TileMap {
    tiles: Vec<Tile>,
    properties: Vec<usize>
}

const TILE_COUNT: isize = 16;
const RESOURCE_COUNT: usize = 6; // number of enum variants

fn get_resource_emoji(resource: &Resource) -> &str {
    match resource {
        Resource::Brick => "🧱",
        Resource::Wood => "🪵 ",
        Resource::Wheat => "🌾",
        Resource::Sheep => "🐑",
        Resource::Ore => "💎",
        Resource::None => "  "
    }
}

fn input<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut input: String = String::with_capacity(64); 
    
    std::io::stdin()
    .read_line(&mut input)
    .expect("Input could not be read");
    
    input.parse()
}

fn display_map(map: &TileMap) {
    println!("MAP:");
    print!("============================\n");
    let mut row_idx: i8 = 0;
    let row_max: i8 = 4;
    for tile in &map.tiles {
        if tile.resource == Resource::None { 
            print!("|     |");
        }
        else{
            print!("|{} {:>2}|", get_resource_emoji(&tile.resource), tile.num);
        }
        row_idx += 1;
        if row_idx >= row_max {
            row_idx = 0; // Reset row_idx to 0 to start the next row
            print!("\n============================\n"); // Move to the next line
        }
    }

    for i in &map.properties {
        println!("Owned Property: {}", i);
    }

    println!(); // Add a final newline for clean output
}

fn display_resources(resources: [i8; 6]){
    for i in 0..resources.len(){
        if RESOURCES[i] == Resource::None {continue;}
        print!("|{}={}|", get_resource_emoji(&RESOURCES[i]), resources[i]);
    }
    println!("\n");
}

fn main() {
    let mut resources: [i8; 6] = [0; RESOURCE_COUNT];
    let mut rng = rand::rng();

        // Create some sample tiles
    let mut tiles = vec![];
    let properties: Vec<usize> = vec![];

    // Populate the tiles vector with random resources and numbers.
    for _ in 0..TILE_COUNT {
        // Choose a random resource from the RESOURCES array.
        let random_resource = *RESOURCES.choose(&mut rng).unwrap();

        // Generate a random number between 2 and 12.
        let random_num: i8 = rng.random_range(2..=12);

        // Create a new Tile instance and add it to the vector.
        tiles.push(Tile {
            resource: random_resource,
            num: random_num as i8,
        });
    }

        // Create the TileMap instance with the generated tiles.
    let mut map = TileMap {
        tiles,
        properties
    };

    loop{
        let input_string = input::<String>();
        match input_string {
            Ok(value) => {
                if value.trim() == "/map"{
                    display_map(&map);
                }

                else if value.trim() == "/resources"{
                    display_resources(resources);
                }

                else if value.trim().contains("/build"){
                    let mut iter = value.trim().split_ascii_whitespace();
                    iter.next();
                    match iter.next(){
                        None => println!("NONE"),
                        Some(s) => map.properties.push(s.parse().expect("Failed to parse string"))
                    }
                }
                
                else if value.trim() == "/roll"{
                    let _random_num: i8 = rng.random_range(2..=12);
                    println!("Dice rolled a sum of: {}", _random_num);
                    for property_index in &map.properties{
                        if map.tiles[*property_index].num == _random_num {
                            resources[map.tiles[*property_index].resource as usize] += 1;
                            println!("Your property made you some {}! You got 1 ", get_resource_emoji(&map.tiles[*property_index].resource));
                        }
                    }
                }

                else{
                    println!("Your command doesn't match any existing commands. 
                    \nTry running /resources, /build, /map, or /roll");
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
