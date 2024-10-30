import lume from "lume/mod.ts";
import multilanguage from "lume/plugins/multilanguage.ts";
import lumocs from "lumocs/mod.ts";

import { default as genGlyphSvgPlugin } from "@tools/genglyphsvg/plugin.mts";
import type { Options as GenSvgOptions } from "@tools/genglyphsvg/plugin.mts";

export default lume({
  src: "./src",
  dest: "./_site",
}, {
  markdown: {
    plugins: [
      [
        genGlyphSvgPlugin,
        {
          bin: "./target/debug/genglyphsvg",
          fontFiles: [
            "./tmp/SourceHanSerif-Regular.otf",
            "./tmp/NotoSerif-Regular.ttf",
            "./tmp/NotoSansMath-Regular.ttf",
            "./tmp/NotoEmoji-Regular.ttf",
          ],
          outDir: "./src/assets/img/genglyphsvg",
          baseUrl: "/assets/img/genglyphsvg",
        } as GenSvgOptions,
      ],
    ],
  },
})
  .ignore((path) => path.endsWith(".md.njk"))
  .use(lumocs())
  .use(multilanguage({
    defaultLanguage: "en",
    languages: ["ja", "en"],
  }))
  .copy("./assets");
