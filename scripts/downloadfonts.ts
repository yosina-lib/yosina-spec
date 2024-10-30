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
  const destFile = await Deno.create(destPath);
  let failed: Error | undefined = undefined;
  try {
    const resp = await fetch(artifact.url, { redirect: "follow" });
    if (resp.status === 200) {
      console.log(`Downloading ${artifact.url} and extract it to ${destPath}`);
    }
    const body = resp.body;
    if (body === null) {
      throw new Error(`Response body is null for ${artifact.url}`);
    }
    let targetEntry: Entry | undefined = undefined;
    for await (const entry of body.pipeThrough(new ZipReaderStream())) {
      if (entry.filename === artifact.pathInArchive) {
        console.log(`Extracting ${artifact.pathInArchive} to ${destPath}`);
        await entry.readable.pipeTo(destFile.writable);
        console.log('Done');
        targetEntry = entry;
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
  }
  if (failed !== undefined) {
    await Deno.remove(destPath);
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
  ].map((artifact) => downloadAndExpand(destDir, artifact)));
} finally {
  await terminateWorkers();
}