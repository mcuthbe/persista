use std::error::Error;

use persy::{Config, Persy, PersyId, ValueIter, ValueMode};

use crate::{enums::ClipboardItem, transformations::clipboard_item_to_u8_vec};

const CLIPS: &str = "clips";
const INDEX_NAME: &str = "name_index";
const INDEX_KEY: &str = "name";

pub fn save_clip(name: &str, item: ClipboardItem) -> Result<PersyId, Box<dyn Error>> {
    let persy = open_database();

    let mut transaction = persy.begin()?;

    transaction.create_index::<String, String>(INDEX_NAME, ValueMode::Cluster)?;
    transaction.put(INDEX_NAME, INDEX_KEY.to_string(), name.to_string())?;

    if !transaction.exists_segment(CLIPS)? {
        transaction.create_segment(CLIPS)?;
    }

    !todo!("Serialize properly here");
    let result = transaction.insert(CLIPS, &clipboard_item_to_u8_vec(item))?;

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

pub fn get_clip(name: &String) -> Result<ValueIter<std::string::String>, Box<dyn Error>> {
    let persy = open_database();

    let result = persy.get(INDEX_NAME, name)?;

    Ok(result)
}

fn open_database() -> Persy {
    let persy = Persy::open_or_create_with("./target/data.persy", Config::new(), |persy| {
        let mut transaction = persy.begin()?;

        transaction.create_segment(CLIPS)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;

        println!("Clips segment and Index successfully created");
        Ok(())
    })
    .expect("Open or create database");

    persy
}
