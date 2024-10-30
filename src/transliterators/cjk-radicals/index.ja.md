---
lang: ja
id: cjk-radicals
url: /transliterators/cjk-radicals/
title: CJK部首翻字器
---
# CJK部首翻字器

CJK部首翻字器は、康熙部首文字とCJK部首補助文字を対応する標準的なCJK統合漢字に変換します。これは、辞書や教材で漢字の構成要素を表すために使用される独立した部首文字を含むテキストの正規化に有用です。

## 概要

このトランスリテレータは、主に2つの部首文字セットを処理します：
- **康熙部首** (U+2F00–U+2FD5): 中国の辞書で使用される伝統的な214部首
- **CJK部首補助** (U+2E80–U+2EF3): 追加の部首形式とバリアント

これらの部首文字は対応する完全なCJK文字に置き換えられ、一貫したテキスト表現を確保し、特殊な部首コードポイントを完全にサポートしていない可能性のあるシステムとの互換性を向上させます。

## 文字マッピング表

| 元のコードポイント | 元の文字名 | 元の文字 | 代替コードポイント | 代替文字名 | 代替文字 |
| --- | --- | --- | --- | --- | --- |
| U+2F00 | KANGXI RADICAL ONE | ![size=24](/assets/img/genglyphsvg/u2f00.svg) | U+4E00 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e00.svg) |
| U+2F01 | KANGXI RADICAL LINE | ![size=24](/assets/img/genglyphsvg/u2f01.svg) | U+4E28 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e28.svg) |
| U+2F02 | KANGXI RADICAL DOT | ![size=24](/assets/img/genglyphsvg/u2f02.svg) | U+4E36 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e36.svg) |
| U+2F03 | KANGXI RADICAL SLASH | ![size=24](/assets/img/genglyphsvg/u2f03.svg) | U+4E3F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e3f.svg) |
| U+2F04 | KANGXI RADICAL SECOND | ![size=24](/assets/img/genglyphsvg/u2f04.svg) | U+4E59 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e59.svg) |
| U+2F05 | KANGXI RADICAL HOOK | ![size=24](/assets/img/genglyphsvg/u2f05.svg) | U+4E85 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e85.svg) |
| U+2F06 | KANGXI RADICAL TWO | ![size=24](/assets/img/genglyphsvg/u2f06.svg) | U+4E8C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e8c.svg) |
| U+2F07 | KANGXI RADICAL LID | ![size=24](/assets/img/genglyphsvg/u2f07.svg) | U+4EA0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4ea0.svg) |
| U+2F08 | KANGXI RADICAL MAN | ![size=24](/assets/img/genglyphsvg/u2f08.svg) | U+4EBA | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4eba.svg) |
| U+2F09 | KANGXI RADICAL LEGS | ![size=24](/assets/img/genglyphsvg/u2f09.svg) | U+513F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u513f.svg) |
| U+2F0A | KANGXI RADICAL ENTER | ![size=24](/assets/img/genglyphsvg/u2f0a.svg) | U+5165 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5165.svg) |
| U+2F0B | KANGXI RADICAL EIGHT | ![size=24](/assets/img/genglyphsvg/u2f0b.svg) | U+516B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u516b.svg) |
| U+2F0C | KANGXI RADICAL DOWN BOX | ![size=24](/assets/img/genglyphsvg/u2f0c.svg) | U+5182 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5182.svg) |
| U+2F0D | KANGXI RADICAL COVER | ![size=24](/assets/img/genglyphsvg/u2f0d.svg) | U+5196 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5196.svg) |
| U+2F0E | KANGXI RADICAL ICE | ![size=24](/assets/img/genglyphsvg/u2f0e.svg) | U+51AB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u51ab.svg) |
| U+2F0F | KANGXI RADICAL TABLE | ![size=24](/assets/img/genglyphsvg/u2f0f.svg) | U+51E0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u51e0.svg) |
| U+2F10 | KANGXI RADICAL OPEN BOX | ![size=24](/assets/img/genglyphsvg/u2f10.svg) | U+51F5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u51f5.svg) |
| U+2F11 | KANGXI RADICAL KNIFE | ![size=24](/assets/img/genglyphsvg/u2f11.svg) | U+5200 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5200.svg) |
| U+2F12 | KANGXI RADICAL POWER | ![size=24](/assets/img/genglyphsvg/u2f12.svg) | U+529B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u529b.svg) |
| U+2F13 | KANGXI RADICAL WRAP | ![size=24](/assets/img/genglyphsvg/u2f13.svg) | U+52F9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u52f9.svg) |
| U+2F14 | KANGXI RADICAL SPOON | ![size=24](/assets/img/genglyphsvg/u2f14.svg) | U+5315 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5315.svg) |
| U+2F15 | KANGXI RADICAL RIGHT OPEN BOX | ![size=24](/assets/img/genglyphsvg/u2f15.svg) | U+531A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u531a.svg) |
| U+2F16 | KANGXI RADICAL HIDING ENCLOSURE | ![size=24](/assets/img/genglyphsvg/u2f16.svg) | U+5338 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5338.svg) |
| U+2F17 | KANGXI RADICAL TEN | ![size=24](/assets/img/genglyphsvg/u2f17.svg) | U+5341 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5341.svg) |
| U+2F18 | KANGXI RADICAL DIVINATION | ![size=24](/assets/img/genglyphsvg/u2f18.svg) | U+535C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u535c.svg) |
| U+2F19 | KANGXI RADICAL SEAL | ![size=24](/assets/img/genglyphsvg/u2f19.svg) | U+5369 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5369.svg) |
| U+2F1A | KANGXI RADICAL CLIFF | ![size=24](/assets/img/genglyphsvg/u2f1a.svg) | U+5382 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5382.svg) |
| U+2F1B | KANGXI RADICAL PRIVATE | ![size=24](/assets/img/genglyphsvg/u2f1b.svg) | U+53B6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u53b6.svg) |
| U+2F1C | KANGXI RADICAL AGAIN | ![size=24](/assets/img/genglyphsvg/u2f1c.svg) | U+53C8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u53c8.svg) |
| U+2F1D | KANGXI RADICAL MOUTH | ![size=24](/assets/img/genglyphsvg/u2f1d.svg) | U+53E3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u53e3.svg) |
| U+2F1E | KANGXI RADICAL ENCLOSURE | ![size=24](/assets/img/genglyphsvg/u2f1e.svg) | U+56D7 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u56d7.svg) |
| U+2F1F | KANGXI RADICAL EARTH | ![size=24](/assets/img/genglyphsvg/u2f1f.svg) | U+571F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u571f.svg) |
| U+2F20 | KANGXI RADICAL SCHOLAR | ![size=24](/assets/img/genglyphsvg/u2f20.svg) | U+58EB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u58eb.svg) |
| U+2F21 | KANGXI RADICAL GO | ![size=24](/assets/img/genglyphsvg/u2f21.svg) | U+5902 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5902.svg) |
| U+2F22 | KANGXI RADICAL GO SLOWLY | ![size=24](/assets/img/genglyphsvg/u2f22.svg) | U+590A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u590a.svg) |
| U+2F23 | KANGXI RADICAL EVENING | ![size=24](/assets/img/genglyphsvg/u2f23.svg) | U+5915 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5915.svg) |
| U+2F24 | KANGXI RADICAL BIG | ![size=24](/assets/img/genglyphsvg/u2f24.svg) | U+5927 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5927.svg) |
| U+2F25 | KANGXI RADICAL WOMAN | ![size=24](/assets/img/genglyphsvg/u2f25.svg) | U+5973 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5973.svg) |
| U+2F26 | KANGXI RADICAL CHILD | ![size=24](/assets/img/genglyphsvg/u2f26.svg) | U+5B50 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5b50.svg) |
| U+2F27 | KANGXI RADICAL ROOF | ![size=24](/assets/img/genglyphsvg/u2f27.svg) | U+5B80 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5b80.svg) |
| U+2F28 | KANGXI RADICAL INCH | ![size=24](/assets/img/genglyphsvg/u2f28.svg) | U+5BF8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5bf8.svg) |
| U+2F29 | KANGXI RADICAL SMALL | ![size=24](/assets/img/genglyphsvg/u2f29.svg) | U+5C0F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c0f.svg) |
| U+2F2A | KANGXI RADICAL LAME | ![size=24](/assets/img/genglyphsvg/u2f2a.svg) | U+5C22 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c22.svg) |
| U+2F2B | KANGXI RADICAL CORPSE | ![size=24](/assets/img/genglyphsvg/u2f2b.svg) | U+5C38 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c38.svg) |
| U+2F2C | KANGXI RADICAL SPROUT | ![size=24](/assets/img/genglyphsvg/u2f2c.svg) | U+5C6E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c6e.svg) |
| U+2F2D | KANGXI RADICAL MOUNTAIN | ![size=24](/assets/img/genglyphsvg/u2f2d.svg) | U+5C71 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c71.svg) |
| U+2F2E | KANGXI RADICAL RIVER | ![size=24](/assets/img/genglyphsvg/u2f2e.svg) | U+5DDB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5ddb.svg) |
| U+2F2F | KANGXI RADICAL WORK | ![size=24](/assets/img/genglyphsvg/u2f2f.svg) | U+5DE5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5de5.svg) |
| U+2F30 | KANGXI RADICAL ONESELF | ![size=24](/assets/img/genglyphsvg/u2f30.svg) | U+5DF1 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5df1.svg) |
| U+2F31 | KANGXI RADICAL TURBAN | ![size=24](/assets/img/genglyphsvg/u2f31.svg) | U+5DFE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5dfe.svg) |
| U+2F32 | KANGXI RADICAL DRY | ![size=24](/assets/img/genglyphsvg/u2f32.svg) | U+5E72 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5e72.svg) |
| U+2F33 | KANGXI RADICAL SHORT THREAD | ![size=24](/assets/img/genglyphsvg/u2f33.svg) | U+5E7A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5e7a.svg) |
| U+2F34 | KANGXI RADICAL DOTTED CLIFF | ![size=24](/assets/img/genglyphsvg/u2f34.svg) | U+5E7F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5e7f.svg) |
| U+2F35 | KANGXI RADICAL LONG STRIDE | ![size=24](/assets/img/genglyphsvg/u2f35.svg) | U+5EF4 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5ef4.svg) |
| U+2F36 | KANGXI RADICAL TWO HANDS | ![size=24](/assets/img/genglyphsvg/u2f36.svg) | U+5EFE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5efe.svg) |
| U+2F37 | KANGXI RADICAL SHOOT | ![size=24](/assets/img/genglyphsvg/u2f37.svg) | U+5F0B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f0b.svg) |
| U+2F38 | KANGXI RADICAL BOW | ![size=24](/assets/img/genglyphsvg/u2f38.svg) | U+5F13 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f13.svg) |
| U+2F39 | KANGXI RADICAL SNOUT | ![size=24](/assets/img/genglyphsvg/u2f39.svg) | U+5F50 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f50.svg) |
| U+2F3A | KANGXI RADICAL BRISTLE | ![size=24](/assets/img/genglyphsvg/u2f3a.svg) | U+5F61 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f61.svg) |
| U+2F3B | KANGXI RADICAL STEP | ![size=24](/assets/img/genglyphsvg/u2f3b.svg) | U+5F73 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f73.svg) |
| U+2F3C | KANGXI RADICAL HEART | ![size=24](/assets/img/genglyphsvg/u2f3c.svg) | U+5FC3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5fc3.svg) |
| U+2F3D | KANGXI RADICAL HALBERD | ![size=24](/assets/img/genglyphsvg/u2f3d.svg) | U+6208 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6208.svg) |
| U+2F3E | KANGXI RADICAL DOOR | ![size=24](/assets/img/genglyphsvg/u2f3e.svg) | U+6236 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6236.svg) |
| U+2F3F | KANGXI RADICAL HAND | ![size=24](/assets/img/genglyphsvg/u2f3f.svg) | U+624B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u624b.svg) |
| U+2F40 | KANGXI RADICAL BRANCH | ![size=24](/assets/img/genglyphsvg/u2f40.svg) | U+652F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u652f.svg) |
| U+2F41 | KANGXI RADICAL RAP | ![size=24](/assets/img/genglyphsvg/u2f41.svg) | U+6534 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6534.svg) |
| U+2F42 | KANGXI RADICAL SCRIPT | ![size=24](/assets/img/genglyphsvg/u2f42.svg) | U+6587 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6587.svg) |
| U+2F43 | KANGXI RADICAL DIPPER | ![size=24](/assets/img/genglyphsvg/u2f43.svg) | U+6597 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6597.svg) |
| U+2F44 | KANGXI RADICAL AXE | ![size=24](/assets/img/genglyphsvg/u2f44.svg) | U+65A4 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u65a4.svg) |
| U+2F45 | KANGXI RADICAL SQUARE | ![size=24](/assets/img/genglyphsvg/u2f45.svg) | U+65B9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u65b9.svg) |
| U+2F46 | KANGXI RADICAL NOT | ![size=24](/assets/img/genglyphsvg/u2f46.svg) | U+65E0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u65e0.svg) |
| U+2F47 | KANGXI RADICAL SUN | ![size=24](/assets/img/genglyphsvg/u2f47.svg) | U+65E5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u65e5.svg) |
| U+2F48 | KANGXI RADICAL SAY | ![size=24](/assets/img/genglyphsvg/u2f48.svg) | U+66F0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u66f0.svg) |
| U+2F49 | KANGXI RADICAL MOON | ![size=24](/assets/img/genglyphsvg/u2f49.svg) | U+6708 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6708.svg) |
| U+2F4A | KANGXI RADICAL TREE | ![size=24](/assets/img/genglyphsvg/u2f4a.svg) | U+6728 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6728.svg) |
| U+2F4B | KANGXI RADICAL LACK | ![size=24](/assets/img/genglyphsvg/u2f4b.svg) | U+6B20 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6b20.svg) |
| U+2F4C | KANGXI RADICAL STOP | ![size=24](/assets/img/genglyphsvg/u2f4c.svg) | U+6B62 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6b62.svg) |
| U+2F4D | KANGXI RADICAL DEATH | ![size=24](/assets/img/genglyphsvg/u2f4d.svg) | U+6B79 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6b79.svg) |
| U+2F4E | KANGXI RADICAL WEAPON | ![size=24](/assets/img/genglyphsvg/u2f4e.svg) | U+6BB3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6bb3.svg) |
| U+2F4F | KANGXI RADICAL DO NOT | ![size=24](/assets/img/genglyphsvg/u2f4f.svg) | U+6BCB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6bcb.svg) |
| U+2F50 | KANGXI RADICAL COMPARE | ![size=24](/assets/img/genglyphsvg/u2f50.svg) | U+6BD4 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6bd4.svg) |
| U+2F51 | KANGXI RADICAL FUR | ![size=24](/assets/img/genglyphsvg/u2f51.svg) | U+6BDB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6bdb.svg) |
| U+2F52 | KANGXI RADICAL CLAN | ![size=24](/assets/img/genglyphsvg/u2f52.svg) | U+6C0F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c0f.svg) |
| U+2F53 | KANGXI RADICAL STEAM | ![size=24](/assets/img/genglyphsvg/u2f53.svg) | U+6C14 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c14.svg) |
| U+2F54 | KANGXI RADICAL WATER | ![size=24](/assets/img/genglyphsvg/u2f54.svg) | U+6C34 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c34.svg) |
| U+2F55 | KANGXI RADICAL FIRE | ![size=24](/assets/img/genglyphsvg/u2f55.svg) | U+706B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u706b.svg) |
| U+2F56 | KANGXI RADICAL CLAW | ![size=24](/assets/img/genglyphsvg/u2f56.svg) | U+722A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u722a.svg) |
| U+2F57 | KANGXI RADICAL FATHER | ![size=24](/assets/img/genglyphsvg/u2f57.svg) | U+7236 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7236.svg) |
| U+2F58 | KANGXI RADICAL DOUBLE X | ![size=24](/assets/img/genglyphsvg/u2f58.svg) | U+723B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u723b.svg) |
| U+2F59 | KANGXI RADICAL HALF TREE TRUNK | ![size=24](/assets/img/genglyphsvg/u2f59.svg) | U+723F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u723f.svg) |
| U+2F5A | KANGXI RADICAL SLICE | ![size=24](/assets/img/genglyphsvg/u2f5a.svg) | U+7247 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7247.svg) |
| U+2F5B | KANGXI RADICAL FANG | ![size=24](/assets/img/genglyphsvg/u2f5b.svg) | U+7259 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7259.svg) |
| U+2F5C | KANGXI RADICAL COW | ![size=24](/assets/img/genglyphsvg/u2f5c.svg) | U+725B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u725b.svg) |
| U+2F5D | KANGXI RADICAL DOG | ![size=24](/assets/img/genglyphsvg/u2f5d.svg) | U+72AC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u72ac.svg) |
| U+2F5E | KANGXI RADICAL PROFOUND | ![size=24](/assets/img/genglyphsvg/u2f5e.svg) | U+7384 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7384.svg) |
| U+2F5F | KANGXI RADICAL JADE | ![size=24](/assets/img/genglyphsvg/u2f5f.svg) | U+7389 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7389.svg) |
| U+2F60 | KANGXI RADICAL MELON | ![size=24](/assets/img/genglyphsvg/u2f60.svg) | U+74DC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u74dc.svg) |
| U+2F61 | KANGXI RADICAL TILE | ![size=24](/assets/img/genglyphsvg/u2f61.svg) | U+74E6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u74e6.svg) |
| U+2F62 | KANGXI RADICAL SWEET | ![size=24](/assets/img/genglyphsvg/u2f62.svg) | U+7518 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7518.svg) |
| U+2F63 | KANGXI RADICAL LIFE | ![size=24](/assets/img/genglyphsvg/u2f63.svg) | U+751F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u751f.svg) |
| U+2F64 | KANGXI RADICAL USE | ![size=24](/assets/img/genglyphsvg/u2f64.svg) | U+7528 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7528.svg) |
| U+2F65 | KANGXI RADICAL FIELD | ![size=24](/assets/img/genglyphsvg/u2f65.svg) | U+7530 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7530.svg) |
| U+2F66 | KANGXI RADICAL BOLT OF CLOTH | ![size=24](/assets/img/genglyphsvg/u2f66.svg) | U+758B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u758b.svg) |
| U+2F67 | KANGXI RADICAL SICKNESS | ![size=24](/assets/img/genglyphsvg/u2f67.svg) | U+7592 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7592.svg) |
| U+2F68 | KANGXI RADICAL DOTTED TENT | ![size=24](/assets/img/genglyphsvg/u2f68.svg) | U+7676 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7676.svg) |
| U+2F69 | KANGXI RADICAL WHITE | ![size=24](/assets/img/genglyphsvg/u2f69.svg) | U+767D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u767d.svg) |
| U+2F6A | KANGXI RADICAL SKIN | ![size=24](/assets/img/genglyphsvg/u2f6a.svg) | U+76AE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u76ae.svg) |
| U+2F6B | KANGXI RADICAL DISH | ![size=24](/assets/img/genglyphsvg/u2f6b.svg) | U+76BF | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u76bf.svg) |
| U+2F6C | KANGXI RADICAL EYE | ![size=24](/assets/img/genglyphsvg/u2f6c.svg) | U+76EE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u76ee.svg) |
| U+2F6D | KANGXI RADICAL SPEAR | ![size=24](/assets/img/genglyphsvg/u2f6d.svg) | U+77DB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u77db.svg) |
| U+2F6E | KANGXI RADICAL ARROW | ![size=24](/assets/img/genglyphsvg/u2f6e.svg) | U+77E2 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u77e2.svg) |
| U+2F6F | KANGXI RADICAL STONE | ![size=24](/assets/img/genglyphsvg/u2f6f.svg) | U+77F3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u77f3.svg) |
| U+2F70 | KANGXI RADICAL SPIRIT | ![size=24](/assets/img/genglyphsvg/u2f70.svg) | U+793A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u793a.svg) |
| U+2F71 | KANGXI RADICAL TRACK | ![size=24](/assets/img/genglyphsvg/u2f71.svg) | U+79B8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u79b8.svg) |
| U+2F72 | KANGXI RADICAL GRAIN | ![size=24](/assets/img/genglyphsvg/u2f72.svg) | U+79BE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u79be.svg) |
| U+2F73 | KANGXI RADICAL CAVE | ![size=24](/assets/img/genglyphsvg/u2f73.svg) | U+7A74 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7a74.svg) |
| U+2F74 | KANGXI RADICAL STAND | ![size=24](/assets/img/genglyphsvg/u2f74.svg) | U+7ACB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7acb.svg) |
| U+2F75 | KANGXI RADICAL BAMBOO | ![size=24](/assets/img/genglyphsvg/u2f75.svg) | U+7AF9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7af9.svg) |
| U+2F76 | KANGXI RADICAL RICE | ![size=24](/assets/img/genglyphsvg/u2f76.svg) | U+7C73 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7c73.svg) |
| U+2F77 | KANGXI RADICAL SILK | ![size=24](/assets/img/genglyphsvg/u2f77.svg) | U+7CF8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7cf8.svg) |
| U+2F78 | KANGXI RADICAL JAR | ![size=24](/assets/img/genglyphsvg/u2f78.svg) | U+7F36 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7f36.svg) |
| U+2F79 | KANGXI RADICAL NET | ![size=24](/assets/img/genglyphsvg/u2f79.svg) | U+7F51 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7f51.svg) |
| U+2F7A | KANGXI RADICAL SHEEP | ![size=24](/assets/img/genglyphsvg/u2f7a.svg) | U+7F8A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7f8a.svg) |
| U+2F7B | KANGXI RADICAL FEATHER | ![size=24](/assets/img/genglyphsvg/u2f7b.svg) | U+7FBD | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7fbd.svg) |
| U+2F7C | KANGXI RADICAL OLD | ![size=24](/assets/img/genglyphsvg/u2f7c.svg) | U+8001 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8001.svg) |
| U+2F7D | KANGXI RADICAL AND | ![size=24](/assets/img/genglyphsvg/u2f7d.svg) | U+800C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u800c.svg) |
| U+2F7E | KANGXI RADICAL PLOW | ![size=24](/assets/img/genglyphsvg/u2f7e.svg) | U+8012 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8012.svg) |
| U+2F7F | KANGXI RADICAL EAR | ![size=24](/assets/img/genglyphsvg/u2f7f.svg) | U+8033 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8033.svg) |
| U+2F80 | KANGXI RADICAL BRUSH | ![size=24](/assets/img/genglyphsvg/u2f80.svg) | U+807F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u807f.svg) |
| U+2F81 | KANGXI RADICAL MEAT | ![size=24](/assets/img/genglyphsvg/u2f81.svg) | U+8089 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8089.svg) |
| U+2F82 | KANGXI RADICAL MINISTER | ![size=24](/assets/img/genglyphsvg/u2f82.svg) | U+81E3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u81e3.svg) |
| U+2F83 | KANGXI RADICAL SELF | ![size=24](/assets/img/genglyphsvg/u2f83.svg) | U+81EA | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u81ea.svg) |
| U+2F84 | KANGXI RADICAL ARRIVE | ![size=24](/assets/img/genglyphsvg/u2f84.svg) | U+81F3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u81f3.svg) |
| U+2F85 | KANGXI RADICAL MORTAR | ![size=24](/assets/img/genglyphsvg/u2f85.svg) | U+81FC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u81fc.svg) |
| U+2F86 | KANGXI RADICAL TONGUE | ![size=24](/assets/img/genglyphsvg/u2f86.svg) | U+820C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u820c.svg) |
| U+2F87 | KANGXI RADICAL OPPOSE | ![size=24](/assets/img/genglyphsvg/u2f87.svg) | U+821B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u821b.svg) |
| U+2F88 | KANGXI RADICAL BOAT | ![size=24](/assets/img/genglyphsvg/u2f88.svg) | U+821F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u821f.svg) |
| U+2F89 | KANGXI RADICAL STOPPING | ![size=24](/assets/img/genglyphsvg/u2f89.svg) | U+826E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u826e.svg) |
| U+2F8A | KANGXI RADICAL COLOR | ![size=24](/assets/img/genglyphsvg/u2f8a.svg) | U+8272 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8272.svg) |
| U+2F8B | KANGXI RADICAL GRASS | ![size=24](/assets/img/genglyphsvg/u2f8b.svg) | U+8278 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8278.svg) |
| U+2F8C | KANGXI RADICAL TIGER | ![size=24](/assets/img/genglyphsvg/u2f8c.svg) | U+864D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u864d.svg) |
| U+2F8D | KANGXI RADICAL INSECT | ![size=24](/assets/img/genglyphsvg/u2f8d.svg) | U+866B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u866b.svg) |
| U+2F8E | KANGXI RADICAL BLOOD | ![size=24](/assets/img/genglyphsvg/u2f8e.svg) | U+8840 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8840.svg) |
| U+2F8F | KANGXI RADICAL WALK ENCLOSURE | ![size=24](/assets/img/genglyphsvg/u2f8f.svg) | U+884C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u884c.svg) |
| U+2F90 | KANGXI RADICAL CLOTHES | ![size=24](/assets/img/genglyphsvg/u2f90.svg) | U+8863 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8863.svg) |
| U+2F91 | KANGXI RADICAL WEST | ![size=24](/assets/img/genglyphsvg/u2f91.svg) | U+897E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u897e.svg) |
| U+2F92 | KANGXI RADICAL SEE | ![size=24](/assets/img/genglyphsvg/u2f92.svg) | U+898B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u898b.svg) |
| U+2F93 | KANGXI RADICAL HORN | ![size=24](/assets/img/genglyphsvg/u2f93.svg) | U+89D2 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u89d2.svg) |
| U+2F94 | KANGXI RADICAL SPEECH | ![size=24](/assets/img/genglyphsvg/u2f94.svg) | U+8A00 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8a00.svg) |
| U+2F95 | KANGXI RADICAL VALLEY | ![size=24](/assets/img/genglyphsvg/u2f95.svg) | U+8C37 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8c37.svg) |
| U+2F96 | KANGXI RADICAL BEAN | ![size=24](/assets/img/genglyphsvg/u2f96.svg) | U+8C46 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8c46.svg) |
| U+2F97 | KANGXI RADICAL PIG | ![size=24](/assets/img/genglyphsvg/u2f97.svg) | U+8C55 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8c55.svg) |
| U+2F98 | KANGXI RADICAL BADGER | ![size=24](/assets/img/genglyphsvg/u2f98.svg) | U+8C78 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8c78.svg) |
| U+2F99 | KANGXI RADICAL SHELL | ![size=24](/assets/img/genglyphsvg/u2f99.svg) | U+8C9D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8c9d.svg) |
| U+2F9A | KANGXI RADICAL RED | ![size=24](/assets/img/genglyphsvg/u2f9a.svg) | U+8D64 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8d64.svg) |
| U+2F9B | KANGXI RADICAL RUN | ![size=24](/assets/img/genglyphsvg/u2f9b.svg) | U+8D70 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8d70.svg) |
| U+2F9C | KANGXI RADICAL FOOT | ![size=24](/assets/img/genglyphsvg/u2f9c.svg) | U+8DB3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8db3.svg) |
| U+2F9D | KANGXI RADICAL BODY | ![size=24](/assets/img/genglyphsvg/u2f9d.svg) | U+8EAB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8eab.svg) |
| U+2F9E | KANGXI RADICAL CART | ![size=24](/assets/img/genglyphsvg/u2f9e.svg) | U+8ECA | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8eca.svg) |
| U+2F9F | KANGXI RADICAL BITTER | ![size=24](/assets/img/genglyphsvg/u2f9f.svg) | U+8F9B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8f9b.svg) |
| U+2FA0 | KANGXI RADICAL MORNING | ![size=24](/assets/img/genglyphsvg/u2fa0.svg) | U+8FB0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8fb0.svg) |
| U+2FA1 | KANGXI RADICAL WALK | ![size=24](/assets/img/genglyphsvg/u2fa1.svg) | U+8FB5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8fb5.svg) |
| U+2FA2 | KANGXI RADICAL CITY | ![size=24](/assets/img/genglyphsvg/u2fa2.svg) | U+9091 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9091.svg) |
| U+2FA3 | KANGXI RADICAL WINE | ![size=24](/assets/img/genglyphsvg/u2fa3.svg) | U+9149 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9149.svg) |
| U+2FA4 | KANGXI RADICAL DISTINGUISH | ![size=24](/assets/img/genglyphsvg/u2fa4.svg) | U+91C6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u91c6.svg) |
| U+2FA5 | KANGXI RADICAL VILLAGE | ![size=24](/assets/img/genglyphsvg/u2fa5.svg) | U+91CC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u91cc.svg) |
| U+2FA6 | KANGXI RADICAL GOLD | ![size=24](/assets/img/genglyphsvg/u2fa6.svg) | U+91D1 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u91d1.svg) |
| U+2FA7 | KANGXI RADICAL LONG | ![size=24](/assets/img/genglyphsvg/u2fa7.svg) | U+9577 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9577.svg) |
| U+2FA8 | KANGXI RADICAL GATE | ![size=24](/assets/img/genglyphsvg/u2fa8.svg) | U+9580 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9580.svg) |
| U+2FA9 | KANGXI RADICAL MOUND | ![size=24](/assets/img/genglyphsvg/u2fa9.svg) | U+961C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u961c.svg) |
| U+2FAA | KANGXI RADICAL SLAVE | ![size=24](/assets/img/genglyphsvg/u2faa.svg) | U+96B6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u96b6.svg) |
| U+2FAB | KANGXI RADICAL SHORT TAILED BIRD | ![size=24](/assets/img/genglyphsvg/u2fab.svg) | U+96B9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u96b9.svg) |
| U+2FAC | KANGXI RADICAL RAIN | ![size=24](/assets/img/genglyphsvg/u2fac.svg) | U+96E8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u96e8.svg) |
| U+2FAD | KANGXI RADICAL BLUE | ![size=24](/assets/img/genglyphsvg/u2fad.svg) | U+9751 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9751.svg) |
| U+2FAE | KANGXI RADICAL WRONG | ![size=24](/assets/img/genglyphsvg/u2fae.svg) | U+975E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u975e.svg) |
| U+2FAF | KANGXI RADICAL FACE | ![size=24](/assets/img/genglyphsvg/u2faf.svg) | U+9762 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9762.svg) |
| U+2FB0 | KANGXI RADICAL LEATHER | ![size=24](/assets/img/genglyphsvg/u2fb0.svg) | U+9769 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9769.svg) |
| U+2FB1 | KANGXI RADICAL TANNED LEATHER | ![size=24](/assets/img/genglyphsvg/u2fb1.svg) | U+97CB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u97cb.svg) |
| U+2FB2 | KANGXI RADICAL LEEK | ![size=24](/assets/img/genglyphsvg/u2fb2.svg) | U+97ED | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u97ed.svg) |
| U+2FB3 | KANGXI RADICAL SOUND | ![size=24](/assets/img/genglyphsvg/u2fb3.svg) | U+97F3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u97f3.svg) |
| U+2FB4 | KANGXI RADICAL LEAF | ![size=24](/assets/img/genglyphsvg/u2fb4.svg) | U+9801 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9801.svg) |
| U+2FB5 | KANGXI RADICAL WIND | ![size=24](/assets/img/genglyphsvg/u2fb5.svg) | U+98A8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98a8.svg) |
| U+2FB6 | KANGXI RADICAL FLY | ![size=24](/assets/img/genglyphsvg/u2fb6.svg) | U+98DB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98db.svg) |
| U+2FB7 | KANGXI RADICAL EAT | ![size=24](/assets/img/genglyphsvg/u2fb7.svg) | U+98DF | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98df.svg) |
| U+2FB8 | KANGXI RADICAL HEAD | ![size=24](/assets/img/genglyphsvg/u2fb8.svg) | U+9996 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9996.svg) |
| U+2FB9 | KANGXI RADICAL FRAGRANT | ![size=24](/assets/img/genglyphsvg/u2fb9.svg) | U+9999 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9999.svg) |
| U+2FBA | KANGXI RADICAL HORSE | ![size=24](/assets/img/genglyphsvg/u2fba.svg) | U+99AC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u99ac.svg) |
| U+2FBB | KANGXI RADICAL BONE | ![size=24](/assets/img/genglyphsvg/u2fbb.svg) | U+9AA8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9aa8.svg) |
| U+2FBC | KANGXI RADICAL TALL | ![size=24](/assets/img/genglyphsvg/u2fbc.svg) | U+9AD8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ad8.svg) |
| U+2FBD | KANGXI RADICAL HAIR | ![size=24](/assets/img/genglyphsvg/u2fbd.svg) | U+9ADF | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9adf.svg) |
| U+2FBE | KANGXI RADICAL FIGHT | ![size=24](/assets/img/genglyphsvg/u2fbe.svg) | U+9B25 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b25.svg) |
| U+2FBF | KANGXI RADICAL SACRIFICIAL WINE | ![size=24](/assets/img/genglyphsvg/u2fbf.svg) | U+9B2F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b2f.svg) |
| U+2FC0 | KANGXI RADICAL CAULDRON | ![size=24](/assets/img/genglyphsvg/u2fc0.svg) | U+9B32 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b32.svg) |
| U+2FC1 | KANGXI RADICAL GHOST | ![size=24](/assets/img/genglyphsvg/u2fc1.svg) | U+9B3C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b3c.svg) |
| U+2FC2 | KANGXI RADICAL FISH | ![size=24](/assets/img/genglyphsvg/u2fc2.svg) | U+9B5A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b5a.svg) |
| U+2FC3 | KANGXI RADICAL BIRD | ![size=24](/assets/img/genglyphsvg/u2fc3.svg) | U+9CE5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ce5.svg) |
| U+2FC4 | KANGXI RADICAL SALT | ![size=24](/assets/img/genglyphsvg/u2fc4.svg) | U+9E75 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9e75.svg) |
| U+2FC5 | KANGXI RADICAL DEER | ![size=24](/assets/img/genglyphsvg/u2fc5.svg) | U+9E7F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9e7f.svg) |
| U+2FC6 | KANGXI RADICAL WHEAT | ![size=24](/assets/img/genglyphsvg/u2fc6.svg) | U+9EA5 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ea5.svg) |
| U+2FC7 | KANGXI RADICAL HEMP | ![size=24](/assets/img/genglyphsvg/u2fc7.svg) | U+9EBB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ebb.svg) |
| U+2FC8 | KANGXI RADICAL YELLOW | ![size=24](/assets/img/genglyphsvg/u2fc8.svg) | U+9EC3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ec3.svg) |
| U+2FC9 | KANGXI RADICAL MILLET | ![size=24](/assets/img/genglyphsvg/u2fc9.svg) | U+9ECD | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ecd.svg) |
| U+2FCA | KANGXI RADICAL BLACK | ![size=24](/assets/img/genglyphsvg/u2fca.svg) | U+9ED1 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ed1.svg) |
| U+2FCB | KANGXI RADICAL EMBROIDERY | ![size=24](/assets/img/genglyphsvg/u2fcb.svg) | U+9EF9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ef9.svg) |
| U+2FCC | KANGXI RADICAL FROG | ![size=24](/assets/img/genglyphsvg/u2fcc.svg) | U+9EFD | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9efd.svg) |
| U+2FCD | KANGXI RADICAL TRIPOD | ![size=24](/assets/img/genglyphsvg/u2fcd.svg) | U+9F0E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f0e.svg) |
| U+2FCE | KANGXI RADICAL DRUM | ![size=24](/assets/img/genglyphsvg/u2fce.svg) | U+9F13 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f13.svg) |
| U+2FCF | KANGXI RADICAL RAT | ![size=24](/assets/img/genglyphsvg/u2fcf.svg) | U+9F20 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f20.svg) |
| U+2FD0 | KANGXI RADICAL NOSE | ![size=24](/assets/img/genglyphsvg/u2fd0.svg) | U+9F3B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f3b.svg) |
| U+2FD1 | KANGXI RADICAL EVEN | ![size=24](/assets/img/genglyphsvg/u2fd1.svg) | U+9F4A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f4a.svg) |
| U+2FD2 | KANGXI RADICAL TOOTH | ![size=24](/assets/img/genglyphsvg/u2fd2.svg) | U+9F52 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f52.svg) |
| U+2FD3 | KANGXI RADICAL DRAGON | ![size=24](/assets/img/genglyphsvg/u2fd3.svg) | U+9F8D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f8d.svg) |
| U+2FD4 | KANGXI RADICAL TURTLE | ![size=24](/assets/img/genglyphsvg/u2fd4.svg) | U+9F9C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f9c.svg) |
| U+2FD5 | KANGXI RADICAL FLUTE | ![size=24](/assets/img/genglyphsvg/u2fd5.svg) | U+9FA0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9fa0.svg) |
| U+2E80 | CJK RADICAL REPEAT | ![size=24](/assets/img/genglyphsvg/u2e80.svg) | U+51AB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u51ab.svg) |
| U+2E81 | CJK RADICAL CLIFF | ![size=24](/assets/img/genglyphsvg/u2e81.svg) | U+5382 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5382.svg) |
| U+2E82 | CJK RADICAL SECOND ONE | ![size=24](/assets/img/genglyphsvg/u2e82.svg) | U+4E5B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e5b.svg) |
| U+2E83 | CJK RADICAL SECOND TWO | ![size=24](/assets/img/genglyphsvg/u2e83.svg) | U+4E5A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e5a.svg) |
| U+2E84 | CJK RADICAL SECOND THREE | ![size=24](/assets/img/genglyphsvg/u2e84.svg) | U+4E59 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e59.svg) |
| U+2E85 | CJK RADICAL PERSON | ![size=24](/assets/img/genglyphsvg/u2e85.svg) | U+4EBB | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4ebb.svg) |
| U+2E86 | CJK RADICAL BOX | ![size=24](/assets/img/genglyphsvg/u2e86.svg) | U+5182 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5182.svg) |
| U+2E89 | CJK RADICAL KNIFE TWO | ![size=24](/assets/img/genglyphsvg/u2e89.svg) | U+5202 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5202.svg) |
| U+2E8A | CJK RADICAL DIVINATION | ![size=24](/assets/img/genglyphsvg/u2e8a.svg) | U+535C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u535c.svg) |
| U+2E8B | CJK RADICAL SEAL | ![size=24](/assets/img/genglyphsvg/u2e8b.svg) | U+353E | CJK Ideograph Extension A | ![size=24](/assets/img/genglyphsvg/u353e.svg) |
| U+2E8E | CJK RADICAL LAME ONE | ![size=24](/assets/img/genglyphsvg/u2e8e.svg) | U+5140 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5140.svg) |
| U+2E8F | CJK RADICAL LAME TWO | ![size=24](/assets/img/genglyphsvg/u2e8f.svg) | U+5C23 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c23.svg) |
| U+2E90 | CJK RADICAL LAME THREE | ![size=24](/assets/img/genglyphsvg/u2e90.svg) | U+5C22 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5c22.svg) |
| U+2E92 | CJK RADICAL SNAKE | ![size=24](/assets/img/genglyphsvg/u2e92.svg) | U+5DF3 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5df3.svg) |
| U+2E93 | CJK RADICAL THREAD | ![size=24](/assets/img/genglyphsvg/u2e93.svg) | U+5E7A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5e7a.svg) |
| U+2E94 | CJK RADICAL SNOUT ONE | ![size=24](/assets/img/genglyphsvg/u2e94.svg) | U+5F51 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f51.svg) |
| U+2E95 | CJK RADICAL SNOUT TWO | ![size=24](/assets/img/genglyphsvg/u2e95.svg) | U+5F50 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5f50.svg) |
| U+2E96 | CJK RADICAL HEART ONE | ![size=24](/assets/img/genglyphsvg/u2e96.svg) | U+5FC4 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5fc4.svg) |
| U+2E97 | CJK RADICAL HEART TWO | ![size=24](/assets/img/genglyphsvg/u2e97.svg) | U+38FA | CJK Ideograph Extension A | ![size=24](/assets/img/genglyphsvg/u38fa.svg) |
| U+2E98 | CJK RADICAL HAND | ![size=24](/assets/img/genglyphsvg/u2e98.svg) | U+624C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u624c.svg) |
| U+2E99 | CJK RADICAL RAP | ![size=24](/assets/img/genglyphsvg/u2e99.svg) | U+6535 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6535.svg) |
| U+2E9B | CJK RADICAL CHOKE | ![size=24](/assets/img/genglyphsvg/u2e9b.svg) | U+65E1 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u65e1.svg) |
| U+2E9D | CJK RADICAL MOON | ![size=24](/assets/img/genglyphsvg/u2e9d.svg) | U+6708 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6708.svg) |
| U+2E9E | CJK RADICAL DEATH | ![size=24](/assets/img/genglyphsvg/u2e9e.svg) | U+6B7A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6b7a.svg) |
| U+2E9F | CJK RADICAL MOTHER | ![size=24](/assets/img/genglyphsvg/u2e9f.svg) | U+6BCD | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6bcd.svg) |
| U+2EA0 | CJK RADICAL CIVILIAN | ![size=24](/assets/img/genglyphsvg/u2ea0.svg) | U+6C11 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c11.svg) |
| U+2EA1 | CJK RADICAL WATER ONE | ![size=24](/assets/img/genglyphsvg/u2ea1.svg) | U+6C35 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c35.svg) |
| U+2EA2 | CJK RADICAL WATER TWO | ![size=24](/assets/img/genglyphsvg/u2ea2.svg) | U+6C3A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6c3a.svg) |
| U+2EA3 | CJK RADICAL FIRE | ![size=24](/assets/img/genglyphsvg/u2ea3.svg) | U+706C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u706c.svg) |
| U+2EA5 | CJK RADICAL PAW TWO | ![size=24](/assets/img/genglyphsvg/u2ea5.svg) | U+722B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u722b.svg) |
| U+2EA6 | CJK RADICAL SIMPLIFIED HALF TREE TRUNK | ![size=24](/assets/img/genglyphsvg/u2ea6.svg) | U+4E2C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e2c.svg) |
| U+2EA8 | CJK RADICAL DOG | ![size=24](/assets/img/genglyphsvg/u2ea8.svg) | U+72AD | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u72ad.svg) |
| U+2EAB | CJK RADICAL EYE | ![size=24](/assets/img/genglyphsvg/u2eab.svg) | U+7F52 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7f52.svg) |
| U+2EAC | CJK RADICAL SPIRIT ONE | ![size=24](/assets/img/genglyphsvg/u2eac.svg) | U+793A | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u793a.svg) |
| U+2EAD | CJK RADICAL SPIRIT TWO | ![size=24](/assets/img/genglyphsvg/u2ead.svg) | U+793B | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u793b.svg) |
| U+2EAF | CJK RADICAL SILK | ![size=24](/assets/img/genglyphsvg/u2eaf.svg) | U+7CF9 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7cf9.svg) |
| U+2EB0 | CJK RADICAL C-SIMPLIFIED SILK | ![size=24](/assets/img/genglyphsvg/u2eb0.svg) | U+7E9F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7e9f.svg) |
| U+2EB1 | CJK RADICAL NET ONE | ![size=24](/assets/img/genglyphsvg/u2eb1.svg) | U+7F53 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7f53.svg) |
| U+2EB3 | CJK RADICAL NET THREE | ![size=24](/assets/img/genglyphsvg/u2eb3.svg) | U+34C1 | CJK Ideograph Extension A | ![size=24](/assets/img/genglyphsvg/u34c1.svg) |
| U+2EB4 | CJK RADICAL NET FOUR | ![size=24](/assets/img/genglyphsvg/u2eb4.svg) | U+34C1 | CJK Ideograph Extension A | ![size=24](/assets/img/genglyphsvg/u34c1.svg) |
| U+2EB9 | CJK RADICAL OLD | ![size=24](/assets/img/genglyphsvg/u2eb9.svg) | U+8002 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8002.svg) |
| U+2EBA | CJK RADICAL BRUSH ONE | ![size=24](/assets/img/genglyphsvg/u2eba.svg) | U+8080 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8080.svg) |
| U+2EBC | CJK RADICAL MEAT | ![size=24](/assets/img/genglyphsvg/u2ebc.svg) | U+6708 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6708.svg) |
| U+2EBD | CJK RADICAL MORTAR | ![size=24](/assets/img/genglyphsvg/u2ebd.svg) | U+81FC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u81fc.svg) |
| U+2EBE | CJK RADICAL GRASS ONE | ![size=24](/assets/img/genglyphsvg/u2ebe.svg) | U+8279 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8279.svg) |
| U+2EBF | CJK RADICAL GRASS TWO | ![size=24](/assets/img/genglyphsvg/u2ebf.svg) | U+8279 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8279.svg) |
| U+2EC0 | CJK RADICAL GRASS THREE | ![size=24](/assets/img/genglyphsvg/u2ec0.svg) | U+8279 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8279.svg) |
| U+2EC1 | CJK RADICAL TIGER | ![size=24](/assets/img/genglyphsvg/u2ec1.svg) | U+864E | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u864e.svg) |
| U+2EC2 | CJK RADICAL CLOTHES | ![size=24](/assets/img/genglyphsvg/u2ec2.svg) | U+8864 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8864.svg) |
| U+2EC3 | CJK RADICAL WEST ONE | ![size=24](/assets/img/genglyphsvg/u2ec3.svg) | U+8980 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8980.svg) |
| U+2EC4 | CJK RADICAL WEST TWO | ![size=24](/assets/img/genglyphsvg/u2ec4.svg) | U+897F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u897f.svg) |
| U+2EC5 | CJK RADICAL C-SIMPLIFIED SEE | ![size=24](/assets/img/genglyphsvg/u2ec5.svg) | U+89C1 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u89c1.svg) |
| U+2EC8 | CJK RADICAL C-SIMPLIFIED SPEECH | ![size=24](/assets/img/genglyphsvg/u2ec8.svg) | U+8BA0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8ba0.svg) |
| U+2EC9 | CJK RADICAL C-SIMPLIFIED SHELL | ![size=24](/assets/img/genglyphsvg/u2ec9.svg) | U+8D1D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8d1d.svg) |
| U+2ECB | CJK RADICAL C-SIMPLIFIED CART | ![size=24](/assets/img/genglyphsvg/u2ecb.svg) | U+8F66 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8f66.svg) |
| U+2ECD | CJK RADICAL WALK ONE | ![size=24](/assets/img/genglyphsvg/u2ecd.svg) | U+8FB6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u8fb6.svg) |
| U+2ECF | CJK RADICAL CITY | ![size=24](/assets/img/genglyphsvg/u2ecf.svg) | U+961D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u961d.svg) |
| U+2ED0 | CJK RADICAL C-SIMPLIFIED GOLD | ![size=24](/assets/img/genglyphsvg/u2ed0.svg) | U+9485 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9485.svg) |
| U+2ED1 | CJK RADICAL LONG ONE | ![size=24](/assets/img/genglyphsvg/u2ed1.svg) | U+9577 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9577.svg) |
| U+2ED2 | CJK RADICAL LONG TWO | ![size=24](/assets/img/genglyphsvg/u2ed2.svg) | U+9578 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9578.svg) |
| U+2ED3 | CJK RADICAL C-SIMPLIFIED LONG | ![size=24](/assets/img/genglyphsvg/u2ed3.svg) | U+957F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u957f.svg) |
| U+2ED6 | CJK RADICAL MOUND TWO | ![size=24](/assets/img/genglyphsvg/u2ed6.svg) | U+961D | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u961d.svg) |
| U+2ED8 | CJK RADICAL BLUE | ![size=24](/assets/img/genglyphsvg/u2ed8.svg) | U+9752 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9752.svg) |
| U+2ED9 | CJK RADICAL C-SIMPLIFIED TANNED LEATHER | ![size=24](/assets/img/genglyphsvg/u2ed9.svg) | U+97E6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u97e6.svg) |
| U+2EDA | CJK RADICAL C-SIMPLIFIED LEAF | ![size=24](/assets/img/genglyphsvg/u2eda.svg) | U+9875 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9875.svg) |
| U+2EDB | CJK RADICAL C-SIMPLIFIED WIND | ![size=24](/assets/img/genglyphsvg/u2edb.svg) | U+98CE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98ce.svg) |
| U+2EDC | CJK RADICAL C-SIMPLIFIED FLY | ![size=24](/assets/img/genglyphsvg/u2edc.svg) | U+98DE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98de.svg) |
| U+2EDD | CJK RADICAL EAT ONE | ![size=24](/assets/img/genglyphsvg/u2edd.svg) | U+98DF | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98df.svg) |
| U+2EDF | CJK RADICAL EAT THREE | ![size=24](/assets/img/genglyphsvg/u2edf.svg) | U+98E0 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u98e0.svg) |
| U+2EE0 | CJK RADICAL C-SIMPLIFIED EAT | ![size=24](/assets/img/genglyphsvg/u2ee0.svg) | U+9963 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9963.svg) |
| U+2EE2 | CJK RADICAL C-SIMPLIFIED HORSE | ![size=24](/assets/img/genglyphsvg/u2ee2.svg) | U+9A6C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9a6c.svg) |
| U+2EE3 | CJK RADICAL BONE | ![size=24](/assets/img/genglyphsvg/u2ee3.svg) | U+9AA8 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9aa8.svg) |
| U+2EE4 | CJK RADICAL GHOST | ![size=24](/assets/img/genglyphsvg/u2ee4.svg) | U+9B3C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9b3c.svg) |
| U+2EE5 | CJK RADICAL C-SIMPLIFIED FISH | ![size=24](/assets/img/genglyphsvg/u2ee5.svg) | U+9C7C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9c7c.svg) |
| U+2EE6 | CJK RADICAL C-SIMPLIFIED BIRD | ![size=24](/assets/img/genglyphsvg/u2ee6.svg) | U+9E1F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9e1f.svg) |
| U+2EE7 | CJK RADICAL C-SIMPLIFIED SALT | ![size=24](/assets/img/genglyphsvg/u2ee7.svg) | U+5364 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u5364.svg) |
| U+2EE8 | CJK RADICAL SIMPLIFIED WHEAT | ![size=24](/assets/img/genglyphsvg/u2ee8.svg) | U+9EA6 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ea6.svg) |
| U+2EE9 | CJK RADICAL SIMPLIFIED YELLOW | ![size=24](/assets/img/genglyphsvg/u2ee9.svg) | U+9EC4 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9ec4.svg) |
| U+2EEA | CJK RADICAL C-SIMPLIFIED FROG | ![size=24](/assets/img/genglyphsvg/u2eea.svg) | U+9EFE | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9efe.svg) |
| U+2EEB | CJK RADICAL J-SIMPLIFIED EVEN | ![size=24](/assets/img/genglyphsvg/u2eeb.svg) | U+6589 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6589.svg) |
| U+2EEC | CJK RADICAL C-SIMPLIFIED EVEN | ![size=24](/assets/img/genglyphsvg/u2eec.svg) | U+9F50 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f50.svg) |
| U+2EED | CJK RADICAL J-SIMPLIFIED TOOTH | ![size=24](/assets/img/genglyphsvg/u2eed.svg) | U+6B6F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u6b6f.svg) |
| U+2EEE | CJK RADICAL C-SIMPLIFIED TOOTH | ![size=24](/assets/img/genglyphsvg/u2eee.svg) | U+9F7F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f7f.svg) |
| U+2EEF | CJK RADICAL J-SIMPLIFIED DRAGON | ![size=24](/assets/img/genglyphsvg/u2eef.svg) | U+7ADC | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u7adc.svg) |
| U+2EF0 | CJK RADICAL C-SIMPLIFIED DRAGON | ![size=24](/assets/img/genglyphsvg/u2ef0.svg) | U+9F99 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f99.svg) |
| U+2EF1 | CJK RADICAL TURTLE | ![size=24](/assets/img/genglyphsvg/u2ef1.svg) | U+9F9C | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f9c.svg) |
| U+2EF2 | CJK RADICAL J-SIMPLIFIED TURTLE | ![size=24](/assets/img/genglyphsvg/u2ef2.svg) | U+4E80 | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u4e80.svg) |
| U+2EF3 | CJK RADICAL C-SIMPLIFIED TURTLE | ![size=24](/assets/img/genglyphsvg/u2ef3.svg) | U+9F9F | CJK Ideograph | ![size=24](/assets/img/genglyphsvg/u9f9f.svg) |

