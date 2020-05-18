use dirs::config_dir;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
struct KanjiList {
    idx: usize,
    kanji: Vec<String>,
}

fn ensure_config_or_fail() -> std::path::PathBuf {
    let cfg = config_dir();

    let cfg = match cfg {
        Some(x) => x,
        None => {
            eprint!("Could not find configuration directory");
            std::process::exit(1);
        }
    };

    let my_config = cfg.join("rust-kanji-rand").join("config.json");

    if !my_config.exists() {
        eprint!("My config.json file did not exist. Exiting..");
        std::process::exit(1);
    }
    my_config
}

fn file_to_str(fpath: &std::path::PathBuf) -> String {
    let mut file = match File::open(&fpath) {
        Err(why) => panic!("Couldnt open {}: {}", fpath.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldnt read {}: {}", fpath.display(), why),
        Ok(_file) => s,
    }
}

fn read_kanji_list(json_str: &String) -> KanjiList {
    match serde_json::from_str(&json_str) {
        Err(why) => panic!(
            "Couldn't read json correctly: {}. Reason: {}",
            json_str, why
        ),
        Ok(list) => list,
    }
}

fn write_config(json_str: &String) {
    let cfg = match config_dir() {
        None => panic!("No config directory??"),
        Some(x) => x,
    };

    let config_path = cfg.join("rust-kanji-rand").join("config.json");

    let mut file = match File::create(&config_path) {
        Err(why) => panic!("Couldn't open {}: {}", config_path.display(), why),
        Ok(file) => file,
    };

    match file.write(json_str.as_bytes()) {
        Err(why) => panic!("Could not write kanji to file! Reason: {}", why),
        Ok(_) => (),
    }
}

fn update_and_save(list: &mut KanjiList) {
    list.idx = list.idx + 1;
    if list.idx >= list.kanji.len() {
        list.idx = 0;
        let mut rng = thread_rng();
        list.kanji.shuffle(&mut rng);
    }
    let serialized = match serde_json::to_string(&list) {
        Ok(x) => x,
        Err(why) => panic!("Could not serialize the kanji list! Reason: {}", why),
    };
    write_config(&serialized);
}

fn display_kanji(list: &KanjiList) {
    print!("{}", list.kanji[list.idx]);
}

fn main() {
    let cfg = ensure_config_or_fail();
    let res = file_to_str(&cfg);
    let mut the_list: KanjiList = read_kanji_list(&res);

    if the_list.idx >= the_list.kanji.len() {
        if the_list.kanji.len() == 0 {
            panic!("No kanji in kanji list to select!");
        }
        the_list.idx = 0;
    }

    display_kanji(&the_list);

    update_and_save(&mut the_list);
}
