---
lang: ja
id: hyphens
url: /transliterators/hyphens/
title: ハイフントランスリテレータ
---
# ハイフントランスリテレータ

ハイフントランスリテレータは、様々なハイフン類似文字や関連記号を、JIS X 0201またはJIS X 0208文字セットに含まれるより一般的な対応文字に正規化するために設計されています。これは、文字エンコーディングの互換性が重要な日本のコンピューティング環境でのテキスト処理に特に有用です。

## 概要

このトランスリテレータは以下のような幅広い文字を処理します：
- 各種ハイフンとダッシュ（ハイフン、エンダッシュ、エムダッシュなど）
- チルダと波ダッシュ文字
- 縦棒のバリエーション
- 特殊な句読点
- 通貨記号（セント記号、ポンド記号）
- カタカナ中黒と長音記号

## マッピングの優先順位

トランスリテレータは設定可能な優先順位で複数のマッピングモードをサポートしています。各マッピングモードは、下記の文字マッピング表の特定の列に対応します：

- **ascii**: 基本的なASCII文字（7ビット）にマッピング → 「ASCII置換コードポイント」および「ASCII置換文字」列
- **jisx0201**: JIS X 0201（8ビット）文字にマッピング → 「JIS X 0201置換コードポイント」および「JIS X 0201置換文字」列
- **jisx0208_90**: JIS X 0208:1990文字にマッピング（デフォルト） → 「JIS X 0208-1978置換コードポイント」および「JIS X 0208-1978置換文字」列
- **jisx0208_90_windows**: Windows固有のJIS X 0208:1990バリアントにマッピング → 「JIS X 0208-1978置換（Windows）」および「JIS X 0208-1978置換（Windows）文字」列
- **jisx0208_verbatim**: 可能な場合は元のJIS X 0208文字を保持 → 「JIS X 0208逐語的置換」および「JIS X 0208逐語的置換文字」列

トランスリテレータは `precedence` オプションを使用して、複数のマッピングを階層的に適用できます。文字を処理する際、優先順位配列で指定された順序で各マッピングをチェックし、その文字に対して最初に利用可能なマッピングが使用されます。これにより、フォールバックチェーンを作成できます。例えば、JIS X 0208マッピングを優先しつつ、必要に応じてJIS X 0201やASCIIにフォールバックすることができます。

デフォルトでは、トランスリテレータは `["jisx0208_90"]` を優先順位として使用します。これをカスタマイズして、`["jisx0208_90_windows", "jisx0201", "ascii"]` のような、複数のフォールバックオプションを提供するより複雑なマッピング戦略を作成できます。

## 文字マッピング表

以下の表は、このトランスリテレータが処理するすべての文字と、異なるモードでのマッピングを示しています：

