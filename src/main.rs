use std::{fs::File, io::Write};

use generate_kana_table::{azik_deserializer, gen_hiragana_table};

fn main() {
    let config = azik_deserializer();
    let mut file = File::create("table.txt").expect("hoge");

    let _ = file.write_all(gen_hiragana_table(config).as_bytes());

    let _ = file.flush();
}
