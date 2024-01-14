use std::{error::Error, str::FromStr};

use persy::{Config, Persy, PersyId, ValueMode};

use crate::{structs::Clip, enums::ClipboardItem};

const CLIPS: &str = "clips";
const INDEX_NAME: &str = "name_index";

pub fn save_clip(persy: &Persy, item: &Clip) -> Result<PersyId, Box<dyn Error>> {
    let mut transaction = persy.begin()?;

    let clip_bytes = bincode::serialize(&item)?;
    let result = transaction.insert(CLIPS, &clip_bytes)?;
    let persy_id_string = result.to_string();
    transaction.put(INDEX_NAME, item.name.to_string(), persy_id_string)?;

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

pub fn get_clip(persy: &Persy, name: &String) -> Result<Option<ClipboardItem>, Box<dyn Error>> {
    let id_string: Option<String> = persy.one(INDEX_NAME, name)?;
    if let Some(id_string) = id_string {
        let persy_id = &PersyId::from_str(&id_string)?;
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

pub fn open_database(db_name: &str) -> Result<Persy, Box<dyn Error>> {
    let persy = Persy::open_or_create_with(db_name, Config::new(), |persy| {
        let mut transaction = persy.begin()?;

        transaction.create_segment(CLIPS)?;

        transaction.create_index::<String, String>(INDEX_NAME, ValueMode::Cluster)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;

        println!("Clips segment and Index successfully created");
        Ok(())
    })?;

    let mut transaction = persy.begin()?;

    if !transaction.exists_segment(CLIPS)? {
        transaction.create_segment(CLIPS)?;
    }

    if !transaction.exists_index(INDEX_NAME)? {
        transaction.create_index::<String, String>(INDEX_NAME, ValueMode::Cluster)?;
    }

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(persy)
}


#[cfg(test)]
mod tests {

    use std::fs;

    use crate::util::generate_unique_value;

    use super::*;
    const TEST_DB_NAME: &str = "target/test.persy";

    fn setup() -> Persy {
        let _ = fs::remove_file(TEST_DB_NAME);
        let persy = open_database(TEST_DB_NAME).unwrap();
        persy
    }

    fn teardown() {
        let _ = fs::remove_file(TEST_DB_NAME);
    }

    #[test]
    fn save_clip_succeeds(){
        // Arrange
        let persy = setup();
        let unique_name = generate_unique_value();
        let clip = Clip {
            name: (&unique_name).to_string(),
            value: ClipboardItem::Text("Test".to_string()),
        };

        // Act
        let result = save_clip(&persy, &clip).unwrap();

        // Assert
        let clip_data = persy.read(CLIPS, &result).unwrap().unwrap();
        let deserialized = bincode::deserialize::<Clip>(&clip_data).unwrap();
        assert_eq!(deserialized.name, unique_name);
        teardown();
    }

    #[test]
    fn save_duplicate_clip_overwrites(){
        // Arrange
        let persy = setup();
        let unique_name = generate_unique_value();
        let clip = Clip {
            name: (&unique_name).to_string(),
            value: ClipboardItem::Text("Test".to_string()),
        };
        let _ = save_clip(&persy, &clip).unwrap();
        let clip = Clip {
            name: (&unique_name).to_string(),
            value: ClipboardItem::Text("Test2".to_string()),
        };

        // Act
        let result = save_clip(&persy, &clip).unwrap();

        // Assert
        let clip_data = persy.read(CLIPS, &result).unwrap().unwrap();
        let deserialized = bincode::deserialize::<Clip>(&clip_data).unwrap();
        assert_eq!(deserialized.name, unique_name);
        assert_eq!(deserialized.value.as_str(), "Test2");
        teardown();
    }
}