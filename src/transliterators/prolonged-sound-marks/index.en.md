---
lang: en
id: prolonged-sound-marks
url: /transliterators/prolonged-sound-marks/
title: Prolonged Sound Marks Transliterator
---
# Prolonged Sound Marks Transliterator

This transliterator normalizes various dash and prolonged sound mark characters into appropriate prolonged sound marks based on the preceding character's script type and context.

## Overview

The transliterator processes various hyphen and dash characters (U+002D, U+2010, U+2014, U+2015, U+2212, U+FF0D, U+FF70, U+30FC) and converts them to appropriate prolonged sound marks depending on the preceding character.

## Algorithm

1. **Character Classification**: Each character is classified by script type and special properties
2. **Lookahead Processing**: The transliterator looks ahead to determine the appropriate replacement character
3. **Context-Sensitive Conversion**: Dash characters are converted based on the preceding character's type:
   - After vowel-ended kana → appropriate prolonged sound mark
   - After sokuon/hatsuon → conditional conversion based on options
   - Between alphanumerics → conditional hyphen replacement
4. **Script Consistency**: Halfwidth characters use halfwidth prolonged marks, fullwidth characters use fullwidth prolonged marks

## Character Classification

The transliterator classifies characters into the following types:

- **Hiragana**: Characters in the hiragana script (U+3041-U+309C, U+309F)
- **Katakana**: Characters in the katakana script (U+30A1-U+30FA, U+30FD-U+30FF)
- **Halfwidth Katakana**: Characters in the halfwidth katakana range (U+FF66-U+FF6F, U+FF71-U+FF9F)
- **Alphabet**: ASCII and fullwidth alphabetic characters
- **Digits**: ASCII and fullwidth numeric characters
- **Special Characters**: Specific characters with special handling:
  - U+FF70: Halfwidth katakana prolonged sound mark
  - U+30FC: Katakana-hiragana prolonged sound mark
  - U+3063: Hiragana small tsu (sokuon)
  - U+3093: Hiragana n (hatsuon)
  - U+30C3: Katakana small tsu (sokuon)
  - U+30F3: Katakana n (hatsuon)
  - U+FF6F: Halfwidth katakana small tsu
  - U+FF9D: Halfwidth katakana n

### Transformation Examples

#### Basic Transformation Examples

| Input | Output | Logic Explained |
| --- | --- | --- |
| `イ-ハト-ヴォ` | `イーハトーヴォ` | `イ`(KATAKANA \| VOWEL_ENDED) → `-` converted to U+30FC |
| `カトラリ-` | `カトラリー` | `リ`(KATAKANA \| VOWEL_ENDED) → `-` converted to U+30FC |
| `イ－ハト－ヴォ` | `イーハトーヴォ` | Fullwidth hyphen (U+FF0D) processed similarly |
| `ｱｲｳ-` | `ｱｲｳｰ` | `ｳ`(KATAKANA \| VOWEL_ENDED \| HALFWIDTH) → `-` converted to U+FF70 |

#### Special Option Examples

**Hatsuon/Sokuon Processing (with options enabled):**
| Input | Default | With Special Options | Logics Explained |
| --- | --- | --- | --- |
| `ウッ-` | `ウッ-` | `ウッー` | `ッ`(SOKUON) becomes prolongable with `allow_prolonged_sokuon` |
| `ウン-` | `ウン-` | `ウンー` | `ン`(HATSUON) becomes prolongable with `allow_prolonged_hatsuon` |
| `ｳﾝ-` | `ｳﾝ-` | `ｳﾝｰ` | Halfwidth hatsuon, same logic but uses U+FF70 |

**Alphanumeric Context Processing:**
| Input | Default | With `replaceProlongedMarksFollowingAlnums` | Logics Explained |
| --- | --- | --- | --- |
| `A-B-アイウ-123-` | `A-B-アイウー123-` | `A-B-アイウー123-` | Replace after alphanumeric, prolonged after Japanese |
| `１ー２ー３` | `１ー２ー３` | `１－２－３` | After fullwidth digits, replace with U+FF0D |
| `a1-b2-` | `a1-b2-` | `a1-b2-` | After halfwidth alphanumeric, replace with U+002D |

#### Complex Combination Examples

| Input | Output | Logics Explained |
| --- | --- | --- |
| `A-イ-ハト-123-` | `A-イーハトー123-` | 1. `A-`: Alphanumeric context→keep<br>2. `イ-`: Katakana→U+30FC<br>3. `ト-`: Katakana→U+30FC<br>4. `123-`: Alphanumeric context→keep |
| `ｱ-ｲ-ｳ-ｴ-ｵ` | `ｱｰｲｰｳｰｴｰｵ` | All after halfwidth katakana, so use U+FF70 |

## Configuration Options

### `allowProlongedSokuon`

When enabled, allows prolonged sound marks after sokuon (っ/ッ) characters.

| Input | Default Output | With `allowProlongedSokuon` |
| --- | --- | --- |
| `ウッ－` | `ウッ－` (unchanged) | `ウッー` |

### `allowProlongedHatsuon`

When enabled, allows prolonged sound marks after hatsuon (ん/ン) characters.

| Input | Default Output | With `allowProlongedHatsuon` |
| --- | --- | --- |
| `ウン－` | `ウン－` (unchanged) | `ウンー` |

### `replaceProlongedMarksFollowingAlnums`

When enabled, replaces prolonged sound marks following alphanumeric characters with appropriate hyphens.

| Input | Default Output | With `replaceProlongedMarksFollowingAlnums` |
| --- | --- | --- |
| `1ー－2ー3` | `1ー－2ー3` (unchanged) | `1--2-3` |
| `１ー－２ー３` | `１ー－２ー３` (unchanged) | `１－－２－３` |

### `skipAlreadyTransliteratedChars`

When enabled, skips characters that have already been transliterated (have a `source` property).

## Input Dash Characters

The following characters are recognized as dash/hyphen characters for conversion:

| Character | Unicode | Name |
| --- | --- | --- |
| `-` | U+002D | Hyphen-minus |
| `‐` | U+2010 | Hyphen |
| `—` | U+2014 | Em dash |
| `―` | U+2015 | Horizontal bar |
| `−` | U+2212 | Minus sign |
| `－` | U+FF0D | Fullwidth hyphen-minus |
| `ｰ` | U+FF70 | Halfwidth katakana prolonged sound mark |
| `ー` | U+30FC | Katakana-hiragana prolonged sound mark |

## Output Prolonged Sound Marks

| Character | Unicode | Usage |
| --- | --- | --- |
| `ー` | U+30FC | Used after hiragana and fullwidth katakana |
| `ｰ` | U+FF70 | Used after halfwidth katakana |
| `-` | U+002D | Used between halfwidth alphanumeric characters (with option) |
| `－` | U+FF0D | Used between fullwidth alphanumeric characters (with option) |