/**
 * Downloads font files needed for the project from their official sources.
 * 
 * This script downloads:
 * - SourceHanSerif-Regular.otf - Japanese serif font from Adobe
 * - NotoSansMath-Regular.ttf - Mathematical symbols font from Google
 * - NotoSerif-Regular.ttf - Serif font from Google
 * - NotoColorEmoji.ttf - Color emoji font from Google
 * - NotoSansSymbols2-Regular.ttf - Additional symbols from Google
 */
import { Entry } from "lume/core/fs.ts";
import * as path from "jsr:@std/path";
import { configure, terminateWorkers, ZipReaderStream } from "jsr:@zip-js/zip-js";

type ArtifactDescriptor = {
  url: string;
  filename: string;
  pathInArchive: string;
};

const downloadAndExpand = async (
  destDir: string,
  artifact: ArtifactDescriptor,
): Promise<void> => {
  const destPath = path.join(destDir, artifact.filename);
  await Deno.mkdir(destDir, { recursive: true });
  
  // Check if file already exists
  try {
    const stat = await Deno.stat(destPath);
    if (stat.isFile && stat.size > 0) {
      console.log(`Skipping ${artifact.filename} - already exists`);
      return;
    }
  } catch {
    // File doesn't exist, continue
  }
  
  const destFile = await Deno.create(destPath);
  let failed: Error | undefined = undefined;
  try {
    console.log(`Downloading ${artifact.url}`);
    const resp = await fetch(artifact.url, { 
      redirect: "follow",
      signal: AbortSignal.timeout(300000) // 5 minute timeout for large files
    });
    
    if (resp.status !== 200) {
      throw new Error(`HTTP ${resp.status} for ${artifact.url}`);
    }
    
    const body = resp.body;
    if (body === null) {
      throw new Error(`Response body is null for ${artifact.url}`);
    }
    
    let targetEntry: Entry | undefined = undefined;
    
    const zipStream = body.pipeThrough(new ZipReaderStream());
    
    for await (const entry of zipStream) {
      if (entry.filename === artifact.pathInArchive) {
        console.log(`Extracting ${artifact.pathInArchive} to ${destPath}`);
        await entry.readable.pipeTo(destFile.writable);
        console.log(`Done: ${artifact.filename}`);
        targetEntry = entry;
        // Don't break - let the stream finish naturally
      } else {
        // Consume and discard the data
        for await (const _ of entry.readable) {
          // Just consume the data
        }
      }
    }
    
    if (targetEntry === undefined) {
      throw new Error(`Entry ${artifact.pathInArchive} not found in archive`);
    }
  } catch (e) {
    if (e instanceof Error) {
      failed = e;
    } else {
      throw e;
    }
  } finally {
    try {
      await destFile.close();
    } catch {
      // Ignore close errors
    }
  }
  
  if (failed !== undefined) {
    console.error(`Failed to download ${artifact.filename}: ${failed.message}`);
    try {
      await Deno.remove(destPath);
    } catch {
      // Ignore removal errors
    }
    throw failed;
  }
};

const destDir = "./tmp";

configure({ useWebWorkers: false });

try {
  await Promise.all([
    {
      url: "https://github.com/adobe-fonts/source-han-serif/releases/download/2.003R/04_SourceHanSerifOTF.zip",
      filename: "SourceHanSerif-Regular.otf",
      pathInArchive: "OTF/Japanese/SourceHanSerif-Regular.otf",
    },
    {
      url: "https://github.com/notofonts/math/releases/download/NotoSansMath-v3.000/NotoSansMath-v3.000.zip",
      filename: "NotoSansMath-Regular.ttf",
      pathInArchive: "NotoSansMath/full/ttf/NotoSansMath-Regular.ttf",
    },
    {
      url: "https://github.com/notofonts/latin-greek-cyrillic/releases/download/NotoSerif-v2.015/NotoSerif-v2.015.zip",
      filename: "NotoSerif-Regular.ttf",
      pathInArchive: "NotoSerif/hinted/ttf/NotoSerif-Regular.ttf",
    },
    {
      url: "https://github.com/googlefonts/noto-emoji/archive/refs/tags/v2.042.zip",
      filename: "Noto-COLRv1.ttf",
      pathInArchive: "noto-emoji-2.042/fonts/Noto-COLRv1.ttf",
    },
    {
      url: "https://github.com/notofonts/symbols/releases/download/NotoSansSymbols2-v2.008/NotoSansSymbols2-v2.008.zip",
      filename: "NotoSansSymbols2-Regular.ttf",
      pathInArchive: "NotoSansSymbols2/hinted/ttf/NotoSansSymbols2-Regular.ttf",
    },
  ].map((artifact) => downloadAndExpand(destDir, artifact)));
} finally {
  await terminateWorkers();
}