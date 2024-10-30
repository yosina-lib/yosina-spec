import { Environment, FileSystemLoader } from "npm:nunjucks";
import { expandGlobSync } from "jsr:@std/fs";
import { parse as parsePath } from "jsr:@std/path";
import { default as unicodeNames } from "npm:@unicode/unicode-16.0.0/Names/index.js";

type Dataset = {
  spaces: string;
  cjkRadicals: string;
  ideographicAnnotationMarks: string;
  mathematicalAlphanumerics: string;
  hyphens: string;
  traditionalToNewKanjis: string;
  ivsSvsBaseMappings: string;
  combinedChars: string;
  circledOrSquared: string;
  romanNumerals: string;
};

const dataset: Dataset = {
  spaces: "./data/spaces.json",
  cjkRadicals: "./data/radicals.json",
  ideographicAnnotationMarks: "./data/ideographic-annotation-marks.json",
  mathematicalAlphanumerics: "./data/mathematical-alphanumerics.json",
  hyphens: "./data/hyphens.json",
  traditionalToNewKanjis: "./data/kanji-old-new-form.json",
  ivsSvsBaseMappings: "./data/ivs-svs-base-mappings.json",
  combinedChars: "./data/combined-chars.json",
  circledOrSquared: "./data/circled-or-squared.json",
  romanNumerals: "./data/roman-numerals.json",
};

type SimpleMappings = Record<string, string>;

type HyphensMapping = {
  code: string;
  name: string;
  ascii: string[] | null;
  shift_jis: number[];
  jisx0201: string[];
  "jisx0208-1978": string[];
  "jisx0208-1978-windows": string[];
  "jisx0208-verbatim": string | null;
};

type CodepointWithIVSSVS = {
  ivs: string[] | null;
  svs: string[] | null;
};

type TraditionalToNewKanjisMapping = [CodepointWithIVSSVS, CodepointWithIVSSVS];

type CodepointWithIVSSVSAndBase = CodepointWithIVSSVS & {
  base: string | null;
};

type TraditionalToNewKanjisMappingWithBase = [
  CodepointWithIVSSVSAndBase,
  CodepointWithIVSSVSAndBase,
];

type IvsSvsBaseMapping = {
  ivs: string[];
  svs: string[] | null;
  base90: string | null;
  base2004: string | null;
};

type CircledOrSquaredMapping = {
  rendering: string;
  type: string;
  emoji: boolean;
};

type RomanNumeralsMapping = {
  value: number;
  codes: {
    upper: string;
    lower: string;
  };
  shift_jis: {
    upper: number[];
    lower: number[];
  };
  decomposed: {
    upper: string[];
    lower: string[];
  };
};

const baseCharmap: "base90" | "base2004" = "base2004";

