---
lang: ja
id: toppage
url: /
title: Yosina翻字器仕様
---
# Yosina翻字器仕様

## はじめに

Yosinaは、日本語の文章で使われる文字や記号を専門的に扱う翻字 (transliteration) ライブラリです。日本語は独自の文字体系を持つ長い歴史があり、中国語や英語由来の文字を取り入れているだけでなく、ドイツ語やフランス語を含む様々な文字体系からの影響も受けています。また、日本の符号化文字集合規格は非常に複雑で、Unicode標準が広く普及した後でも、依然として不確実性を引き起こしています。

「Yosina」という名前は、古い日本語の副詞「よしなに」に由来し、「適切に」「適当に」「良きに計らう」という意味を持ちます。日本語のテキストを扱う開発者は、なぜ同じ文字にこれほど多くのバリエーションが存在するのかと常に疑問に思い、そのような落とし穴をすべて忘れさせてくれるものがあればと願ったことがあるでしょう。Yosinaは、開発者がそのようなテキストをより適切に扱えるツールとなることを願って名付けられました。

Yosinaは、例えば以下のようなケースを扱うことができます：

* 半角カタカナから全角カタカナへの翻字

    <svg><use href="/assets/img/conversion-example1.svg#a" /></svg>

* 半角記号から全角記号への翻字

    <svg><use href="/assets/img/conversion-example2.svg#a" /></svg>

* 視覚的に曖昧な文字の翻字：カタカナ・ひらがな間のハイフンマイナスを文脈に応じて長音記号に置換

    <svg><use href="/assets/img/conversion-example3.svg#a" /></svg>

* 視覚的に曖昧な文字の翻字：視覚的に曖昧な長音記号を文脈に応じてハイフンマイナスに置換

    <svg><use href="/assets/img/conversion-example4.svg#a" /></svg>

* 旧字体から新字体への翻字

    <svg><use href="/assets/img/conversion-example5.svg#a" /></svg>

## プロジェクトの範囲と制限事項

韓国語など他の言語にも同様の問題が存在することは知られていますが、当面このライブラリは日本語の文章に現れる文字のみを対象とします。

ライブラリが扱う符号化文字集合 (CCS)はUnicodeのみです。JIS規格で定義されたCCSは特定のバージョンのUnicode文字集合によってサポートされていると想定しますが、その逆は想定していません。

JIS X 0201は、通常のラテン文字とQUOTATION MARKやAPOSTROPHEなどの特定の記号を、JIS X 0211で規定された制御コードと組み合わせることで、ダイアクリティカルマーク付きアルファベットをレンダリングする制御シーケンスを規定しています。しかし、上記の理由により、このようなシーケンスはサポート対象外です。

## 関連規格とデータセット

Yosina仕様の策定には、以下の規格を採用しています。

* JIS X 0201:1997
* JIS X 0208:1997, JIS X 0208:2004
* JIS X 0213
* [Unicode 12.1](https://www.unicode.org/versions/Unicode12.1.0/)

以下の公開データセットも使用しています。

* [Adobe-Japan1-7](https://github.com/adobe-type-tools/Adobe-Japan1/)
* [UniJIS-UTF32-H CMAP](https://github.com/adobe-type-tools/cmap-resources/)
* [UniJIS2004-UTF32-H CMAP](https://github.com/adobe-type-tools/cmap-resources/)
* [CVJKI Ideograph Database](https://kanji-database.sourceforge.net/) 

## この仕様で定義される翻字器

* [CJK部首トランスリテレータ](/transliterators/cjk-radicals/)
* [表意文字注記トランスリテレータ](/transliterators/ideographic-annotations/)
* [数学用英数記号トランスリテレータ](/transliterators/mathematical-alphanumerics/)
* [ハイフントランスリテレータ](/transliterators/hyphens/)
* [旧字体から新字体への漢字トランスリテレータ](/transliterators/traditional-to-new-kanjis/)
* [JIS X 0201および類似文字トランスリテレータ](/transliterators/jisx0201-and-alike/)
* [ひらがな・カタカナ合成トランスリテレータ](/transliterators/hiragana-katakana-composition/)
* [長音記号トランスリテレータ](/transliterators/prolonged-sound-marks/)
* [結合文字トランスリテレータ](/transliterators/combined-chars/)
* [丸囲み・角囲みトランスリテレータ](/transliterators/circled-or-squared/)