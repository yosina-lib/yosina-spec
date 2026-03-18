---
lang: en
id: historical-hirakatas
url: /transliterators/historical-hirakatas/
title: Historical Hiragana/Katakana Transliterator
---
# Historical Hiragana/Katakana Transliterator

This transliterator converts historical hiragana and katakana characters into their modern equivalents. Historical kana such as WI (ゐ/ヰ) and WE (ゑ/ヱ) are rarely used in contemporary Japanese and are typically replaced with their modern counterparts in text normalization.

## Overview

The transliterator handles the following categories of historical kana, each controlled by a separate option:

- **Historical hiragana**: ゐ (U+3090) and ゑ (U+3091). Controlled by [`hiraganas`](#hiraganas).
- **Historical katakana**: ヰ (U+30F0) and ヱ (U+30F1). Controlled by [`katakanas`](#katakanas).
- **Voiced historical katakana**: ヷ (U+30F7), ヸ (U+30F8), ヹ (U+30F9), and ヺ (U+30FA). These characters do not have single-character counterparts in modern katakana. Controlled by [`voicedKatakanas`](#voicedkatakanas).

All options are independent and can be combined freely.

## Options

### `hiraganas`

Controls how historical hiragana WI (ゐ) and WE (ゑ) are converted. The default is `"simple"`.

- `"simple"` — Convert to single-character modern hiragana equivalents.
- `"decompose"` — Convert to multi-character modern hiragana representations that more closely reflect the original phonetics.
- `"skip"` — Leave these characters as-is without conversion.

#### Mappings

| Original codepoint | Original glyph | `"simple"` (default) | `"decompose"` |
| --- | --- | --- | --- |
| U+3090 | ゐ | い (U+3044) | うぃ (U+3046 U+3043) |
| U+3091 | ゑ | え (U+3048) | うぇ (U+3046 U+3047) |

### `katakanas`

Controls how historical katakana WI (ヰ) and WE (ヱ) are converted. The default is `"simple"`.

- `"simple"` — Convert to single-character modern katakana equivalents.
- `"decompose"` — Convert to multi-character modern katakana representations that more closely reflect the original phonetics.
- `"skip"` — Leave these characters as-is without conversion.

#### Mappings

| Original codepoint | Original glyph | `"simple"` (default) | `"decompose"` |
| --- | --- | --- | --- |
| U+30F0 | ヰ | イ (U+30A4) | ウィ (U+30A6 U+30A3) |
| U+30F1 | ヱ | エ (U+30A8) | ウェ (U+30A6 U+30A7) |

### `voicedKatakanas`

Controls how voiced historical katakana characters (ヷ, ヸ, ヹ, ヺ) are handled. These characters were historically used to represent "va", "vi", "ve", "vo" sounds, but they have no single-character modern katakana equivalents. The default is `"skip"`.

- `"decompose"` — Convert to multi-character representations using ヴ (U+30F4, KATAKANA VU) followed by the appropriate small vowel kana.
- `"skip"` — Leave these characters as-is without conversion.

#### Mappings when `voicedKatakanas` is `"decompose"`

| Original codepoint | Original glyph | Substitute codepoints | Substitute glyphs |
| --- | --- | --- | --- |
| U+30F7 | ヷ | U+30F4 U+30A1 | ヴァ |
| U+30F8 | ヸ | U+30F4 U+30A3 | ヴィ |
| U+30F9 | ヹ | U+30F4 U+30A7 | ヴェ |
| U+30FA | ヺ | U+30F4 U+30A9 | ヴォ |

## Transformation Examples

The following examples illustrate the behavior under various combinations of options. Unless otherwise specified, options are set to their default values (`hiraganas: "simple"`, `katakanas: "simple"`, `voicedKatakanas: "skip"`).

### Default behavior (all defaults)

| Input | Output |
| --- | --- |
| `ゐた` | `いた` |
| `ゑがく` | `えがく` |
| `ヰスキー` | `イスキー` |
| `ヱビス` | `エビス` |
| `ヷイオリン` | `ヷイオリン` (unchanged) |

### With `hiraganas: "decompose"`

| Input | Output |
| --- | --- |
| `ゐた` | `うぃた` |
| `ゑがく` | `うぇがく` |
| `ヰスキー` | `イスキー` (katakana unaffected) |

### With `katakanas: "decompose"`

| Input | Output |
| --- | --- |
| `ヰスキー` | `ウィスキー` |
| `ヱビス` | `ウェビス` |
| `ゐた` | `いた` (hiragana unaffected) |

### With `voicedKatakanas: "decompose"`

| Input | Output |
| --- | --- |
| `ヷイオリン` | `ヴァイオリン` |
| `ヸオラ` | `ヴィオラ` |
| `ヹネチア` | `ヴェネチア` |
| `ヺーカル` | `ヴォーカル` |

### Combined: `katakanas: "decompose"` and `voicedKatakanas: "decompose"`

| Input | Output |
| --- | --- |
| `ヰスキー` | `ウィスキー` |
| `ヱビス` | `ウェビス` |
| `ヷイオリン` | `ヴァイオリン` |

### With `"skip"` options

| Input | `hiraganas: "skip"` | `katakanas: "skip"` |
| --- | --- | --- |
| `ゐた` | `ゐた` (unchanged) | `いた` |
| `ヰスキー` | `イスキー` | `ヰスキー` (unchanged) |
