use std::{error::Error, io::ErrorKind};

use persy::{Config, Persy, PersyId, ValueMode};

const CLIPS: &str = "clips";

pub fn save_clip(text: &str) -> Result<PersyId, Box<dyn Error>> {
    let persy = open_database();

    let mut transaction = persy.begin()?;
    if !transaction.exists_segment(CLIPS)? {
        print!("here");
        transaction.create_segment(CLIPS)?;
    }
    let result = transaction.insert(CLIPS, text.as_bytes())?;
    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

fn get_clip(id: PersyId) -> Result<Vec<u8>, Box<dyn Error>> {
    let persy = open_database();

    let mut transaction = persy.begin()?;
    let result = match transaction.read("clips", &id)? {
        Some(value) => value,
        None => Err(std::io::Error::new(ErrorKind::NotFound, "Clip not found"))?,
    };
    Ok(result)
}

fn open_database() -> Persy {
    let persy = Persy::open_or_create_with("./target/data.persy", Config::new(), |persy| {
        let mut transaction = persy.begin()?;

        transaction.create_segment("clips")?;
        transaction.create_index::<u64, PersyId>("index", ValueMode::Replace)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;

        println!("Clips segment and Index successfully created");
        Ok(())
    })
    .expect("Open or create database");

    println!("Database opened");

    persy
}
