---
lang: en
id: small-hirakatas
url: /transliterators/small-hirakatas/
title: Small Hiragana/Katakana Transliterator
---
# Small Hiragana/Katakana Transliterator

This transliterator converts small (subscript-sized) hiragana and katakana characters into their ordinary-sized equivalents. Small kana are used in Japanese to represent specific phonetic nuances, but for text normalization and search purposes, it is often desirable to map them to their standard counterparts.

## Overview

The transliterator handles small forms across multiple Unicode blocks:

- **Small hiragana** (U+3041, U+3043, U+3045, U+3047, U+3049, U+3063, U+3083, U+3085, U+3087, U+308E, U+3095, U+3096): Small vowels (ぁぃぅぇぉ), small tsu (っ), small ya/yu/yo (ゃゅょ), small wa (ゎ), and small ka/ke (ゕゖ).
- **Small katakana** (U+30A1, U+30A3, U+30A5, U+30A7, U+30A9, U+30C3, U+30E3, U+30E5, U+30E7, U+30EE, U+30F5, U+30F6): The katakana equivalents of the above.
- **Small katakana extensions** (U+31F0–U+31FF): Additional small katakana for Ainu language support.
- **Halfwidth small katakana** (U+FF67–U+FF6F): Halfwidth forms of small katakana.
- **Kana extended blocks** (U+1B132, U+1B150–U+1B152, U+1B155, U+1B164–U+1B167): Small kana from the Kana Extended-A and Kana Extended-B blocks.

## Character Mapping Table

| Original codepoint | Original glyph | Substitute codepoint | Substitute glyph |
| --- | --- | --- | --- |
| U+3041 | ぁ | U+3042 | あ |
| U+3043 | ぃ | U+3044 | い |
| U+3045 | ぅ | U+3046 | う |
| U+3047 | ぇ | U+3048 | え |
| U+3049 | ぉ | U+304A | お |
| U+3063 | っ | U+3064 | つ |
| U+3083 | ゃ | U+3084 | や |
| U+3085 | ゅ | U+3086 | ゆ |
| U+3087 | ょ | U+3088 | よ |
| U+308E | ゎ | U+308F | わ |
| U+3095 | ゕ | U+304B | か |
| U+3096 | ゖ | U+3051 | け |
| U+30A1 | ァ | U+30A2 | ア |
| U+30A3 | ィ | U+30A4 | イ |
| U+30A5 | ゥ | U+30A6 | ウ |
| U+30A7 | ェ | U+30A8 | エ |
| U+30A9 | ォ | U+30AA | オ |
| U+30C3 | ッ | U+30C4 | ツ |
| U+30E3 | ャ | U+30E4 | ヤ |
| U+30E5 | ュ | U+30E6 | ユ |
| U+30E7 | ョ | U+30E8 | ヨ |
| U+30EE | ヮ | U+30EF | ワ |
| U+30F5 | ヵ | U+30AB | カ |
| U+30F6 | ヶ | U+30B1 | ケ |
| U+31F0 | ㇰ | U+30AF | ク |
| U+31F1 | ㇱ | U+30B7 | シ |
| U+31F2 | ㇲ | U+30B9 | ス |
| U+31F3 | ㇳ | U+30C8 | ト |
| U+31F4 | ㇴ | U+30CC | ヌ |
| U+31F5 | ㇵ | U+30CF | ハ |
| U+31F6 | ㇶ | U+30D2 | ヒ |
| U+31F7 | ㇷ | U+30D5 | フ |
| U+31F8 | ㇸ | U+30D8 | ヘ |
| U+31F9 | ㇹ | U+30DB | ホ |
| U+31FA | ㇺ | U+30E0 | ム |
| U+31FB | ㇻ | U+30E9 | ラ |
| U+31FC | ㇼ | U+30EA | リ |
| U+31FD | ㇽ | U+30EB | ル |
| U+31FE | ㇾ | U+30EC | レ |
| U+31FF | ㇿ | U+30ED | ロ |
| U+FF67 | ｧ | U+FF71 | ｱ |
| U+FF68 | ｨ | U+FF72 | ｲ |
| U+FF69 | ｩ | U+FF73 | ｳ |
| U+FF6A | ｪ | U+FF74 | ｴ |
| U+FF6B | ｫ | U+FF75 | ｵ |
| U+FF6C | ｬ | U+FF94 | ﾔ |
| U+FF6D | ｭ | U+FF95 | ﾕ |
| U+FF6E | ｮ | U+FF96 | ﾖ |
| U+FF6F | ｯ | U+FF82 | ﾂ |
| U+1B132 | 𛄲 | U+3053 | こ |
| U+1B150 | 𛅐 | U+3090 | ゐ |
| U+1B151 | 𛅑 | U+3091 | ゑ |
| U+1B152 | 𛅒 | U+3092 | を |
| U+1B155 | 𛅕 | U+30B3 | コ |
| U+1B164 | 𛅤 | U+30F0 | ヰ |
| U+1B165 | 𛅥 | U+30F1 | ヱ |
| U+1B166 | 𛅦 | U+30F2 | ヲ |
| U+1B167 | 𛅧 | U+30F3 | ン |

