use std::{collections::BTreeMap, fs::File, io::BufReader};

pub fn get_champ_id_map() -> BTreeMap<i64, String> {
    let file = File::open("./data/champion_id.json").unwrap();
    let reader = BufReader::new(file);

    let champ_id_map: BTreeMap<i64, String> = serde_json::from_reader(reader).unwrap();
    
    champ_id_map
}