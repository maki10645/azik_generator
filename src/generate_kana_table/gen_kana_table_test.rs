mod test {
    use super::*;
    use std::mem;

    #[test]
    fn test() {
        let consonants = Consonants::K as i32;

        assert_eq!(consonants, 0); // Enumをi32とみなしたとき始点が0のインデックスとして使用可能であることを確認

        // assert_eq!(mem::variant_count::<Consonants>(), 39);
        // UnstableチャンネルにてConsonantsが39要素を持つことを確認済み
        // assert_eq!(mem::variant_count::<Vowels>(), 5)
        // UnstableチャンネルにてVowelsが5要素を持つことを確認済み
    }
}
