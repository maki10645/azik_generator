struct Vowels<'a> {
    a: &'a str,
    i: &'a str,
    u: &'a str,
    e: &'a str,
    o: &'a str,
}

impl Vowels<'static> {
    fn order(vowel: &str) {
        match vowel {
            "a" => Ok(0),
            "i" => Ok(0),
            "u" => Ok(0),
            "e" => Ok(0),
            "o" => Ok(0),
            _ => Err(114514),
        };
    }
}

fn static_vowels() -> Vowels<'static> {
    Vowels {
        a: "あ",
        i: "い",
        u: "う",
        e: "え",
        o: "お",
    }
}

struct Consonants<'a> {
    k: [&'a str; 5],
    ky: [&'a str; 5],
    g: [&'a str; 5],
    gy: [&'a str; 5],
    s: [&'a str; 5],
    sy: [&'a str; 5],
    sh: [&'a str; 5],
    sg: [&'a str; 5],
    z: [&'a str; 5],
    j: [&'a str; 5],
    zy: [&'a str; 5],
    zg: [&'a str; 5],
    t: [&'a str; 5],
    ty: [&'a str; 5],
    th: [&'a str; 5],
    d: [&'a str; 5],
    dy: [&'a str; 5],
    dh: [&'a str; 5],
    n: [&'a str; 5],
    ny: [&'a str; 5],
    h: [&'a str; 5],
    hy: [&'a str; 5],
    hg: [&'a str; 5],
    f: [&'a str; 5],
    fy: [&'a str; 5],
    b: [&'a str; 5],
    by: [&'a str; 5],
    bg: [&'a str; 5],
    p: [&'a str; 5],
    py: [&'a str; 5],
    pg: [&'a str; 5],
    m: [&'a str; 5],
    my: [&'a str; 5],
    r: [&'a str; 5],
    ry: [&'a str; 5],
    w: [&'a str; 5],
    wh: [&'a str; 5],
    y: [&'a str; 5],
    v: [&'a str; 5],
}

impl Consonants<'static> {}

fn static_hiragana() -> Consonants<'static> {
    Consonants {
        k: ["か", "き", "く", "け", "こ"],
        ky: ["きゃ", "きぃ", "きゅ", "きぇ", "きょ"],
        g: ["が", "ぎ", "ぐ", "げ", "ご"],
        gy: ["ぎゃ", "ぎぃ", "ぎゅ", "ぎぇ", "ぎょ"],
        s: ["さ", "し", "す", "せ", "そ"],
        sy: ["しゃ", "し", "しゅ", "しぇ", "しょ"],
        sh: ["しゃ", "し", "しゅ", "しぇ", "しょ"],
        sg: ["しゃ", "し", "しゅ", "しぇ", "しょ"],
        z: ["ず", "じ", "ず", "ぜ", "ぞ"],
        j: ["じゃ", "じ", "じゅ", "じぇ", "じょ"],
        zy: ["じゃ", "じ", "じゅ", "じぇ", "じょ"],
        zg: ["じゃ", "じ", "じゅ", "じぇ", "じょ"],
        t: ["た", "ち", "つ", "て", "と"],
        ty: ["ちゃ", "ちぃ", "ちゅ", "ちぇ", "ちょ"],
        th: ["てゃ", "てぃ", "てゅ", "てぇ", "てょ"],
        d: ["だ", "ぢ", "づ", "で", "ど"],
        dy: ["ぢゃ", "ぢぃ", "ぢゅ", "ぢぇ", "ぢょ"],
        dh: ["でゃ", "でぃ", "でゅ", "でぇ", "でょ"],
        n: ["な", "に", "ぬ", "ね", "の"],
        ny: ["にゃ", "に", "にゅ", "にぇ", "にょ"],
        h: ["は", "ひ", "ふ", "へ", "ほ"],
        hy: ["ひゃ", "ひ", "ひゅ", "ひぇ", "ひょ"],
        hg: ["ひゃ", "ひ", "ひゅ", "ひぇ", "ひょ"],
        f: ["ふぁ", "ふぃ", "ふ", "ふぇ", "ふぉ"],
        fy: ["ふゃ", "ふぃ", "ふゅ", "ふぇ", "ふょ"],
        b: ["ば", "び", "ぶ", "べ", "ぼ"],
        by: ["びゃ", "び", "びゅ", "びぇ", "びょ"],
        bg: ["びゃ", "び", "びゅ", "びぇ", "びょ"],
        p: ["ぱ", "ぴ", "ぷ", "ぺ", "ぽ"],
        py: ["ぴゃ", "ぴ", "ぴゅ", "ぴぇ", "ぴょ"],
        pg: ["ぴゃ", "ぴ", "ぴゅ", "ぴぇ", "ぴょ"],
        m: ["ま", "み", "む", "め", "も"],
        my: ["みゃ", "み", "みゅ", "みぇ", "みょ"],
        r: ["ら", "り", "る", "れ", "ろ"],
        ry: ["りゃ", "り", "りゅ", "りぇ", "りょ"],
        w: ["わ", "うぃ", "う", "うぇ", "を"],
        wh: ["うぁ", "うぃ", "う", "うぇ", "うぉ"],
        y: ["や", "い", "ゆ", "いぇ", "よ"],
        v: ["ゔぁ", "ゔぃ", "ゔ", "ゔぇ", "ゔぉ"],
    }
}

struct AzikTokens {
    mini_tsu: String,
    nn: String,
}

impl AzikTokens {
    fn new(nn: String, mini_tsu: String) -> AzikTokens {
        AzikTokens { mini_tsu, nn }
    }

    fn default_init() -> AzikTokens {
        AzikTokens {
            mini_tsu: String::from("v"),
            nn: String::from("c"),
        }
    }
}

pub fn gen_kana_table() {
    let ou = "ou";
    let vowels = static_vowels();

    let vowel = ou.chars().next().unwrap();
}

#[cfg(test)]
include!("./gen_kana_table_test.rs");
