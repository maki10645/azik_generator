mod generate_kana_table;

use crate::gen_kana_table::{gen_consonants_array, gen_hiragana, gen_vowels_array};

pub use crate::generate_kana_table::gen_kana_table;

fn main() {
    let consonant = *gen_consonants_array().get(0).unwrap();
    let vowel = *gen_vowels_array().get(0).unwrap();

    print!("{}", gen_hiragana(consonant, vowel))
}
