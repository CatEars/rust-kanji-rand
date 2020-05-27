use dirs::config_dir;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;

fn initial_config_data() -> String {
    String::from("{\"idx\":0,\"kanji\":[\"金\",\"耳\",\"子\",\"電\",\"土\",\"男\",\"雨\",\"文\",\"気\",\"名\",\"父\",\"音\",\"下\",\"女\",\"年\",\"村\",\"木\",\"一\",\"赤\",\"休\",\"八\",\"二\",\"石\",\"川\",\"午\",\"東\",\"月\",\"貝\",\"中\",\"入\",\"字\",\"花\",\"生\",\"夕\",\"人\",\"四\",\"西\",\"左\",\"話\",\"車\",\"天\",\"食\",\"九\",\"山\",\"手\",\"行\",\"目\",\"六\",\"半\",\"先\",\"長\",\"力\",\"時\",\"足\",\"何\",\"今\",\"田\",\"王\",\"前\",\"高\",\"本\",\"間\",\"青\",\"友\",\"火\",\"外\",\"五\",\"右\",\"三\",\"千\",\"空\",\"竹\",\"聞\",\"草\",\"後\",\"小\",\"来\",\"町\",\"毎\",\"立\",\"学\",\"森\",\"校\",\"百\",\"林\",\"見\",\"糸\",\"口\",\"円\",\"出\",\"母\",\"北\",\"早\",\"南\",\"七\",\"読\",\"万\",\"玉\",\"大\",\"正\",\"上\",\"十\",\"水\",\"分\",\"犬\",\"日\",\"書\",\"語\",\"国\",\"虫\",\"白\"]}")
}

#[derive(Deserialize, Serialize)]
struct KanjiList {
    idx: usize,
    kanji: Vec<String>,
}

fn ensure_config_or_fail() -> std::path::PathBuf {
    let cfg = config_dir().unwrap();

    let my_config_dir = cfg.join("rust-kanji-rand");
    create_dir_all(my_config_dir).unwrap();

    let my_config = cfg.join("rust-kanji-rand").join("config.json");

    if !my_config.exists() {
        write_config(&initial_config_data());
    }

    my_config
}

fn file_to_str(fpath: &std::path::PathBuf) -> String {
    let mut file = File::open(&fpath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn read_kanji_list(json_str: &String) -> KanjiList {
    serde_json::from_str(&json_str).unwrap()
}

fn write_config(json_str: &String) {
    let cfg = config_dir().unwrap();

    let config_path = cfg.join("rust-kanji-rand").join("config.json");

    let mut file = File::create(&config_path).unwrap();
    file.write(json_str.as_bytes()).unwrap();
}

fn update_and_save(list: &mut KanjiList) {
    list.idx = list.idx + 1;
    if list.idx >= list.kanji.len() {
        list.idx = 0;
        let mut rng = thread_rng();
        list.kanji.shuffle(&mut rng);
    }
    let serialized = serde_json::to_string(&list).expect("Could not serialize the kanji list!");

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
