# AZik_Generator

JsonでAzikが実装されたGoogle IMEのかなテーブルが生成できます
木村氏のAzikの割り当てが気に食わない、またはqwerty以外の配列でもAzikが使いたい
というような方は試してみてください

インストールは

```bash
cargo install azik_generator
```

で実行は

```bash
azik_generator input.json
```

となっています

入力するべきjsonは以下の様です

```jsonc
// input.json
{
  "Sequence": [
    {
      "Token": "c",
      "Sequence": "ou"
    },
    {
      "Token": ",",
      "Sequence": "ai"
    },
    {
      "Token": "v",
      "Sequence": "ei"
    },
    {
      "Token": "l",
      "Sequence": "uu"
    },
    {
      "Token": "-",
      "Sequence": "oi"
    },
    {
      "Token": "n",
      "Sequence": "an"
    },
    {
      "Token": "s",
      "Sequence": "in"
    },
    {
      "Token": "r",
      "Sequence": "un"
    },
    {
      "Token": "h",
      "Sequence": "en"
    },
    {
      "Token": "t",
      "Sequence": "on"
    }
  ],
  "Sokuon": "v",
  "Hatsuon": "c"
}
```

これは私が作っている[まきゆきAZIK4大西](https://github.com/maki-07061210/MakiyukiAZIK4Ohnishi)とおおよそ同一の動作をするためのjsonです

Hatsuon(撥音)とは"ん"のことであり
Sokuon(促音)とは"っ"のことです。

現状では完全にまきゆきAZIK4大西の動作をするものでは有りませんが、出力されたtxtファイルを編集することで
普通に1からIMEのかなテーブルを編集するより100倍位ラクです。

現状はこの状態で公開しますが、以下の更新の余地を残しています

- [x] シーケンスと子音か被った場合(shでしゃ行が打ちたいがhにennが割り当てられているなど)の対処[このようにすれば対処可能](https://makiyuki.blog/blog/updateazik117.html)
- [ ] 特殊拡張の設定を可能に

[紹介記事](https://makiyuki.blog/blog/azik_generator.html)
