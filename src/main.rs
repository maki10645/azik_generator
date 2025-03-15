use std::{fs::File, io::Write};

use generate_kana_table::{
    azik_deserializer, gen_corvuss_skk_table, gen_google_ime_table, get_mode,
};

fn main() {
    let config = azik_deserializer();
    let mode = get_mode();
    let corvus = String::from("couvusskk");

    let mut file = File::create("table.txt").expect("hoge");

    match mode.as_deref() {
        Some("corvusskk") => {
            println!("{}", corvus);
            let _ = file.write_all(gen_corvuss_skk_table(config).as_bytes());

            let _ = file.flush();
        }
        Some(_) => {}
        None => {
            let _ = file.write_all(gen_google_ime_table(config).as_bytes());

            let _ = file.flush();
        }
    }
}
