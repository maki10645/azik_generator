use std::str::FromStr;

use regex::Regex;

use super::{
    assignable_tokens::{Assignable, AssignableTokens},
    gen_consonant::gen_consonants_array,
    gen_kana::{gen_half_katakana, gen_hiragana, gen_katakana},
    gen_vowel::{vowel_to_half_katakana, vowel_to_hiragana, vowel_to_katakana, Vowels},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AzikConfig {
    pub Sequence: Vec<Assignable>,
    pub Sokuon: String,
    pub Hatsuon: String,
}

pub fn gen_google_ime_sequence(config: AzikConfig) -> (String, String) {
    let mut out = String::new();
    let mut out_seq = String::new();
    let hatsuon_sequence = AssignableTokens::from_str(config.Hatsuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Hatsuon in JSON")
        .to_string()
        .to_lowercase()
        + "\t\t"
        + "ん\n";

    out.push_str(&hatsuon_sequence.to_string());

    let sokuon_sequence = AssignableTokens::from_str(config.Sokuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Sokuon in JSON")
        .to_string()
        .to_lowercase()
        + "\t\t"
        + "っ\n\n";

    out.push_str(&sokuon_sequence.to_string());

    let mut delete_list: Vec<String> = vec![];

    for i in &config.Sequence {
        let sequence: Vec<_> = i.Sequence.to_string().chars().collect();
        let kana_vowel = Vowels::from_str(&sequence[0].to_uppercase().to_string().as_str());
        let free_vowel = vowel_to_hiragana(
            Vowels::from_str(&sequence[1].to_uppercase().to_string().as_str())
                .expect("It's not vowel or something"),
        );
        let assignable = i.Token.to_lowercase();

        for j in gen_consonants_array() {
            let consonant = j;
            let kana = gen_hiragana(consonant, kana_vowel.expect("It's not vowel or something"));

            if consonant.to_string().chars().count() == 2 {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "	"
                        + kana
                        + free_vowel
                        + "\n"),
                );
            } else {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "		"
                        + kana
                        + free_vowel
                        + "\n"),
                );
            }
        }
        out_seq.push_str(&assignable);
        out.push("\n".chars().next().unwrap());
    }
    for j in gen_consonants_array() {
        let consonant = j;
        for i in &config.Sequence {
            if consonant.to_string().chars().last().unwrap().to_string() == i.Token.to_lowercase() {
                for t in &config.Sequence {
                    let re = Regex::new(
                        &(consonant.to_string().to_lowercase()
                            + t.Token.as_str()
                            + r"\t\p{Hiragana}*"),
                    )
                    .unwrap();

                    match re.captures(&out) {
                        Some(cap) => {
                            delete_list.push(cap[0].to_string());
                        }
                        None => {}
                    }
                }
                continue;
            }
        }
        if consonant.to_string().to_lowercase() == "v" {
            for t in &config.Sequence {
                let re = Regex::new(&("v".to_string() + t.Token.as_str() + r"\t\t\p{Hiragana}*"))
                    .unwrap();

                match re.captures(&out) {
                    Some(cap) => {
                        delete_list.push(cap[0].to_string());
                    }
                    None => {}
                }
            }
        }
    }

    for i in &delete_list {
        out.remove_matches(i);
    }

    (out, out_seq.to_string())
}

pub fn gen_corvus_skk_sequence(config: AzikConfig) -> (String, String) {
    let mut out = String::new();
    let mut out_seq = String::new();
    let hatsuon_sequence = "\n".to_owned()
        + &(AssignableTokens::from_str(config.Hatsuon.to_uppercase().as_str())
            .expect("It's not Assignable. Did you use vowels or numbers? change Hatsuon in JSON")
            .to_string()
            .to_lowercase())
        + "\tん\tン\tﾝ\t2\n";

    out.push_str(&hatsuon_sequence.to_string());

    let sokuon_sequence = AssignableTokens::from_str(config.Sokuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Sokuon in JSON")
        .to_string()
        .to_lowercase()
        + "\tっ\tッ\tｯ\t2\n\n";

    out.push_str(&sokuon_sequence.to_string());

    let mut delete_list: Vec<String> = vec![];

    for i in &config.Sequence {
        let sequence: Vec<_> = i.Sequence.to_string().chars().collect();
        let kana_vowel = Vowels::from_str(&sequence[0].to_uppercase().to_string().as_str());
        let free_hiragana = vowel_to_hiragana(
            Vowels::from_str(&sequence[1].to_uppercase().to_string().as_str())
                .expect("It's not vowel or something"),
        );
        let free_katakana = vowel_to_katakana(
            Vowels::from_str(&sequence[1].to_uppercase().to_string().as_str())
                .expect("It's not vowel or something"),
        );
        let free_half_katakana = vowel_to_half_katakana(
            Vowels::from_str(&sequence[1].to_uppercase().to_string().as_str())
                .expect("It's not vowel or something"),
        );
        let assignable = i.Token.to_lowercase();

        for j in gen_consonants_array() {
            let consonant = j;
            let hiragana =
                gen_hiragana(consonant, kana_vowel.expect("It's not vowel or something"));
            let katakana =
                gen_katakana(consonant, kana_vowel.expect("It's not vowel or something"));
            let half_katakana =
                gen_half_katakana(consonant, kana_vowel.expect("It's not vowel or something"));

            if consonant.to_string().chars().count() == 2 {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "	"
                        + hiragana
                        + free_hiragana
                        + "\t"
                        + katakana
                        + free_katakana
                        + "\t"
                        + half_katakana
                        + free_half_katakana
                        + "\t0"
                        + "\n"),
                );
            } else {
                out.push_str(
                    &(consonant.to_string().to_lowercase()
                        + &assignable
                        + "	"
                        + hiragana
                        + free_hiragana
                        + "\t"
                        + katakana
                        + free_katakana
                        + "\t"
                        + half_katakana
                        + free_half_katakana
                        + "\t2"
                        + "\n"),
                );
            }
        }
        out_seq.push_str(&assignable);
        out.push("\n".chars().next().unwrap());
    }
    for j in gen_consonants_array() {
        let consonant = j;
        for i in &config.Sequence {
            if consonant.to_string().chars().last().unwrap().to_string() == i.Token.to_lowercase() {
                for t in &config.Sequence {
                    let re = Regex::new(
                        &(consonant.to_string().to_lowercase()
                            + t.Token.as_str()
                            + r"\t([ぁ-んー]*)"
                            + r"\t([ァ-ヴ]*)"
                            + r"\t([[ｦ-ﾟ]]*)"
                            + r"\t([0-9]*)"),
                    )
                    .unwrap();

                    match re.captures(&out) {
                        Some(cap) => {
                            delete_list.push(cap[0].to_string());
                        }
                        None => {}
                    }
                }
                continue;
            }
        }
        if consonant.to_string().to_lowercase() == "v" {
            for t in &config.Sequence {
                let re = Regex::new(
                    &("v".to_string()
                        + t.Token.as_str()
                        + r"\t(\p{Hiragana}*)\t([ァ-ヴ]*)\t([[ｦ-ﾟ]]*)\t([0-9]*)"),
                )
                .unwrap();

                match re.captures(&out) {
                    Some(cap) => {
                        delete_list.push(cap[0].to_string());
                    }
                    None => {}
                }
            }
        }
    }

    for i in &delete_list {
        out.remove_matches(i);
    }

    (out, out_seq.to_string())
}