| 元のコードポイント | 元の文字名 | 元の文字 | ASCII置換コードポイント | ASCII置換文字 | JIS X 0201置換コードポイント | JIS X 0201置換文字 | JIS X 0208-1978置換コードポイント | JIS X 0208-1978置換文字 | JIS X 0208-1978置換（Windows） | JIS X 0208-1978置換（Windows）文字 | JIS X 0208逐語的置換 | JIS X 0208逐語的置換文字 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U+002D | HYPHEN-MINUS | ![size=24](/assets/img/genglyphsvg/u002d.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  |  |  |
| U+007C | VERTICAL LINE | ![size=24](/assets/img/genglyphsvg/u007c.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  |  |  |
| U+007E | TILDE | ![size=24](/assets/img/genglyphsvg/u007e.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+FF5E | ![size=24](/assets/img/genglyphsvg/uff5e.svg)  |  |  |
| U+00A2 | CENT SIGN | ![size=24](/assets/img/genglyphsvg/u00a2.svg) |  |  |  |  | U+00A2 | ![size=24](/assets/img/genglyphsvg/u00a2.svg)  | U+FFE0 | ![size=24](/assets/img/genglyphsvg/uffe0.svg)  | U+00A2 | ![size=24](/assets/img/genglyphsvg/u00a2.svg) |
| U+00A3 | POUND SIGN | ![size=24](/assets/img/genglyphsvg/u00a3.svg) |  |  |  |  | U+00A3 | ![size=24](/assets/img/genglyphsvg/u00a3.svg)  | U+FFE1 | ![size=24](/assets/img/genglyphsvg/uffe1.svg)  | U+00A3 | ![size=24](/assets/img/genglyphsvg/u00a3.svg) |
| U+00A6 | BROKEN BAR | ![size=24](/assets/img/genglyphsvg/u00a6.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+00A6 | ![size=24](/assets/img/genglyphsvg/u00a6.svg) |
| U+02D7 | MODIFIER LETTER MINUS SIGN | ![size=24](/assets/img/genglyphsvg/u02d7.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  | U+FF0D | ![size=24](/assets/img/genglyphsvg/uff0d.svg)  |  |  |
| U+2010 | HYPHEN | ![size=24](/assets/img/genglyphsvg/u2010.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg) |
| U+2011 | NON-BREAKING HYPHEN | ![size=24](/assets/img/genglyphsvg/u2011.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  |  |  |
| U+2012 | FIGURE DASH | ![size=24](/assets/img/genglyphsvg/u2012.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  |  |  |
| U+2013 | EN DASH | ![size=24](/assets/img/genglyphsvg/u2013.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2013 | ![size=24](/assets/img/genglyphsvg/u2013.svg) |
| U+2014 | EM DASH | ![size=24](/assets/img/genglyphsvg/u2014.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2014 | ![size=24](/assets/img/genglyphsvg/u2014.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2014 | ![size=24](/assets/img/genglyphsvg/u2014.svg) |
| U+2015 | HORIZONTAL BAR | ![size=24](/assets/img/genglyphsvg/u2015.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg) |
| U+2016 | DOUBLE VERTICAL LINE | ![size=24](/assets/img/genglyphsvg/u2016.svg) |  |  |  |  | U+2016 | ![size=24](/assets/img/genglyphsvg/u2016.svg)  | U+2225 | ![size=24](/assets/img/genglyphsvg/u2225.svg)  | U+2016 | ![size=24](/assets/img/genglyphsvg/u2016.svg) |
| U+2032 | PRIME | ![size=24](/assets/img/genglyphsvg/u2032.svg) | U+0027 | ![size=24](/assets/img/genglyphsvg/u0027.svg)  | U+0027 | ![size=24](/assets/img/genglyphsvg/u0027.svg)  | U+2032 | ![size=24](/assets/img/genglyphsvg/u2032.svg)  | U+2032 | ![size=24](/assets/img/genglyphsvg/u2032.svg)  | U+2032 | ![size=24](/assets/img/genglyphsvg/u2032.svg) |
| U+2033 | DOUBLE PRIME | ![size=24](/assets/img/genglyphsvg/u2033.svg) | U+0022 | ![size=24](/assets/img/genglyphsvg/u0022.svg)  | U+0022 | ![size=24](/assets/img/genglyphsvg/u0022.svg)  | U+2033 | ![size=24](/assets/img/genglyphsvg/u2033.svg)  | U+2033 | ![size=24](/assets/img/genglyphsvg/u2033.svg)  | U+2033 | ![size=24](/assets/img/genglyphsvg/u2033.svg) |
| U+203E | OVERLINE | ![size=24](/assets/img/genglyphsvg/u203e.svg) |  |  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+FFE3 | ![size=24](/assets/img/genglyphsvg/uffe3.svg)  | U+FFE3 | ![size=24](/assets/img/genglyphsvg/uffe3.svg)  | U+203D | ![size=24](/assets/img/genglyphsvg/u203d.svg) |
| U+2043 | HYPHEN BULLET | ![size=24](/assets/img/genglyphsvg/u2043.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  |  |  |
| U+2053 | SWUNG DASH | ![size=24](/assets/img/genglyphsvg/u2053.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  |  |  |
| U+2212 | MINUS SIGN | ![size=24](/assets/img/genglyphsvg/u2212.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  | U+FF0D | ![size=24](/assets/img/genglyphsvg/uff0d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg) |
| U+2225 | PARALLEL TO | ![size=24](/assets/img/genglyphsvg/u2225.svg) |  |  |  |  | U+2016 | ![size=24](/assets/img/genglyphsvg/u2016.svg)  | U+2225 | ![size=24](/assets/img/genglyphsvg/u2225.svg)  | U+2225 | ![size=24](/assets/img/genglyphsvg/u2225.svg) |
| U+223C | TILDE OPERATOR | ![size=24](/assets/img/genglyphsvg/u223c.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+FF5E | ![size=24](/assets/img/genglyphsvg/uff5e.svg)  |  |  |
| U+223D | REVERSED TILDE | ![size=24](/assets/img/genglyphsvg/u223d.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+FF5E | ![size=24](/assets/img/genglyphsvg/uff5e.svg)  |  |  |
| U+2500 | BOX DRAWINGS LIGHT HORIZONTAL | ![size=24](/assets/img/genglyphsvg/u2500.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2500 | ![size=24](/assets/img/genglyphsvg/u2500.svg) |
| U+2501 | BOX DRAWINGS HEAVY HORIZONTAL | ![size=24](/assets/img/genglyphsvg/u2501.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg)  | U+2501 | ![size=24](/assets/img/genglyphsvg/u2501.svg) |
| U+2502 | BOX DRAWINGS LIGHT VERTICAL | ![size=24](/assets/img/genglyphsvg/u2502.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+2502 | ![size=24](/assets/img/genglyphsvg/u2502.svg) |
| U+2796 | HEAVY MINUS SIGN | ![size=24](/assets/img/genglyphsvg/u2796.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  | U+FF0D | ![size=24](/assets/img/genglyphsvg/uff0d.svg)  |  |  |
| U+29FF | MINY | ![size=24](/assets/img/genglyphsvg/u29ff.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+FF0D | ![size=24](/assets/img/genglyphsvg/uff0d.svg)  |  |  |
| U+2E3A | TWO-EM DASH | ![size=24](/assets/img/genglyphsvg/u2e3a.svg) | U+002D U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2014 U+2014 | ![size=24](/assets/img/genglyphsvg/u2014.svg) ![size=24](/assets/img/genglyphsvg/u2014.svg)  | U+2015 U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg) ![size=24](/assets/img/genglyphsvg/u2015.svg)  |  |  |
| U+2E3B | THREE-EM DASH | ![size=24](/assets/img/genglyphsvg/u2e3b.svg) | U+002D U+002D U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D U+002D U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg) ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2014 U+2014 U+2014 | ![size=24](/assets/img/genglyphsvg/u2014.svg) ![size=24](/assets/img/genglyphsvg/u2014.svg) ![size=24](/assets/img/genglyphsvg/u2014.svg)  | U+2015 U+2015 U+2015 | ![size=24](/assets/img/genglyphsvg/u2015.svg) ![size=24](/assets/img/genglyphsvg/u2015.svg) ![size=24](/assets/img/genglyphsvg/u2015.svg)  |  |  |
| U+301C | WAVE DASH | ![size=24](/assets/img/genglyphsvg/u301c.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+FF5E | ![size=24](/assets/img/genglyphsvg/uff5e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg) |
| U+30A0 | KATAKANA-HIRAGANA DOUBLE HYPHEN | ![size=24](/assets/img/genglyphsvg/u30a0.svg) | U+003D | ![size=24](/assets/img/genglyphsvg/u003d.svg)  | U+003D | ![size=24](/assets/img/genglyphsvg/u003d.svg)  | U+FF1D | ![size=24](/assets/img/genglyphsvg/uff1d.svg)  | U+FF1D | ![size=24](/assets/img/genglyphsvg/uff1d.svg)  | U+30A0 | ![size=24](/assets/img/genglyphsvg/u30a0.svg) |
| U+30FB | KATAKANA MIDDLE DOT | ![size=24](/assets/img/genglyphsvg/u30fb.svg) |  |  | U+FF65 | ![size=24](/assets/img/genglyphsvg/uff65.svg)  | U+30FB | ![size=24](/assets/img/genglyphsvg/u30fb.svg)  | U+30FB | ![size=24](/assets/img/genglyphsvg/u30fb.svg)  | U+30FB | ![size=24](/assets/img/genglyphsvg/u30fb.svg) |
| U+30FC | KATAKANA-HIRAGANA PROLONGED SOUND MARK | ![size=24](/assets/img/genglyphsvg/u30fc.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+30FC | ![size=24](/assets/img/genglyphsvg/u30fc.svg)  | U+30FC | ![size=24](/assets/img/genglyphsvg/u30fc.svg)  | U+30FC | ![size=24](/assets/img/genglyphsvg/u30fc.svg) |
| U+FE31 | PRESENTATION FORM FOR VERTICAL EM DASH | ![size=24](/assets/img/genglyphsvg/ufe31.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  |  |  |
| U+FE58 | SMALL EM DASH | ![size=24](/assets/img/genglyphsvg/ufe58.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  |  |  |
| U+FE63 | SMALL HYPHEN-MINUS | ![size=24](/assets/img/genglyphsvg/ufe63.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  | U+2010 | ![size=24](/assets/img/genglyphsvg/u2010.svg)  |  |  |
| U+FF0D | FULLWIDTH HYPHEN-MINUS | ![size=24](/assets/img/genglyphsvg/uff0d.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+2212 | ![size=24](/assets/img/genglyphsvg/u2212.svg)  | U+FF0D | ![size=24](/assets/img/genglyphsvg/uff0d.svg)  |  |  |
| U+FF5C | FULLWIDTH VERTICAL LINE | ![size=24](/assets/img/genglyphsvg/uff5c.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg) |
| U+FF5E | FULLWIDTH TILDE | ![size=24](/assets/img/genglyphsvg/uff5e.svg) | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+007E | ![size=24](/assets/img/genglyphsvg/u007e.svg)  | U+301C | ![size=24](/assets/img/genglyphsvg/u301c.svg)  | U+FF5E | ![size=24](/assets/img/genglyphsvg/uff5e.svg)  |  |  |
| U+FFE4 | FULLWIDTH BROKEN BAR | ![size=24](/assets/img/genglyphsvg/uffe4.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FFE4 | ![size=24](/assets/img/genglyphsvg/uffe4.svg)  | U+FFE4 | ![size=24](/assets/img/genglyphsvg/uffe4.svg) |
| U+FF70 | HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK | ![size=24](/assets/img/genglyphsvg/uff70.svg) | U+002D | ![size=24](/assets/img/genglyphsvg/u002d.svg)  | U+FF70 | ![size=24](/assets/img/genglyphsvg/uff70.svg)  | U+30FC | ![size=24](/assets/img/genglyphsvg/u30fc.svg)  | U+30FC | ![size=24](/assets/img/genglyphsvg/u30fc.svg)  |  |  |
| U+FFE8 | HALFWIDTH FORMS LIGHT VERTICAL | ![size=24](/assets/img/genglyphsvg/uffe8.svg) | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+007C | ![size=24](/assets/img/genglyphsvg/u007c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  | U+FF5C | ![size=24](/assets/img/genglyphsvg/uff5c.svg)  |  |  |
