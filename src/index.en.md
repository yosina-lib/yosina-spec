---
lang: en
id: toppage
url: /
title: Yosina Transliterator Specification
---
# Yosina Transliterator Specification

## Introduction

Yosina is a transliteration library that specifically deals with the letters and symbols used in Japanese writing.  Japanese has a long history with its unique writing system which not only incorporates different kinds of characters, such as those from Chinese and English, but is also influenced by various writing systems, including German and French.  There also lie quite complicated consequences in the Japanese standards of coded character sets, which is still causing uncertanities even after the Unicode standard was deployed widely.

The name “Yosina” is taken from the old Japanese adverb “Yoshina-ni”, which means suitably, appropriately, or “as you think best”.  Developers tackling Japanese texts should have always wondered why that many variations exist for the same letter, and once wished there would be a thing that lets them forget all the gotchas.  Yosina was named in the hope it will be a way for the developers to better handle such texts.

Yosina can handle the following cases for instance:

* Transliteration from half-width katakana to full-width counterparts.

    <svg><use href="/assets/img/conversion-example1.svg#a" /></svg>

* Transliteration from half-width signs to full-width counterparts.

    <svg><use href="/assets/img/conversion-example2.svg#a" /></svg>

* Transliteration of visually-ambiguous characters; contextually replace hyphen-minuses between katakana/hiraganas with long-vowel marks.

    <svg><use href="/assets/img/conversion-example3.svg#a" /></svg>

* Transliteration of visually-ambiguous characters; contextually replace visually-ambiguous long-vowel marks with hyphen minuses.

    <svg><use href="/assets/img/conversion-example4.svg#a" /></svg>

* Transliteration from old-style glyphs (旧字体; kyu-ji-tai) to new-style counterparts (新字体; shin-ji-tai).

    <svg><use href="/assets/img/conversion-example5.svg#a" /></svg>

## Project Scope and Limitations

While it is known that similar problems exist in the other languages such as Korean, the library only handles characters that appear in Japanese writing for the time being.

The only coded character set (CCS) the library handles is Unicode.  It assumes CCSes defined in the JIS standards are backed by the Unicode character set of a certain version, but not for the opposite.

JIS X 0201 specifies control sequences to render alphabets with diacritics by combining an ordinary latin alphabets and one of particular symbols such as QUOTATION MARK and APOSTROPHE along with the control codes specified in JIS X 0211. However, such sequences will not be supported because of the reason above.

## Related Standards and Datasets

The following standards are adopted to constitute the Yosina specification.

* JIS X 0201:1997
* JIS X 0208:1997, JIS X 0208:2004
* JIS X 0213
* [Unicode 12.1](https://www.unicode.org/versions/Unicode12.1.0/)

The following publicly-available datasets are also employed.

* [Adobe-Japan1-7](https://github.com/adobe-type-tools/Adobe-Japan1/)
* [UniJIS-UTF32-H CMAP](https://github.com/adobe-type-tools/cmap-resources/)
* [UniJIS2004-UTF32-H CMAP](https://github.com/adobe-type-tools/cmap-resources/)
* [CVJKI Ideograph Database](https://kanji-database.sourceforge.net/) 

## Transliterators Defined in this Specification

* [CJK radicals transliterator](/en/transliterators/cjk-radicals/)
* [Ideographic annotations transliterator](/en/transliterators/ideographic-annotations/)
* [Mathematical alphanumeric symbols transliterator](/en/transliterators/mathematical-alphanumerics/)
* [Hyphens transliterator](/en/transliterators/hyphens/)
* [Traditional to new form kanji transliterator](/en/transliterators/traditional-to-new-kanjis/)
* [JIS X 0201 and alike transliterator](/en/transliterators/jisx0201-and-alike/)
* [Hiragana-katakana composition transliterator](/en/transliterators/hiragana-katakana-composition/)
* [Prolonged sound marks transliterator](/en/transliterators/prolonged-sound-marks/)
* [Combined characters transliterator](/en/transliterators/combined-chars/)
* [Circled or squared transliterator](/en/transliterators/circled-or-squared/)