---
lang: en
id: jisx0201-and-alike
url: /transliterators/jisx0201-and-alike/
title: JIS X 0201 and Alike Transliterators
---
# JIS X 0201 and Alike Transliterators

JIS X 0201 and alike transliterators perform conversions between Unicode codepoints that correspond to the JIS X 0201 character set. The JIS X 0201 character set contains alphanumerics, symbols and katakana characters.

There are two codepoint groups (half-width and full-width) in Unicode that correspond to JIS X 0201 character set. Each transliterator is responsible for conversion one group to the other, in opposite directions.

- Half-width group:
  - Alphabets, numerics, and symbols: U+0020 - U+007E, U+00A5, and U+203E.
  - Half-width katakanas: U+FF61 - U+FF9F.
- Full-width group:
  - Full-width alphabets, numerics, and symbols: U+FF01 - U+FF5E, U+FFE3, and U+FFE5.
  - Wave dash: U+301C.
  - Hiraganas: U+3041 - U+3094.
  - Katakanas: U+30A1 - U+30F7 and U+30FA.
  - Hiragana/katakana voicing marks: U+309B, U+309C, and U+30FC.
  - Japanese punctuations: U+3001, U+3002, U+30A0, and U+30FB.

Some widely used fonts, such as MS Mincho, map incorrect glyphs to codepoints that are supposed to have respective JIS X 0201 counterparts. These transliterators also provides options to address such inconsistencies.

## Half-width to full-width transliterator

This transliterator converts half-width alphanumerics, symbols, and katakanas to their full-width counterparts.

### Options

- `convertGL` — If set to `true`, the transliterator converts half-width alphanumerics and symbols to their full-width counterparts, but not for half-width katakanas. The default is `true`.
- `convertGR` — If set to `true`, the transliterator converts half-width katakanas to their full-width counterparts. The default is `true`.
- `convertUnsafeSpecials` — If set to `true`, convert U+003D (`=`, EQUALS SIGN) to U+30A0 (`゠`, KATAKANA-HIRAGANA DOUBLE HYPHEN).
- `combineVoicingMarks` — If set to `true`, along with `fullwidthToHalfwidth` being `false`, and `convertGR` being `true`, the transliterator will render the pair of a half-width katakana and a voiced sound mark (濁点 — voiced sound mark or 半濁点 — semi-voiced sound mark) into the literally equilavent single full-width katakana character.
- `u005cAsYenSign` — If set to `true`, the transliterator converts U+005C (backslash) to U+00A5 (yen sign). The default is 'true'.
- `u005cAsYenSign` — If set to `true`, treat U+005C (REVERSE SOLIDUS) as if it were U+00A5 (YEN SIGN)
- `u005cAsBackslash` — If set to `true`, treat U+005C (REVERSE SOLIDUS) verbatim. This option cannot be used with `u005cAsYenSign`.
- `u007eAsFullwidthTilde` — If set to `true`, convert U+007E (TILDE) to U+FF5E (FULLWIDTH TILDE). This is mutually exclusive with `u007eAsWaveDash`, `u007eAsOverline`, and `u007eAsFullwidthMacron`.
- `u007eAsWaveDash` — If set to `true`, convert U+007E (TILDE) to U+301C (WAVE DASH). This is mutually exclusive with `u007eAsFullwidthTilde`, `u007eAsOverline`, and `u007eAsFullwidthMacron`.
- `u007eAsOverline` — If set to `true`, convert U+007E (TILDE) to U+203E (OVERLINE). This is mutually exclusive with `u007eAsFullwidthTilde`, `u007eAsWaveDash`, and `u007eAsFullwidthMacron`.
- `u007eAsFullwidthMacron` — If set to `true`, convert U+007E (TILDE) to U+FFE3 (FULLWIDTH MACRON). This is mutually exclusive with `u007eAsFullwidthTilde`, `u007eAsWaveDash`, and `u007eAsOverline`.
- `u00a5AsYenSign` — If set to `true`, convert U+00A5 (YEN SIGN) to U+005C (REVERSE SOLIDUS), The default is `false`.

## Full-width to half-width transliterator

This transliterator converts full-width alphanumerics, symbols, and katakanas to their halfwidth-width counterparts.

### Options

- `convertGL` — If set to `true`, the transliterator converts full-width alphanumerics and symbols to their half-width counterparts, but not for full-width katakanas. The default is `true`.
- `convertGR` — If set to `true`, the transliterator converts full-width katakanas to their half-width counterparts. The default is `true`.
- `convertUnsafeSpecials` — If set to `true`, convert U+30A0 (`゠`, KATAKANA-HIRAGANA DOUBLE HYPHEN) to U+003D (`=`, EQUALS SIGN).
- `u005cAsYenSign` — If set to `true`, the transliterator converts U+FFE5 (FULLWIDTH YEN SIGN) to U+005C (backslash).
- `u005cAsBackslash` — If set to `true`, treat U+FF3C (FULLWIDTH REVERSE SOLIDUS) to U+005C (REVERSE SOLIDUS).
- `u007eAsFullwidthTilde` — If set to `true`, convert U+FF5E (FULLWIDTH TILDE) to U+007E (TILDE).
- `u007eAsWaveDash` — If set to `true`, convert U+301C (WAVE DASH) to U+007E (TILDE).
- `u007eAsOverline` — If set to `true`, convert U+203E (OVERLINE) to U+007E (TILDE).
- `u007eAsFullwidthMacron` — If set to `true`, convert U+FFE3 (FULLWIDTH MACRON) to U+007E (TILDE).
- `u00a5AsYenSign` — If set to `true`, convert U+FFE5 (FULLWIDTH YEN SIGN) to U+00A5 (YEN SIGN).