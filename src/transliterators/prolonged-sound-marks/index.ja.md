# 長音記号トランスリテレータ

このトランスリテレータは、様々なダッシュおよび長音記号文字を、先行する文字のスクリプトタイプとコンテキストに基づいて適切な長音記号に正規化します。

## 概要

このトランスリテレータは、様々なハイフンおよびダッシュ文字 (U+002D、U+2010、U+2014、U+2015、U+2212、U+FF0D、U+FF70、U+30FC)を処理し、先行する文字に応じて適切な長音記号に変換します。

## アルゴリズム

1. **文字分類**：各文字はスクリプトタイプと特殊プロパティによって分類されます
2. **先読み処理**：英数字置換が有効な場合、トランスリテレータは適切な置換文字を決定するために先読みを行います
3. **文脈依存変換**：ダッシュ文字は先行する文字のタイプに基づいて変換されます：
   - 母音で終わる仮名の後 → 適切な長音記号
   - 促音・撥音の後 → オプションに基づく条件付き変換
   - 英数字間 → 条件付きハイフン置換
4. **スクリプトの一貫性**：半角文字は半角長音記号を使用し、全角文字は全角長音記号を使用します

## 文字分類

トランスリテレータは文字を以下のタイプに分類します：

- **ひらがな**：ひらがなスクリプトの文字 (U+3041-U+309C、U+309F)
- **カタカナ**：カタカナスクリプトの文字 (U+30A1-U+30FA、U+30FD-U+30FF)
- **半角カタカナ**：半角カタカナ範囲の文字 (U+FF66-U+FF6F、U+FF71-U+FF9F)
- **アルファベット**：ASCIIおよび全角英字
- **数字**：ASCIIおよび全角数字
- **特殊文字**：特別な処理を持つ特定の文字：
  - U+FF70：半角カタカナ長音記号
  - U+30FC：カタカナひらがな長音記号
  - U+3063：ひらがな小さいつ (促音)
  - U+3093：ひらがなん (撥音)
  - U+30C3：カタカナ小さいツ (促音)
  - U+30F3：カタカナン (撥音)
  - U+FF6F：半角カタカナ小さいツ
  - U+FF9D：半角カタカナン

### 例

| 入力 | 出力 | 説明 |
| --- | --- | --- |
| `イ-ハト-ヴォ` | `イーハトーヴォ` | カタカナの後のハイフンはカタカナ長音記号になります |
| `カトラリ-` | `カトラリー` | カタカナの後のハイフンはカタカナ長音記号になります |
| `イ－ハト－ヴォ` | `イーハトーヴォ` | 全角ハイフンは長音記号に変換されます |

## 設定オプション

### `allowProlongedSokuon`

有効にすると、促音 (っ/ッ)文字の後に長音記号を許可します。

| 入力 | デフォルト出力 | `allowProlongedSokuon`有効時 |
| --- | --- | --- |
| `ウッ－` | `ウッ－` (変更なし) | `ウッー` |

### `allowProlongedHatsuon`

有効にすると、撥音 (ん/ン)文字の後に長音記号を許可します。

| 入力 | デフォルト出力 | `allowProlongedHatsuon`有効時 |
| --- | --- | --- |
| `ウン－` | `ウン－` (変更なし) | `ウンー` |

### `replaceProlongedMarksBetweenAlnums`

有効にすると、英数字間の長音記号を適切なハイフンに置換します。

| 入力 | デフォルト出力 | `replaceProlongedMarksBetweenAlnums`有効時 |
| --- | --- | --- |
| `1ー－2ー3` | `1ー－2ー3` (変更なし) | `1--2-3` |
| `１ー－２ー３` | `１ー－２ー３` (変更なし) | `１－－２－３` |

### `skipAlreadyTransliteratedChars`

有効にすると、すでに字訳された文字 (`source`プロパティを持つ)をスキップします。

## 入力ダッシュ文字

以下の文字が変換対象のダッシュ/ハイフン文字として認識されます：

| 文字 | Unicode | 名前 |
| --- | --- | --- |
| `-` | U+002D | ハイフンマイナス |
| `‐` | U+2010 | ハイフン |
| `—` | U+2014 | emダッシュ |
| `―` | U+2015 | 水平バー |
| `−` | U+2212 | マイナス記号 |
| `－` | U+FF0D | 全角ハイフンマイナス |
| `ｰ` | U+FF70 | 半角カタカナ長音記号 |
| `ー` | U+30FC | カタカナひらがな長音記号 |

## 出力長音記号

| 文字 | Unicode | 使用法 |
| --- | --- | --- |
| `ー` | U+30FC | ひらがなおよび全角カタカナの後で使用 |
| `ｰ` | U+FF70 | 半角カタカナの後で使用 |
| `-` | U+002D | 半角英数字間で使用 (オプション有効時) |
| `－` | U+FF0D | 全角英数字間で使用 (オプション有効時) |