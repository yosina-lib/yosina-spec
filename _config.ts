import lume from "lume/mod.ts";
import modifyUrls from "lume/plugins/modify_urls.ts";
import wiki from "wiki/mod.ts";

import { default as genGlyphSvgPlugin } from "@tools/genglyphsvg/plugin.mts";
import type { Options as GenSvgOptions } from "@tools/genglyphsvg/plugin.mts";

const languages = ["ja", "en"];
const urlPrefix = Deno.env.get("YOSINA_SPEC_URL_PREFIX") ?? "/spec";
const enableGenGlyphSvg = Boolean(
  Deno.env.get("YOSINA_ENABLE_GENGLYPHSVG") ?? false,
);
const langPrefixedRegexp = new RegExp(
  `^(?:/(${
    languages.map(RegExp.escape).join("|")
  }))?(/(?<!(?:assets|_includes|pagefind)/).*)?$`,
);
const menuFileRegexp = new RegExp(
  `^/menu-(${languages.map(RegExp.escape).join("|")})\.json$`,
);

export default lume({
  src: "./src",
  dest: "./_site",
  location: new URL(
    Deno.env.get("YOSINA_SPEC_BASE_PATH") ?? "",
    "https://yosina.github.io",
  ),
}, {
  markdown: {
    plugins: [
      ...(enableGenGlyphSvg
        ? [
          [
            genGlyphSvgPlugin,
            {
              bin: "./target/debug/genglyphsvg",
              fontFiles: [
                "./tmp/SourceHanSerif-Regular.otf",
                "./tmp/NotoSerif-Regular.ttf",
                "./tmp/NotoSansMath-Regular.ttf",
                "./tmp/Noto-COLRv1.ttf",
                "./tmp/NotoSansSymbols2-Regular.ttf",
              ],
              outDir: "./src/assets/img/genglyphsvg",
              baseUrl: "/assets/img/genglyphsvg",
            } as GenSvgOptions,
          ],
        ]
        : []),
    ],
  },
})
  .ignore(
    (path) => path.endsWith(".md.njk"),
    "**/*.rs",
  )
  .use(wiki({
    languages: languages,
  }))
  .process([".html"], (pages, _) => {
    for (const page of pages) {
      if (!page.outputPath.endsWith(".html")) {
        continue;
      }

      const m = langPrefixedRegexp.exec(page.data.url);
      if (m !== null && m[2] !== undefined) {
        const newUrl = m[1] === undefined
          ? `/${languages[0]}${urlPrefix}${m[2]}`
          : `/${m[1]}${urlPrefix}${m[2]}`;
        page.data.url = newUrl;
      }
    }
  })
  .process([".json"], (pages, _) => {
    for (const page of pages) {
      if (!menuFileRegexp.test(page.data.url)) {
        continue;
      }
      const content = JSON.parse(page.text);
      type MenuNode = {
        data: {
          url?: string;
        };
        children?: MenuNode[];
      };
      const processContent = (items: MenuNode[]): MenuNode[] =>
        items.map((item) => {
          let newItem: MenuNode | undefined;
          if (item.data.url !== undefined) {
            const m = langPrefixedRegexp.exec(item.data.url);
            if (m !== null && m[2] !== undefined) {
              const newUrl = `/${(m[1] ?? languages[0])}${urlPrefix}${m[2]}`;
              newItem = {
                ...item,
                data: {
                  ...item.data,
                  url: newUrl,
                },
              };
            }
          }
          if (newItem === undefined) {
            if (item.children === undefined) {
              return item;
            }
            newItem = { ...item };
          }
          if (newItem.children === undefined) {
            return newItem;
          }
          newItem.children = processContent(newItem.children);
          return newItem;
        });
      const newContent = JSON.stringify(processContent(content), undefined, 2);
      page.text = newContent;
    }
  })
  .use(modifyUrls({
    fn: (url) => {
      const m = langPrefixedRegexp.exec(url);
      if (!url.endsWith("/") && !url.endsWith(".html")) {
        return url;
      }
      if (m !== null && m[2] !== undefined) {
        return `/${(m[1] ?? languages[0])}${urlPrefix}${m[2]}`;
      }
      return url;
    },
  }))
  .copy("./assets");
