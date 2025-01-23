use std::fs::File;
use std::io::{BufReader, Write};

use super::azik_config::AzikConfig;
use super::gen_consonant::gen_consonants_array;
use super::gen_kana::gen_hiragana;
use super::gen_vowel::gen_vowels_array;

pub fn gen_hiragana_table() {
    let mut out = String::new();
    // Azikの対象外を事前に作成
    out.push_str(
        "la	ぁ
li	ぃ
lu	ぅ
le	ぇ
lo	ぉ
lya	ゃ
lyu	ゅ
lyo	ょ
0	0
1	1
2	2
3	3
4	4
5	5
6	6
7	7
8	8
9	9
?	？
!	！
\"	\"
#	#
$	$
%	%
&	&
'	'
(	(
)	)
*	*
+	+
,	、
-	ー
.	。
/	/
:	:
;	ん
<	<
=	=
>	>
?	？
@	@
[	「
\\	\\
]	」
^	^
_	_
`	`
{	{
|	|
}	}
~	~
z<	‥
z~	〜
z>	…
z/	・
z(	（
z)	）
z[	『
z]	』
z{	【
z}	】
zH	←
zJ	↓
zK	↑
zL	→
z;	゛
z:	゜

a	あ
i	い
u	う
e	え
o	お
",
    );

    for i in 0..39 {
        let consonant = *gen_consonants_array().get(i).unwrap();
        let consonant_alph = consonant.to_string();
        for j in 0..5 {
            let vowel = *gen_vowels_array().get(j).unwrap();
            let kana = gen_hiragana(consonant, vowel).to_string() + "\n";
            let vowel_alph = vowel.to_string();

            let alphs = consonant_alph.clone().to_lowercase() + &vowel_alph.to_lowercase();

            out.push_str(&(alphs + "	" + kana.as_str()));
        }
    }
    let mut file = File::create("table.txt").expect("hoge");

    let _ = file.write_all(out.as_bytes());

    let _ = file.flush();
}

pub fn azik_deserializer() {
    let file = File::open("input.json").expect("Can't read file");
    let reader: BufReader<File> = BufReader::new(file);
    let out: Result<AzikConfig, serde_json::Error> = serde_json::from_reader(reader);

    print!("{:?}", out.unwrap())
}
