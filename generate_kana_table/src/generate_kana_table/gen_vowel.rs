#[derive(Clone, Debug, Copy, strum::Display, strum::EnumString)]
pub enum Vowels {
    A,
    I,
    U,
    E,
    O,
    N,
}

pub fn gen_vowels_array() -> Vec<Vowels> {
    let mut out: Vec<Vowels> = Vec::with_capacity(5);

    out.extend(vec![Vowels::A, Vowels::I, Vowels::U, Vowels::E, Vowels::O]);
    out
}

pub fn vowel_to_hiragana(vowel: Vowels) -> &'static str {
    match vowel {
        Vowels::A => "あ",
        Vowels::I => "い",
        Vowels::U => "う",
        Vowels::E => "え",
        Vowels::O => "お",
        Vowels::N => "ん",
    }
}
pub fn vowel_to_katakana(vowel: Vowels) -> &'static str {
    match vowel {
        Vowels::A => "ア",
        Vowels::I => "イ",
        Vowels::U => "ウ",
        Vowels::E => "エ",
        Vowels::O => "オ",
        Vowels::N => "ン",
    }
}
pub fn vowel_to_half_katakana(vowel: Vowels) -> &'static str {
    match vowel {
        Vowels::A => "ｱ",
        Vowels::I => "ｲ",
        Vowels::U => "ｳ",
        Vowels::E => "ｴ",
        Vowels::O => "ｵ",
        Vowels::N => "ﾝ",
    }
}
