---
lang: ja
id: historical-hirakatas
url: /transliterators/historical-hirakatas/
title: 歴史的仮名トランスリテレータ
---
# 歴史的仮名トランスリテレータ

このトランスリテレータは、歴史的仮名文字を現代の対応する仮名文字に変換します。ゐ（ヰ）やゑ（ヱ）などの歴史的仮名は現代日本語ではほとんど使用されず、テキスト正規化においては通常、現代の対応する文字に置き換えられます。

## 概要

このトランスリテレータは以下のカテゴリの歴史的仮名を処理します。各カテゴリはそれぞれ独立したオプションで制御されます：

- **歴史的ひらがな**：ゐ（U+3090）およびゑ（U+3091）。[`convertHistoricalHiragana`](#converthistoricalhiragana)で制御します。
- **歴史的カタカナ**：ヰ（U+30F0）およびヱ（U+30F1）。[`convertHistoricalKatakana`](#converthistoricalkatakana)で制御します。
- **歴史的カタカナの濁音形**：ヷ（U+30F7）、ヸ（U+30F8）、ヹ（U+30F9）、ヺ（U+30FA）。これらの文字は現代カタカナに単一文字の対応がありません。[`convertVoicedHistoricalKana`](#convertvoicedhistoricalkana)で制御します。

すべてのオプションは独立しており、自由に組み合わせることができます。

## オプション

### `convertHistoricalHiragana`

歴史的ひらがなゐ・ゑの変換方法を制御します。デフォルトは`"simple"`です。

- `"simple"` — 現代ひらがなの単一文字に変換します。
- `"decompose"` — 元の音韻をより忠実に反映した複数文字の現代ひらがな表現に変換します。
- `"skip"` — これらの文字を変換せずそのまま残します。

#### マッピング

| 変換元コードポイント | 変換元グリフ | `"simple"`（デフォルト） | `"decompose"` |
| --- | --- | --- | --- |
| U+3090 | ゐ | い（U+3044） | うぃ（U+3046 U+3043） |
| U+3091 | ゑ | え（U+3048） | うぇ（U+3046 U+3047） |

### `convertHistoricalKatakana`

歴史的カタカナヰ・ヱの変換方法を制御します。デフォルトは`"simple"`です。

- `"simple"` — 現代カタカナの単一文字に変換します。
- `"decompose"` — 元の音韻をより忠実に反映した複数文字の現代カタカナ表現に変換します。
- `"skip"` — これらの文字を変換せずそのまま残します。

#### マッピング

| 変換元コードポイント | 変換元グリフ | `"simple"`（デフォルト） | `"decompose"` |
| --- | --- | --- | --- |
| U+30F0 | ヰ | イ（U+30A4） | ウィ（U+30A6 U+30A3） |
| U+30F1 | ヱ | エ（U+30A8） | ウェ（U+30A6 U+30A7） |

### `convertVoicedHistoricalKana`

歴史的カタカナの濁音形（ヷ、ヸ、ヹ、ヺ）の処理方法を制御します。これらの文字は歴史的に「ゔぁ」「ゔぃ」「ゔぇ」「ゔぉ」の音を表すために使用されていましたが、現代カタカナには単一文字の対応がありません。デフォルトは`"skip"`です。

- `"decompose"` — ヴ（U+30F4、カタカナ ヴ）に適切な小書き母音カタカナを続ける複数文字表現に変換します。
- `"skip"` — これらの文字を変換せずそのまま残します。

#### `convertVoicedHistoricalKana`が`"decompose"`の場合のマッピング

| 変換元コードポイント | 変換元グリフ | 変換先コードポイント | 変換先グリフ |
| --- | --- | --- | --- |
| U+30F7 | ヷ | U+30F4 U+30A1 | ヴァ |
| U+30F8 | ヸ | U+30F4 U+30A3 | ヴィ |
| U+30F9 | ヹ | U+30F4 U+30A7 | ヴェ |
| U+30FA | ヺ | U+30F4 U+30A9 | ヴォ |

## 変換例

以下の例は、各オプションの組み合わせによる動作を示しています。特に記載がない限り、オプションはデフォルト値（`convertHistoricalHiragana: "simple"`、`convertHistoricalKatakana: "simple"`、`convertVoicedHistoricalKana: "skip"`）です。

### デフォルト動作（すべてデフォルト値）

| 入力 | 出力 |
| --- | --- |
| `ゐた` | `いた` |
| `ゑがく` | `えがく` |
| `ヰスキー` | `イスキー` |
| `ヱビス` | `エビス` |
| `ヷイオリン` | `ヷイオリン`（変換なし） |

### `convertHistoricalHiragana: "decompose"`の場合

| 入力 | 出力 |
| --- | --- |
| `ゐた` | `うぃた` |
| `ゑがく` | `うぇがく` |
| `ヰスキー` | `イスキー`（カタカナは影響なし） |

### `convertHistoricalKatakana: "decompose"`の場合

| 入力 | 出力 |
| --- | --- |
| `ヰスキー` | `ウィスキー` |
| `ヱビス` | `ウェビス` |
| `ゐた` | `いた`（ひらがなは影響なし） |

### `convertVoicedHistoricalKana: "decompose"`の場合

| 入力 | 出力 |
| --- | --- |
| `ヷイオリン` | `ヴァイオリン` |
| `ヸオラ` | `ヴィオラ` |
| `ヹネチア` | `ヴェネチア` |
| `ヺーカル` | `ヴォーカル` |

### 組み合わせ：`convertHistoricalKatakana: "decompose"`かつ`convertVoicedHistoricalKana: "decompose"`

| 入力 | 出力 |
| --- | --- |
| `ヰスキー` | `ウィスキー` |
| `ヱビス` | `ウェビス` |
| `ヷイオリン` | `ヴァイオリン` |

### `"skip"`オプションの場合

| 入力 | `convertHistoricalHiragana: "skip"` | `convertHistoricalKatakana: "skip"` |
| --- | --- | --- |
| `ゐた` | `ゐた`（変換なし） | `いた` |
| `ヰスキー` | `イスキー` | `ヰスキー`（変換なし） |
