use std::error::Error;

use persy::{Config, Persy, PersyId};

const CLIPS: &str = "clips";

pub fn save_clip(text: &str) -> Result<PersyId, Box<dyn Error>> {
    Persy::create("./target/data.persy")?;

    let persy = Persy::open("./target/data.persy", Config::new())?;

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
