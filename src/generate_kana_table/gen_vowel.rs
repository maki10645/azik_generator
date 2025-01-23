#[derive(Clone, Debug, Copy, strum::Display)]
pub enum Vowels {
    A,
    I,
    U,
    E,
    O,
}

pub fn gen_vowels_array() -> Vec<Vowels> {
    let mut out: Vec<Vowels> = Vec::with_capacity(5);

    out.extend(vec![Vowels::A, Vowels::I, Vowels::U, Vowels::E, Vowels::O]);
    out
}
