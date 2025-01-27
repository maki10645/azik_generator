use std::{fs::File, io::Write};

mod generate_kana_table;

use generate_kana_table::{deserealizer::azik_deserializer, gen_kana_table::gen_hiragana_table};

pub use crate::generate_kana_table::{azik_config, deserealizer, gen_kana_table};

fn main() {
    let config = azik_deserializer();
    let mut file = File::create("table.txt").expect("hoge");

    let _ = file.write_all(gen_hiragana_table(config).as_bytes());

    let _ = file.flush();
}
