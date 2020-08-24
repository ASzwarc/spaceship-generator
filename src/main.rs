use multimap::MultiMap;
use std::env;
use std::fs;
use std::fmt;
use rand::prelude::*;

mod error;

use error::Error;

#[derive(Debug)]
struct Spaceship {
    engine: String,
    fuselage: String,
    cabin: String,
    small_wings: Option<String>,
    big_wings: Option<String>,
    armor: String,
    weapons: Vec<String>,
}

impl Spaceship {
    fn generate_from_file(avail_parts: &MultiMap<&str, String>) -> Result<Spaceship, Error> {
        Ok(Spaceship {
            engine: draw_element(avail_parts.get_vec("engine").ok_or_else(|| Error::LackOfPartInTheFile("engine".to_string()))?),
            fuselage: avail_parts.get("fuselage").unwrap().to_string(),
            cabin: avail_parts.get("cabin").unwrap().to_string(),
            small_wings: if *avail_parts.get("wings").unwrap() == "no" {
                None
            } else {
                Some(avail_parts.get("wings").unwrap().to_string())
            },
            big_wings: if *avail_parts.get("wings").unwrap() == "no" {
                None
            } else {
                Some(avail_parts.get("wings").unwrap().to_string())
            },
            armor: avail_parts.get("armor").unwrap().to_string(),
            weapons: vec![avail_parts.get("weapon").unwrap().to_string()],
        })
    }
}

impl fmt::Display for Spaceship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "A ship with:")?;
        writeln!(f, "{} engine,", self.engine)?;
        writeln!(f, "{} fuselage,", self.fuselage)?;
        writeln!(f, "{} cabin,", self.cabin)?;
        if let Some(small_wings) = &self.small_wings {
            writeln!(f, "{} wings,", small_wings)?;
        }
        if let Some(big_wings) = &self.big_wings {
            writeln!(f, "{} wings,", big_wings)?;
        }
        writeln!(f, "{} armor,", self.armor)?;
        write!(f, "weapons:\n")?;
        if self.weapons.is_empty() {
            writeln!(f, "None")?;
        } else {
            for weapon in &self.weapons {
                writeln!(f, "{},", weapon)?;
            }
        }
        Ok(())
    }
}

fn draw_element(avail_parts: &Vec<String>) -> String {
    let mut rng = thread_rng();
    let element = rng.gen_range(0, avail_parts.len());
    avail_parts[element].to_string()
}

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        return Err(Error::WrongNumberOfArguments(args.len() as u16));
    }

    let filename = &args[1];
    println!("Filename {}", filename);

    let mut spaceship_parts = MultiMap::new();

    let contents = fs::read_to_string(filename).unwrap();
    for iter in contents.lines() {
        let mut elems: Vec<&str> = iter.split_whitespace().collect();
        // TODO: think about error handling and what should be done if element doesn't met requirements
        let key = elems.pop().expect("Something went wrong and it shouldn't");
        let value = elems.join(" ");
        spaceship_parts.insert(key, value);
    }
    println!("{:?}", spaceship_parts);
    let spaceship = Spaceship::generate_from_file(&spaceship_parts);
    println!("Generated spaceship:\n{}", spaceship.expect("Something went wrong while generating ship!!!"));
    Ok(())
}
