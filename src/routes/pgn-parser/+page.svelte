<script>
import {goto} from "$app/navigation";
import {Button} from "$components/ui/button/index.js";

import { open } from "@tauri-apps/plugin-dialog";
import { readTextFile } from "@tauri-apps/plugin-fs";
import {invoke} from "@tauri-apps/api/core";

let fileContent = "";
let resultMap;

async function readFile(path) {
    const result = await readTextFile(path);
    console.log(result);
}
const openFile = async () => {
  const result = await open({
    defaultPath: ".",
    filters: [{ name: "PGN Files", extensions: ["pgn"] }],
    multiple: false,
      directory: false,
  });

  if (result) {
    console.log(result);
    await(readFile(result.path));

    resultMap = result;
  }
};

async function openAndReadFile() {
    const result = await open({
        defaultPath: ".",
        filters: [{ name: "PGN Files", extensions: ["pgn"] }],
        multiple: false,
    });

    if (result) {
        fileContent = await invoke('read_file', { filename: result.path });
    }
}
</script>

<h1 class="text-center">PGN Parser</h1>
<Button href="/">Go to HOME</Button>

<br />
<br />
<Button on:click={openFile}>Open file</Button>
{#if resultMap}
    <pre>{JSON.stringify(resultMap, null, 2)}</pre>
{/if}
<br />
<br />
<Button on:click={openAndReadFile}>Open file via Rust</Button>
<br />
{#if fileContent}
    <pre>{fileContent}</pre>
{/if}