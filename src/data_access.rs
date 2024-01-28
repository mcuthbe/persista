use std::{error::Error, str::FromStr};

use persy::{Config, Persy, PersyId, ValueIter, ValueMode};

use crate::{enums::ClipboardItem, structs::Clip};

const CLIPS: &str = "clips";
const INDEX_NAME: &str = "name_index";

pub fn save_clip(persy: &Persy, item: &Clip) -> Result<PersyId, Box<dyn Error>> {
    let existing_id: Option<String> = persy.one(INDEX_NAME, &item.name)?;
    if (existing_id.is_some()) {
        let persy_id = PersyId::from_str(&existing_id.unwrap())?;
        let mut transaction = persy.begin()?;
        transaction.delete(CLIPS, &persy_id)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;
    }
    let mut transaction = persy.begin()?;

    let clip_bytes = bincode::serialize(&item)?;
    let result = transaction.insert(CLIPS, &clip_bytes)?;
    let persy_id_string = result.to_string();
    transaction.put(INDEX_NAME, item.name.to_string(), persy_id_string)?;

    let prepared = transaction.prepare()?;
    prepared.commit()?;

    Ok(result)
}

pub fn retrieve_clip(
    persy: &Persy,
    name: &String,
) -> Result<Option<ClipboardItem>, Box<dyn Error>> {
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

pub fn search_clips(persy: &Persy, search: &str) -> Result<Vec<Clip>, Box<dyn Error>> {
    let results = persy.scan(CLIPS)?;

    let values: Vec<Clip> = results
        .filter_map(|(_, bytes)| {
            if let Ok(clip) = bincode::deserialize::<Clip>(&bytes) {
                if search.is_empty() || clip.name.contains(search) {
                    Some(clip)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(values)
}

pub fn delete_clip(persy: &Persy, name: &String) -> Result<bool, Box<dyn Error>> {
    let id_string: Option<String> = persy.one(INDEX_NAME, name)?;
    Ok(if let Some(id_string) = id_string {
        let persy_id = PersyId::from_str(&id_string)?;

        let mut transaction = persy.begin()?;
        transaction.delete(CLIPS, &persy_id)?;

        let prepared = transaction.prepare()?;
        prepared.commit()?;
        true
    } else {
        false
    })
}

pub fn open_database(db_name: &str) -> Result<Persy, Box<dyn Error>> {
    println!("{}", &db_name);
    let persy = Persy::open_or_create_with(db_name, Config::new(), |_| Ok({}))?;

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

    fn wrap_test(test: Box<dyn FnOnce(&Persy) -> ()>) {
        let _ = fs::create_dir_all("target/");
        let unique_name = "target/tests/".to_owned() + &generate_unique_value();

        let persy = open_database(&unique_name).unwrap();
        test(&persy);
        let _ = fs::remove_file(unique_name);
    }

    #[test]
    fn save_clip_succeeds() {
        wrap_test(Box::new(|persy| {
            let unique_name = &generate_unique_value();
            let clip = Clip {
                name: (&unique_name).to_string(),
                value: ClipboardItem::Text("Test".to_string()),
            };

            let result = save_clip(&persy, &clip).unwrap();

            let clip_data = persy.read(CLIPS, &result).unwrap().unwrap();
            let deserialized = bincode::deserialize::<Clip>(&clip_data).unwrap();
            assert_eq!(&deserialized.name, unique_name);
        }));
    }

    #[test]
    fn save_duplicate_clip_overwrites() {
        wrap_test(Box::new(|persy| {
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
            let result = save_clip(&persy, &clip).unwrap();

            let clip_data = persy.read(CLIPS, &result).unwrap().unwrap();
            let deserialized = bincode::deserialize::<Clip>(&clip_data).unwrap();
            assert_eq!(deserialized.name, unique_name);
            assert_eq!(deserialized.value.as_str(), "Test2");
        }));
    }

    #[test]
    fn retrieve_clip_returns_same_clip_set_by_save_clip() {
        wrap_test(Box::new(|persy| {
            let unique_name = generate_unique_value();

            let clip = Clip {
                name: (&unique_name).to_string(),
                value: ClipboardItem::Text("Test".to_string()),
            };
            let _ = save_clip(&persy, &clip).unwrap();

            let result = retrieve_clip(&persy, &unique_name).unwrap().unwrap();

            assert_eq!(result.as_str(), "Test");
        }));
    }

    //get successfully retrieves clip
    #[test]
    fn retrieve_clip_success() {
        wrap_test(Box::new(|persy| {
            let unique_name = generate_unique_value();

            let clip = Clip {
                name: (&unique_name).to_string(),
                value: ClipboardItem::Text("Test".to_string()),
            };
            let mut transaction = persy.begin().unwrap();

            let clip_bytes = bincode::serialize(&clip).unwrap();
            let result = transaction.insert(CLIPS, &clip_bytes).unwrap();
            let persy_id_string = result.to_string();
            transaction
                .put(INDEX_NAME, clip.name.to_string(), persy_id_string)
                .unwrap();

            let prepared = transaction.prepare().unwrap();
            prepared.commit().unwrap();
            let _ = save_clip(&persy, &clip).unwrap();

            let result = retrieve_clip(&persy, &unique_name).unwrap().unwrap();

            assert_eq!(result.as_str(), "Test");
        }));
    }

    #[test]
    fn retrieve_clip_returns_none_if_clip_does_not_exist() {
        wrap_test(Box::new(|persy| {
            let unique_name = generate_unique_value();

            let result = retrieve_clip(&persy, &unique_name).unwrap();

            assert!(result.is_none());
        }));
    }

    #[test]
    fn search_clips_returns_all_with_empty_search() {
        wrap_test(Box::new(|persy| {
            let unique_name = generate_unique_value();

            let clip = Clip {
                name: (&unique_name).to_string(),
                value: ClipboardItem::Text("Test".to_string()),
            };
            let _ = save_clip(&persy, &clip).unwrap();

            let result = search_clips(&persy, &"".to_string()).unwrap();

            assert_eq!(result.len(), 1);
            assert_eq!(result[0].name, unique_name);
        }));
    }

    #[test]
    fn search_clips_returns_only_searched_clips() {
        wrap_test(Box::new(|persy| {
            let clip = Clip {
                name: "Test1".to_owned(),
                value: ClipboardItem::Text("Test".to_string()),
            };
            let _ = save_clip(&persy, &clip).unwrap();

            let result = search_clips(&persy, &"Test1".to_string()).unwrap();

            assert_eq!(result.len(), 1);
            assert_eq!(result[0].name, "Test1");
        }));
    }

    #[test]
    fn delete_clip_success() {
        wrap_test(Box::new(|persy| {
            let unique_name = generate_unique_value();

            let clip = Clip {
                name: (&unique_name).to_string(),
                value: ClipboardItem::Text("Test".to_string()),
            };
            let _ = save_clip(&persy, &clip).unwrap();

            let result = delete_clip(&persy, &unique_name).unwrap();

            assert!(result);

            //try get clip, and assert that it returns None
            let result = retrieve_clip(&persy, &unique_name).unwrap();
            assert!(result.is_none());
        }));
    }
}
