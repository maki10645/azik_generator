use regex::Regex;

use crate::generate_kana_table::azik_config::gen_sequence;

use super::azik_config::AzikConfig;
use super::gen_consonant::gen_consonants_array;
use super::gen_kana::gen_hiragana;
use super::gen_vowel::gen_vowels_array;

pub fn gen_hiragana_table(config: AzikConfig) -> String {
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
;	;
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
zh	←
zj	↓
zk	↑
zl	→
z;	゛
z:	゜

a	あ
i	い
u	う
e	え
o	お
",
    );
    let gen_seq = gen_sequence(config);
    let sequences = gen_seq.0;
    let tokens = gen_seq.1;
    out.push_str(sequences.as_str());

    for i in 0..39 {
        let consonant = *gen_consonants_array().get(i).unwrap();
        let consonant_alph = consonant.to_string().to_lowercase();
        for j in 0..5 {
            let mut dup_flag = false;
            let vowel = *gen_vowels_array().get(j).unwrap();
            let kana = gen_hiragana(consonant, vowel).to_string() + "\n";
            let vowel_alph = vowel.to_string();
            let re =
                Regex::new(&(consonant_alph.to_lowercase() + "\t\t" + r"([ぁ-んー]*)")).unwrap();
            let consonant_last = consonant_alph
                .to_lowercase()
                .as_str()
                .chars()
                .last()
                .unwrap();

            let mut alphs = consonant_alph.clone().to_lowercase() + &vowel_alph.to_lowercase();

            for t in tokens.split("") {
                if consonant_last.to_string() == t && consonant_alph.len() >= 2 {
                    //println!("{}\t{}", consonant_alph_low, t);

                    let lhs = match re.captures(&sequences.to_lowercase()) {
                        Some(caps) => alphs.replace(&consonant_alph, &caps[1]),
                        None => "How".to_string(),
                    };

                    out.push_str(&(lhs + "	" + kana.as_str()));
                    alphs.remove_matches(&(alphs.clone() + "\t" + &kana.as_str()));
                    dup_flag = true
                }
            }
            if !dup_flag {
                out.push_str(&(alphs.clone() + "\t" + &kana.as_str()));
            }
        }
    }
    out.push("\n".chars().next().unwrap());

    out
}
