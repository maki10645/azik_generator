use std::{fs::File, io::BufReader, str::FromStr};

use serde::{Deserialize, Serialize};

use super::assignable_tokens::{Assignable, AssignableTokens};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct AzikConfig {
    Add: Vec<Assignable>,
    Sokuon: String,
    Hatsuon: String,
}

impl AzikConfig {
    fn gen_sequence(
        Self {
            Add,
            Sokuon,
            Hatsuon,
        }: Self,
    ) {
    }
}

pub fn azik_deserializer() {
    let file = File::open("input.json").expect("Can't read file");
    let reader: BufReader<File> = BufReader::new(file);
    let expected = AzikConfig {
        Add: vec![Assignable {
            Token: AssignableTokens::from_str(",").unwrap().to_string(),
            Sequence: "ou".to_string(),
        }],
        Sokuon: "v".to_string(),
        Hatsuon: "c".to_string(),
    };
    let out: Result<AzikConfig, serde_json::Error> = serde_json::from_reader(reader);

    println!("{:?}", serde_json::to_string(&expected).unwrap());
    println!("{:?}", out.unwrap().Add);
}
