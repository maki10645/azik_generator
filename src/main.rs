mod generate_kana_table;

use generate_kana_table::gen_kana_table::gen_hiragana_table;

pub use crate::generate_kana_table::gen_kana_table;

fn main() {
    gen_hiragana_table();
    gen_kana_table::azik_deserializer();
}
