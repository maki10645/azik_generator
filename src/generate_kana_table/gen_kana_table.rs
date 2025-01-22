use std::vec;

#[derive(Clone, Debug, Copy)]
pub enum Consonants {
    K,
    Ky,
    G,
    Gy,
    S,
    Sy,
    Sh,
    Sg,
    Z,
    J,
    Zy,
    Zg,
    T,
    Ty,
    Th,
    D,
    Dy,
    Dh,
    N,
    Ny,
    H,
    Hy,
    Hg,
    F,
    Fy,
    B,
    By,
    Bg,
    P,
    Py,
    Pg,
    M,
    My,
    Y,
    R,
    Ry,
    W,
    Wh,
    V,
}

#[derive(Clone, Debug, Copy)]
pub enum Vowels {
    A,
    I,
    U,
    E,
    O,
}

pub fn gen_consonants_array() -> Vec<Consonants> {
    let mut out: Vec<Consonants> = Vec::with_capacity(39);
    out.extend(vec![
        Consonants::K,
        Consonants::Ky,
        Consonants::G,
        Consonants::Gy,
        Consonants::S,
        Consonants::Sy,
        Consonants::Sh,
        Consonants::Sg,
        Consonants::Z,
        Consonants::J,
        Consonants::Zy,
        Consonants::Zg,
        Consonants::T,
        Consonants::Ty,
        Consonants::Th,
        Consonants::D,
        Consonants::Dy,
        Consonants::Dh,
        Consonants::N,
        Consonants::Ny,
        Consonants::H,
        Consonants::Hy,
        Consonants::Hg,
        Consonants::F,
        Consonants::Fy,
        Consonants::B,
        Consonants::By,
        Consonants::Bg,
        Consonants::P,
        Consonants::Py,
        Consonants::Pg,
        Consonants::M,
        Consonants::My,
        Consonants::Y,
        Consonants::R,
        Consonants::Ry,
        Consonants::W,
        Consonants::Wh,
        Consonants::V,
    ]);
    out
}

fn gen_vowels_array() -> Vec<Vowels> {
    let mut out: Vec<Vowels> = Vec::with_capacity(5);

    out.extend(vec![Vowels::A, Vowels::I, Vowels::U, Vowels::E, Vowels::O]);
    out
}

