mod test {
    use super::*;

    #[test]
    fn test_about_types() {
        let consonants = Consonants::K as i32;

        assert_eq!(consonants, 0); // Enumをi32とみなしたとき始点が0のインデックスとして使用可能であることを確認

        // assert_eq!(mem::variant_count::<Consonants>(), 39);
        // UnstableチャンネルにてConsonantsが39要素を持つことを確認済み
        // assert_eq!(mem::variant_count::<Vowels>(), 5)
        // UnstableチャンネルにてVowelsが5要素を持つことを確認済み
    }
    #[test]
    fn test_about_functions() {
        // gen_hiraganaが正常に動作することを確認。
        assert_eq!(gen_hiragana(Consonants::K, Vowels::A), "か");

        // gen_consonants_arrayとgen_vowels_arrayの結合テスト
        let consonant = gen_consonants_array();
        let vowels = gen_vowels_array();

        assert_eq!(
            gen_hiragana(*consonant.get(2).unwrap(), *vowels.get(2).unwrap()),
            "ぐ"
        );
    }
}
