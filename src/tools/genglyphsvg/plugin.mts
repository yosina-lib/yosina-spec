import type { default as MarkdownIt } from "markdown-it";
import type { default as StateCore } from "markdown-it/lib/rules_core/state_core.mjs";
import { default as Token } from "npm:markdown-it/lib/token.mjs";
import { escapeHtml } from "npm:markdown-it/lib/common/utils.mjs"

export type Options = {
  bin: string;
  fontFiles: string[];
  outDir: string;
  baseUrl: string;
};

const uniCodepointRegex = /^u([0-9A-Fa-f]+)$/;

type TraverseContext = Options & {
  sourceLastModified: Date;
};

const parseCodepoints = (ctx: TraverseContext, v: string): number[] | undefined => {
  if (!v.startsWith(ctx.baseUrl) || !v.endsWith(".svg") || v[ctx.baseUrl.length] !== '/') {
    return undefined;
  }
  v = v.slice(ctx.baseUrl.length + 1, v.length - 4);
  return v.split(/_+/).map((cp) => {
    const m = cp.trim().match(uniCodepointRegex);
    if (m === null) {
      throw new Error(`invalid codepoint string: ${cp}`);
    }
    const v = Number.parseInt(m[1], 16);
    if (isNaN(v)) {
      throw new Error(`invalid codepoint string: ${cp}`);
    }
    return v;
  });
};

const parseAttributes = (v: string): Record<string, string | undefined> =>
  Object.fromEntries(v.split(/\s+/).map((c) => (c.split(/\s*=\s*/, 2).map((x) => x.trim()) as [string, string | undefined])));

const lastComponent = (v: string): string | undefined => {
  const components = v.split("/");
  const last = components[components.length - 1];
  return last === "" ? undefined : components[components.length - 1];
};

const handleToken = (
  ctx: TraverseContext,
  token: Token,
): [Token[], boolean] => {
  if (token.type !== "image" || token.attrs === null) {
    return [[token], false];
  }
  const attrs = Object.fromEntries(token.attrs) as {
    src?: string;
  };
  if (attrs?.src === undefined) {
    return [[token], false];
  }
  const assetUrl = attrs?.src;
  const codepoints = parseCodepoints(ctx, assetUrl);
  if (codepoints === undefined) {
    return [[token], false];
  }
  const origAlt = token.children?.[0]?.content;
  const altAttrs = origAlt !== undefined ? parseAttributes(origAlt) as { size?: string }: undefined;
  const size = Number.parseFloat(altAttrs?.size ?? "12.");
  const filename = lastComponent(assetUrl);
  if (filename === undefined ) {
    return [[token], false];
  }
  const outPath = `${ctx.outDir}/${filename}`;
  let outPathStat = undefined;
  try {
    outPathStat = Deno.statSync(outPath);
  } catch (_e) {
    // ignore
  }
  const alt = String.fromCodePoint(...codepoints);
  const newTokens = [
    Object.assign(new Token("html_inline", "svg", 1), {
      content: `<svg aria-label="${escapeHtml(alt)}" width="${size}" height="${size}"><use href="${assetUrl.split('/').map(encodeURIComponent).join('/')}" /></svg>`,
    }),
  ];
  if (
    outPathStat?.mtime != null && outPathStat.mtime >= ctx.sourceLastModified
  ) {
    return [newTokens, true];
  }
  const args = [
    ...ctx.fontFiles.flatMap((fontFile) => [
      "--font",
      fontFile,
    ]),
    "--output",
    outPath,
    "--size",
    String(size),
    codepoints.map((u) => `U+${u.toString(16)}`).join(" "),
  ];
  const cmd = new Deno.Command(ctx.bin, {
    args: args,
    stdin: "inherit",
    stdout: "inherit",
    stderr: "piped",
  });
  const { stderr, code } = cmd.outputSync();
  if (code !== 0) {
    console.error(
      `child process return with status ${code}: ${
        new TextDecoder().decode(stderr)
      } where command is ${ctx.bin} ${args.join(" ")}`,
    );
  }
  return [newTokens, true];
};

const traverse = (ctx: TraverseContext, tokens: Token[]): boolean => {
  let handled = false;
  const newTokens = tokens.flatMap((token) => {
    const [tokens_, handled_] = handleToken(ctx, token);
    handled ||= handled_;
    for (const descendant of tokens_) {
      if (descendant.children) {
        if (traverse(ctx, descendant.children)) {
          handled = true;
        }
      }
    }
    return tokens_;
  });
  tokens.splice(
    0,
    tokens.length,
    ...newTokens,
  );
  return handled;
};

export default (md: MarkdownIt, options: Partial<Options>) => {
  if (options?.bin === undefined) {
    throw new Error("genglyphsvg executable not specified.");
  }
  if (options?.fontFiles === undefined || options.fontFiles.length === 0) {
    throw new Error("font file not specified.");
  }
  if (options?.outDir === undefined) {
    throw new Error("output directory not specified.");
  }
  if (options?.baseUrl === undefined) {
    throw new Error("base URL not specified.");
  }
  md.core.ruler.after("inline", "genglyphsvg", (state: StateCore) => {
    const data = state.tokens;
    if (data === undefined) {
      return;
    }
    const sourceLastModified =
      Deno.statSync(state.env.data.page.src.entry.src).mtime;
    if (sourceLastModified === null) {
      throw new Error(`failed to retrieve last modified time of ${state.src}`);
    }
    traverse({
      ...options,
      sourceLastModified,
    }, data);
  });
};
