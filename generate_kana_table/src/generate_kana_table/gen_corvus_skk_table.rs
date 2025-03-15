use kanaria::string::UCSStr;
use kanaria::utils::ConvertTarget;
use regex::Regex;

use super::azik_config::{gen_corvus_skk_sequence, AzikConfig};
use super::gen_consonant::gen_consonants_array;
use super::gen_kana::{gen_half_katakana, gen_hiragana, gen_katakana};
use super::gen_vowel::gen_vowels_array;

pub fn gen_corvuss_skk_table(config: AzikConfig) -> String {
    let mut out = String::new();
    // Azikの対象外を事前に作成
    out.push_str(
        "la	ぁ	ァ	ｧ	0
li	ぃ	ィ	ｨ	0
lu	ぅ	ゥ	ｩ	0
le	ぇ	ェ	ｪ	0
lo	ぉ	ォ	ｫ	0
lya	ゃ	ャ	ｬ	0
lyu	ゅ	ュ	ｭ	0
lyo	ょ	ョ	ｮ	0
0	0	0	0	0
1	1	1	1	0
2	2	2	2	0
3	3	3	3	0
4	4	4	4	0
5	5	5	5	0
6	6	6	6	0
7	7	7	7	0
8	8	8	8	0
9	9	9	9	0
?	？	？	?	0
!	！	！	!	0
\"	\"	\"	\"	0
#	#	#	#	0
$	$	$	$	0
%	%	%	%	0
&	&	&	&	0
'	'	'	'	0
(	（	（	（	0
)	）	）	）	0
*	*	*	*	0
+	+	+	+	0
,	、	、	、	0
-	ー	ー	ー	0
.	。	。	。	0
/	/	/	/	0
:	:	:	:	0
;	;	;	;	0
<	<	<	<	0
=	=	=	=	0
>	>	>	>	0
 	 	 	 	0
@	@	@	@	0
[	「	「	「	0
\\	\\	\\	\\	0
]	」	」	」	0
^	^	^	^	0
_	_	_	_	0
`	`	`	`	0
{	{	{	{	0
|	|	|	|	0
}	}	}	}	0
~	~	~	~	0
z<	‥	‥	‥	0
z>	…	…	…	0
z~	〜	〜	〜	0
z/	・	・	・	0
z(	(	(	(	0
z)	)	)	)	0
z[	『	『	『	0
z]	』	』	』	0
z{	【	【	【	0
z}	】	】	】	0
zK	←	←	←	0
zT	↓	↓	↓	0
zN	↑	↑	↑	0
zS	→	→	→	0

a	あ	ア	ｱ	0
i	い	イ	ｲ	0
u	う	ウ	ｳ	0
e	え	エ	ｴ	0
o	お	オ	ｵ	0",
    );
    let gen_seq = gen_corvus_skk_sequence(config);
    let sequences = gen_seq.0;
    let tokens = gen_seq.1;
    out.push_str(sequences.as_str());

    for i in 0..39 {
        let consonant = *gen_consonants_array().get(i).unwrap();
        let consonant_alph = consonant.to_string().to_lowercase();
        for j in 0..5 {
            let mut dup_flag = false;
            let vowel = *gen_vowels_array().get(j).unwrap();
            let hiragana = gen_hiragana(consonant, vowel).to_string();
            let katakana = gen_katakana(consonant, vowel).to_string();
            let half_katakana = gen_half_katakana(consonant, vowel).to_string();
            let vowel_alph = vowel.to_string();
            let re = Regex::new(
                &(consonant_alph.to_lowercase()
                    + "\t"
                    + r"([ぁ-んー]*)"
                    + "\t"
                    + r"([ァ-ヴ]*)"
                    + "\t"
                    + r"([ｦ-ﾟ]*)"
                    + "\t([0-9]*)"),
            )
            .unwrap();
            let consonant_last = consonant_alph
                .to_lowercase()
                .as_str()
                .chars()
                .last()
                .unwrap();

            let mut alphs = consonant_alph.clone().to_lowercase() + &vowel_alph.to_lowercase();

            for t in tokens.split("") {
                if consonant_last.to_string() == t && consonant_alph.len() >= 2 {
                    let lhs = match re.captures(&sequences.to_lowercase()) {
                        Some(caps) => {
                            out.push_str(
                                &(UCSStr::from_str(&caps[1]).katakana().to_string()
                                    + &vowel_alph.to_lowercase()
                                    + "\t"
                                    + hiragana.as_str()
                                    + "\t"
                                    + katakana.as_str()
                                    + "\t"
                                    + &half_katakana
                                    + "\t0\n"),
                            );
                            out.push_str(
                                &(UCSStr::from_str(&caps[1])
                                    .katakana()
                                    .narrow(ConvertTarget::all())
                                    .to_string()
                                    + &vowel_alph.to_lowercase()
                                    + "\t"
                                    + hiragana.as_str()
                                    + "\t"
                                    + katakana.as_str()
                                    + "\t"
                                    + &half_katakana
                                    + "\t0\n"),
                            );
                            alphs.replace(&consonant_alph, &caps[1])
                        }
                        None => "How".to_string(),
                    };

                    out.push_str(
                        &(lhs
                            + "	"
                            + hiragana.as_str()
                            + "\t"
                            + katakana.as_str()
                            + "\t"
                            + half_katakana.as_str()
                            + "\t0\n"),
                    );
                    alphs.remove_matches(
                        &(alphs.clone()
                            + "\t"
                            + &hiragana.as_str()
                            + "\t"
                            + &katakana.as_str()
                            + "\t"
                            + &half_katakana.as_str()
                            + "\t0\n"),
                    );
                    dup_flag = true
                }
            }
            if consonant_alph == "v" {
                let lhs = match re.captures(&sequences.to_lowercase()) {
                    Some(caps) => alphs.replace(&consonant_alph, &caps[1]),
                    None => "How".to_string(),
                };

                out.push_str(
                    &(lhs
                        + "	"
                        + hiragana.as_str()
                        + "\t"
                        + katakana.as_str()
                        + "\t"
                        + half_katakana.as_str()
                        + "\t0\n"),
                );
                alphs.remove_matches(&(alphs.clone() + "\t" + &hiragana.as_str()));
                dup_flag = true
            }

            if !dup_flag {
                out.push_str(
                    &(alphs.clone()
                        + "\t"
                        + &hiragana
                        + "\t"
                        + &katakana
                        + "\t"
                        + &half_katakana
                        + "\t0\n"),
                );
            }
        }
    }
    out.push("\n".chars().next().unwrap());

    out
}
