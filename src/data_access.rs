use std::{error::Error, result, str::FromStr};

use persy::{Config, Persy, PersyId, SegmentId, ValueIter, ValueMode};

use crate::{structs::Clip, enums::ClipboardItem};

const CLIPS: &str = "clips";
const INDEX_NAME: &str = "name_index";

pub fn save_clip(item: &Clip) -> Result<PersyId, Box<dyn Error>> {
    let persy = open_database()?;

    let mut transaction = persy.begin()?;

    if !transaction.exists_segment(CLIPS)? {
        transaction.create_segment(CLIPS)?;
    }

    let clip_bytes = bincode::serialize(&item)?;

    let result = transaction.insert(CLIPS, &clip_bytes)?;

    if !transaction.exists_index(INDEX_NAME)? {
        transaction.create_index::<String, PersyId>(INDEX_NAME, ValueMode::Cluster)?;
    }

    let persy_id_string = result.to_string();

    transaction.put(INDEX_NAME, item.name.to_string(), persy_id_string)?;

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

pub fn get_clip(name: &String) -> Result<Option<ClipboardItem>, Box<dyn Error>> {
    let persy = open_database()?;

    let mut persy_ids: ValueIter<String> = persy.get(INDEX_NAME, name)?;
    if let Some(first) = persy_ids.next() {
        let persy_id = &PersyId::from_str(&first)?;
        let result = persy.read(CLIPS, persy_id)?;
        match result {
            Some(clip_bytes) => {
                let clip: Clip = bincode::deserialize(&clip_bytes)?;
                return Ok(Some(clip.value));
            }
            None => return Ok(None),
        }
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
