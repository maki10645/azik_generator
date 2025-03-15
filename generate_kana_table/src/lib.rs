#![feature(string_remove_matches)]
mod generate_kana_table;

pub use generate_kana_table::{
    azik_config::AzikConfig,
    deserealizer::{azik_deserializer, get_mode},
    gen_corvus_skk_table::gen_corvuss_skk_table,
    gen_google_ime_table::gen_google_ime_table,
};