fn gen_hiragana(consonant: Consonants, vowel: Vowels) -> &'static str {
    match consonant {
        Consonants::K => match vowel {
            Vowels::A => "か",
            Vowels::I => "き",
            Vowels::U => "く",
            Vowels::E => "け",
            Vowels::O => "こ",
        },
        Consonants::Ky => match vowel {
            Vowels::A => "きゃ",
            Vowels::I => "きぃ",
            Vowels::U => "きゅ",
            Vowels::E => "きぇ",
            Vowels::O => "きょ",
        },
        Consonants::G => match vowel {
            Vowels::A => "が",
            Vowels::I => "ぎ",
            Vowels::U => "ぐ",
            Vowels::E => "げ",
            Vowels::O => "ご",
        },
        Consonants::Gy => match vowel {
            Vowels::A => "ぎゃ",
            Vowels::I => "ぎぃ",
            Vowels::U => "ぎゅ",
            Vowels::E => "ぎぇ",
            Vowels::O => "ぎょ",
        },
        Consonants::S => match vowel {
            Vowels::A => "さ",
            Vowels::I => "し",
            Vowels::U => "す",
            Vowels::E => "せ",
            Vowels::O => "そ",
        },
        Consonants::Sy => match vowel {
            Vowels::A => "しゃ",
            Vowels::I => "しぃ",
            Vowels::U => "しゅ",
            Vowels::E => "しぇ",
            Vowels::O => "しょ",
        },
        Consonants::Sh => match vowel {
            Vowels::A => "しゃ",
            Vowels::I => "しぃ",
            Vowels::U => "しゅ",
            Vowels::E => "しぇ",
            Vowels::O => "しょ",
        },
        Consonants::Sg => match vowel {
            Vowels::A => "しゃ",
            Vowels::I => "しぃ",
            Vowels::U => "しゅ",
            Vowels::E => "しぇ",
            Vowels::O => "しょ",
        },
        Consonants::Z => match vowel {
            Vowels::A => "ざ",
            Vowels::I => "じ",
            Vowels::U => "ず",
            Vowels::E => "ぜ",
            Vowels::O => "ぞ",
        },
        Consonants::J => match vowel {
            Vowels::A => "じゃ",
            Vowels::I => "じぃ",
            Vowels::U => "じゅ",
            Vowels::E => "じぇ",
            Vowels::O => "じょ",
        },
        Consonants::Zy => match vowel {
            Vowels::A => "じゃ",
            Vowels::I => "じぃ",
            Vowels::U => "じゅ",
            Vowels::E => "じぇ",
            Vowels::O => "じょ",
        },
        Consonants::Zg => match vowel {
            Vowels::A => "じゃ",
            Vowels::I => "じぃ",
            Vowels::U => "じゅ",
            Vowels::E => "じぇ",
            Vowels::O => "じょ",
        },
        Consonants::T => match vowel {
            Vowels::A => "た",
            Vowels::I => "ち",
            Vowels::U => "つ",
            Vowels::E => "て",
            Vowels::O => "と",
        },
        Consonants::Ty => match vowel {
            Vowels::A => "ちゃ",
            Vowels::I => "ちぃ",
            Vowels::U => "ちゅ",
            Vowels::E => "ちぇ",
            Vowels::O => "ちょ",
        },
        Consonants::Th => match vowel {
            Vowels::A => "てゃ",
            Vowels::I => "てぃ",
            Vowels::U => "てゅ",
            Vowels::E => "てぇ",
            Vowels::O => "てょ",
        },
        Consonants::D => match vowel {
            Vowels::A => "だ",
            Vowels::I => "ぢ",
            Vowels::U => "づ",
            Vowels::E => "で",
            Vowels::O => "ど",
        },
        Consonants::Dy => match vowel {
            Vowels::A => "ぢゃ",
            Vowels::I => "ぢぃ",
            Vowels::U => "ぢゅ",
            Vowels::E => "ぢぇ",
            Vowels::O => "ぢょ",
        },
        Consonants::Dh => match vowel {
            Vowels::A => "でゃ",
            Vowels::I => "でぃ",
            Vowels::U => "でゅ",
            Vowels::E => "でぇ",
            Vowels::O => "でょ",
        },
        Consonants::N => match vowel {
            Vowels::A => "な",
            Vowels::I => "に",
            Vowels::U => "ぬ",
            Vowels::E => "ね",
            Vowels::O => "の",
        },
        Consonants::Ny => match vowel {
            Vowels::A => "にゃ",
            Vowels::I => "にぃ",
            Vowels::U => "にゅ",
            Vowels::E => "にぇ",
            Vowels::O => "にょ",
        },
        Consonants::H => match vowel {
            Vowels::A => "は",
            Vowels::I => "ひ",
            Vowels::U => "ふ",
            Vowels::E => "へ",
            Vowels::O => "ほ",
        },
        Consonants::Hy => match vowel {
            Vowels::A => "ひゃ",
            Vowels::I => "ひぃ",
            Vowels::U => "ひゅ",
            Vowels::E => "ひぇ",
            Vowels::O => "ひょ",
        },
        Consonants::Hg => match vowel {
            Vowels::A => "ひゃ",
            Vowels::I => "ひぃ",
            Vowels::U => "ひゅ",
            Vowels::E => "ひぇ",
            Vowels::O => "ひょ",
        },
        Consonants::F => match vowel {
            Vowels::A => "ふぁ",
            Vowels::I => "ふぃ",
            Vowels::U => "ふ",
            Vowels::E => "ふぇ",
            Vowels::O => "ふぉ",
        },
        Consonants::Fy => match vowel {
            Vowels::A => "ふゃ",
            Vowels::I => "ふぃ",
            Vowels::U => "ふゅ",
            Vowels::E => "ふぇ",
            Vowels::O => "ふょ",
        },
        Consonants::B => match vowel {
            Vowels::A => "ば",
            Vowels::I => "び",
            Vowels::U => "ぶ",
            Vowels::E => "べ",
            Vowels::O => "ぼ",
        },
        Consonants::By => match vowel {
            Vowels::A => "びゃ",
            Vowels::I => "びぃ",
            Vowels::U => "びゅ",
            Vowels::E => "びぇ",
            Vowels::O => "びょ",
        },
        Consonants::Bg => match vowel {
            Vowels::A => "びゃ",
            Vowels::I => "びぃ",
            Vowels::U => "びゅ",
            Vowels::E => "びぇ",
            Vowels::O => "びょ",
        },
        Consonants::P => match vowel {
            Vowels::A => "ぱ",
            Vowels::I => "ぴ",
            Vowels::U => "ぷ",
            Vowels::E => "ぺ",
            Vowels::O => "ぽ",
        },
        Consonants::Py => match vowel {
            Vowels::A => "ぴゃ",
            Vowels::I => "ぴぃ",
            Vowels::U => "ぴゅ",
            Vowels::E => "ぴぇ",
            Vowels::O => "ぴょ",
        },
        Consonants::Pg => match vowel {
            Vowels::A => "ぴゃ",
            Vowels::I => "ぴぃ",
            Vowels::U => "ぴゅ",
            Vowels::E => "ぴぇ",
            Vowels::O => "ぴょ",
        },
        Consonants::M => match vowel {
            Vowels::A => "ま",
            Vowels::I => "み",
            Vowels::U => "む",
            Vowels::E => "め",
            Vowels::O => "お",
        },
        Consonants::My => match vowel {
            Vowels::A => "みゃ",
            Vowels::I => "みぃ",
            Vowels::U => "みゅ",
            Vowels::E => "みぇ",
            Vowels::O => "みょ",
        },
        Consonants::Y => match vowel {
            Vowels::A => "や",
            Vowels::I => "い",
            Vowels::U => "ゆ",
            Vowels::E => "いぇ",
            Vowels::O => "よ",
        },
        Consonants::R => match vowel {
            Vowels::A => "ら",
            Vowels::I => "り",
            Vowels::U => "る",
            Vowels::E => "れ",
            Vowels::O => "ろ",
        },
        Consonants::Ry => match vowel {
            Vowels::A => "りゃ",
            Vowels::I => "りぃ",
            Vowels::U => "りゅ",
            Vowels::E => "りぇ",
            Vowels::O => "りょ",
        },
        Consonants::W => match vowel {
            Vowels::A => "わ",
            Vowels::I => "うぃ",
            Vowels::U => "う",
            Vowels::E => "うぇ",
            Vowels::O => "を",
        },
        Consonants::Wh => match vowel {
            Vowels::A => "うぁ",
            Vowels::I => "うぃ",
            Vowels::U => "う",
            Vowels::E => "うぇ",
            Vowels::O => "うぉ",
        },
        Consonants::V => match vowel {
            Vowels::A => "ゔぁ",
            Vowels::I => "ゔぃ",
            Vowels::U => "ゔ",
            Vowels::E => "ゔぇ",
            Vowels::O => "ゔぉ",
        },
    }
}

pub fn gen_hiragana_table() -> String {
    let mut out = String::new();

    for i in 0..39 {
        let consonant = *gen_consonants_array().get(i).unwrap();
        for j in 0..5 {
            let vowel = *gen_vowels_array().get(j).unwrap();

            out.push_str(gen_hiragana(consonant, vowel));
        }
    }

    out
}

#[cfg(test)]
include!("./gen_kana_table_test.rs");
