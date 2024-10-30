---
lang: ja
id: jisx0201-and-alike
url: /transliterators/jisx0201-and-alike/
title: JIS X 0201および類似文字トランスリテレータ
---
# JIS X 0201および類似文字トランスリテレータ

JIS X 0201および類似文字トランスリテレータは、JIS X 0201文字集合に対応するUnicodeコードポイント間の変換を行います。JIS X 0201文字集合には、英数字、記号、カタカナ文字が含まれます。

UnicodeにはJIS X 0201文字集合に対応する2つのコードポイントグループ（半角と全角）があります。各トランスリテレータは、一方のグループから他方への変換を、それぞれ逆方向に行う責任があります。

- 半角グループ：
  - アルファベット、数字、記号：U+0020 - U+007E、U+00A5、U+203E
  - 半角カタカナ：U+FF61 - U+FF9F
- 全角グループ：
  - 全角アルファベット、数字、記号：U+FF01 - U+FF5E、U+FFE3、U+FFE5
  - 波ダッシュ：U+301C
  - ひらがな：U+3041 - U+3094
  - カタカナ：U+30A1 - U+30F7およびU+30FA
  - ひらがな/カタカナ濁点記号：U+309B、U+309C、U+30FC
  - 日本語句読点：U+3001、U+3002、U+30A0、U+30FB

MS明朝などの広く使用されているフォントの中には、JIS X 0201の対応文字を持つべきコードポイントに誤ったグリフをマッピングしているものがあります。これらのトランスリテレータは、そのような不整合に対処するオプションも提供しています。

## 半角から全角へのトランスリテレータ

このトランスリテレータは、半角英数字、記号、カタカナを全角の対応文字に変換します。

### オプション

- `convertGL` — `true`に設定すると、トランスリテレータは半角英数字と記号を全角の対応文字に変換しますが、半角カタカナは変換しません。デフォルトは`true`です。
- `convertGR` — `true`に設定すると、トランスリテレータは半角カタカナを全角の対応文字に変換します。デフォルトは`true`です。
- `convertUnsafeSpecials` — `true`に設定すると、U+003D（`=`、等号）をU+30A0（`゠`、カタカナ・ひらがなダブルハイフン）に変換します。
- `combineVoicingMarks` — `true`に設定し、`fullwidthToHalfwidth`が`false`、`convertGR`が`true`の場合、トランスリテレータは半角カタカナと濁点記号（濁点または半濁点）のペアを、文字通り等価な単一の全角カタカナ文字にレンダリングします。
- `u005cAsYenSign` — `true`に設定すると、トランスリテレータはU+005C（バックスラッシュ）をU+00A5（円記号）に変換します。デフォルトは`true`です。
- `u005cAsYenSign` — `true`に設定すると、U+005C（逆斜線）をU+00A5（円記号）として扱います
- `u005cAsBackslash` — `true`に設定すると、U+005C（逆斜線）をそのまま扱います。このオプションは`u005cAsYenSign`と同時に使用できません。
- `u007eAsFullwidthTilde` — `true`に設定すると、U+007E（チルダ）をU+FF5E（全角チルダ）に変換します。これは`u007eAsWaveDash`、`u007eAsOverline`、`u007eAsFullwidthMacron`と相互排他的です。
- `u007eAsWaveDash` — `true`に設定すると、U+007E（チルダ）をU+301C（波ダッシュ）に変換します。これは`u007eAsFullwidthTilde`、`u007eAsOverline`、`u007eAsFullwidthMacron`と相互排他的です。
- `u007eAsOverline` — `true`に設定すると、U+007E（チルダ）をU+203E（オーバーライン）に変換します。これは`u007eAsFullwidthTilde`、`u007eAsWaveDash`、`u007eAsFullwidthMacron`と相互排他的です。
- `u007eAsFullwidthMacron` — `true`に設定すると、U+007E（チルダ）をU+FFE3（全角マクロン）に変換します。これは`u007eAsFullwidthTilde`、`u007eAsWaveDash`、`u007eAsOverline`と相互排他的です。
- `u00a5AsYenSign` — `true`に設定すると、U+00A5（円記号）をU+005C（逆斜線）に変換します。デフォルトは`false`です。

## 全角から半角へのトランスリテレータ

このトランスリテレータは、全角英数字、記号、カタカナを半角の対応文字に変換します。

### オプション

- `convertGL` — `true`に設定すると、トランスリテレータは全角英数字と記号を半角の対応文字に変換しますが、全角カタカナは変換しません。デフォルトは`true`です。
- `convertGR` — `true`に設定すると、トランスリテレータは全角カタカナを半角の対応文字に変換します。デフォルトは`true`です。
- `convertUnsafeSpecials` — `true`に設定すると、U+30A0（`゠`、カタカナ・ひらがなダブルハイフン）をU+003D（`=`、等号）に変換します。
- `u005cAsYenSign` — `true`に設定すると、トランスリテレータはU+FFE5（全角円記号）をU+005C（バックスラッシュ）に変換します。
- `u005cAsBackslash` — `true`に設定すると、U+FF3C（全角逆斜線）をU+005C（逆斜線）として扱います。
- `u007eAsFullwidthTilde` — `true`に設定すると、U+FF5E（全角チルダ）をU+007E（チルダ）に変換します。
- `u007eAsWaveDash` — `true`に設定すると、U+301C（波ダッシュ）をU+007E（チルダ）に変換します。
- `u007eAsOverline` — `true`に設定すると、U+203E（オーバーライン）をU+007E（チルダ）に変換します。
- `u007eAsFullwidthMacron` — `true`に設定すると、U+FFE3（全角マクロン）をU+007E（チルダ）に変換します。
- `u00a5AsYenSign` — `true`に設定すると、U+FFE5（全角円記号）をU+00A5（円記号）に変換します。