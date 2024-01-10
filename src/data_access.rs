use std::{error::Error, result, str::FromStr};

use persy::{Config, Persy, PersyId, SegmentId, ValueIter, ValueMode};

use crate::structs::Clip;

const CLIPS: &str = "clips";
const INDEX_NAME: &str = "name_index";
const INDEX_KEY: &str = "name";

pub fn save_clip(item: &Clip) -> Result<PersyId, Box<dyn Error>> {
    let persy = open_database()?;

    let mut transaction = persy.begin()?;

    if !transaction.exists_segment(CLIPS)? {
        transaction.create_segment(CLIPS)?;
    }

    let clip_bytes = bincode::serialize(&item)?;

    let result = transaction.insert(CLIPS, &clip_bytes)?;

    transaction.create_index::<String, PersyId>(INDEX_NAME, ValueMode::Cluster)?;
    transaction.put(INDEX_NAME, INDEX_KEY.to_string(), result.to_string())?;

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

pub fn get_clip(name: &String) -> Result<Option<String>, Box<dyn Error>> {
    let persy = open_database()?;

    let mut persy_ids: ValueIter<String> = persy.get(INDEX_NAME, name)?;
    if let Some(first) = persy_ids.next() {
        let persy_id = &PersyId::from_str(&first)?;
        let value = persy.read(CLIPS, persy_id);
        return Ok(Some(first.to_owned()));
    }

    Ok(None)
}

fn open_database() -> Result<Persy, Box<dyn Error>> {
    let persy = Persy::open_or_create_with("./target/data.persy", Config::new(), |persy| {
        let mut transaction = persy.begin()?;

        transaction.create_segment(CLIPS)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;

        println!("Clips segment and Index successfully created");
        Ok(())
    })?;

    Ok(persy)
}