const dataLoaders = (dataset: Dataset) => ({
  _baseCharmap: baseCharmap,
  _cjkRadicals: undefined as SimpleMappings | undefined,
  _ideographicAnnotationMarks: undefined as SimpleMappings | undefined,
  _mathematicalAlphanumerics: undefined as SimpleMappings | undefined,
  _spaces: undefined as SimpleMappings | undefined,
  _hyphens: undefined as HyphensMapping[] | undefined,
  _traditionalToNewKanjis: undefined as
    | TraditionalToNewKanjisMappingWithBase[]
    | undefined,
  _ivsSvsBaseMappings: undefined as
    | Record<string, IvsSvsBaseMapping>
    | undefined,
  _combinedChars: undefined as SimpleMappings | undefined,
  _circledOrSquared: undefined as Record<string, CircledOrSquaredMapping> | undefined,
  _romanNumerals: undefined as RomanNumeralsMapping[] | undefined,

  get cjkRadicals() {
    if (this._cjkRadicals === undefined) {
      this._cjkRadicals = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.cjkRadicals)),
      );
    }
    return this._cjkRadicals;
  },
  get ideographicAnnotationMarks() {
    if (this._ideographicAnnotationMarks === undefined) {
      this._ideographicAnnotationMarks = JSON.parse(
        new TextDecoder("utf-8").decode(
          Deno.readFileSync(dataset.ideographicAnnotationMarks),
        ),
      );
    }
    return this._ideographicAnnotationMarks;
  },
  get mathematicalAlphanumerics() {
    if (this._mathematicalAlphanumerics === undefined) {
      this._mathematicalAlphanumerics = JSON.parse(
        new TextDecoder("utf-8").decode(
          Deno.readFileSync(dataset.mathematicalAlphanumerics),
        ),
      );
    }
    return this._mathematicalAlphanumerics;
  },
  get spaces() {
    if (this._spaces === undefined) {
      this._spaces = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.spaces)),
      );
    }
    return this._spaces;
  },
  get hyphens() {
    if (this._hyphens === undefined) {
      this._hyphens = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.hyphens)),
      );
    }
    return this._hyphens;
  },
  get traditionalToNewKanjis() {
    if (this._traditionalToNewKanjis === undefined) {
      const traditionalToNewKanjis = JSON.parse(
        new TextDecoder("utf-8").decode(
          Deno.readFileSync(dataset.traditionalToNewKanjis),
        ),
      );
      const ivsSvsBaseMappings = this.ivsSvsBaseMappings;
      const appendBase = (
        m: CodepointWithIVSSVS,
      ): CodepointWithIVSSVSAndBase => (
        {
          base: (m.ivs !== null
            ? ivsSvsBaseMappings[m.ivs.join(" ")][baseCharmap]
            : undefined) ??
            (m.svs !== null
              ? ivsSvsBaseMappings[m.svs.join(" ")][baseCharmap]
              : undefined) ??
            null,
          ...m,
        }
      );
      this._traditionalToNewKanjis = traditionalToNewKanjis.map(
        (value: TraditionalToNewKanjisMapping) => {
          return [appendBase(value[0]), appendBase(value[1])];
        },
      );
    }
    return this._traditionalToNewKanjis;
  },
  get ivsSvsBaseMappings() {
    if (this._ivsSvsBaseMappings === undefined) {
      const ivsSvsBaseMappings = JSON.parse(
        new TextDecoder("utf-8").decode(
          Deno.readFileSync(dataset.ivsSvsBaseMappings),
        ),
      );
      this._ivsSvsBaseMappings = Object.fromEntries(
        ivsSvsBaseMappings.flatMap((value: IvsSvsBaseMapping) => [
          [
            value.ivs.join(" "),
            value,
          ],
          ...(
            value.svs !== null
              ? [
                [
                  value.svs.join(" "),
                  value,
                ]
              ]
              : []
          ),
        ]),
      );
    }
    return this._ivsSvsBaseMappings;
  },
  get combinedChars() {
    if (this._combinedChars === undefined) {
      this._combinedChars = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.combinedChars)),
      );
    }
    return this._combinedChars;
  },
  get circledOrSquared() {
    if (this._circledOrSquared === undefined) {
      this._circledOrSquared = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.circledOrSquared)),
      );
    }
    return this._circledOrSquared;
  },
  get romanNumerals() {
    if (this._romanNumerals === undefined) {
      this._romanNumerals = JSON.parse(
        new TextDecoder("utf-8").decode(Deno.readFileSync(dataset.romanNumerals)),
      );
    }
    return this._romanNumerals;
  },
  get unicodeNames(): Map<number, string> {
    return unicodeNames as unknown as Map<number, string>;
  },
});

const njkEnv = new Environment(new FileSystemLoader("."), {
  autoescape: false,
});

njkEnv.addFilter("decodeCodepoint", (v: string): number | undefined => {
  const m = /^U\+([0-9A-F]+)$/.exec(v.toUpperCase());
  if (m === null) {
    return undefined;
  }
  return Number.parseInt(m[1], 16);
});

njkEnv.addFilter("buildFilename", (v: string|string[]): string => {
  return (typeof v === 'string' ? [v] : v).map((s) => s.replace("+", "")).join("_").toLowerCase();
});

njkEnv.addFilter("codepointsToString", (codepoints: string[]): string => {
  return codepoints.map((cp) => {
    const match = /^U\+([0-9A-F]+)$/.exec(cp.toUpperCase());
    if (match) {
      const codePoint = Number.parseInt(match[1], 16);
      return String.fromCodePoint(codePoint);
    }
    return cp;
  }).join('');
});

const doIt = () => {
  const targets = expandGlobSync("./src/**/*.md.njk");
  for (const target of targets) {
    const parsed = parsePath(target.path);
    const dest = target.path.slice(0, target.path.length - parsed.ext.length);
    Deno.writeFileSync(
      dest,
      new TextEncoder().encode(
        njkEnv.getTemplate(target.path).render(dataLoaders(dataset)),
      ),
    );
  }
};

doIt();
