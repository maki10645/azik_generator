use super::assignable_tokens::Assignable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AzikConfig {
    pub Add: Vec<Assignable>,
    pub Sokuon: String,
    pub Hatsuon: String,
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
