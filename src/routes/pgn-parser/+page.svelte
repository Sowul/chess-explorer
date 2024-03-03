<script>
import {goto} from "$app/navigation";
import {Button} from "$components/ui/button/index.js";

import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api/tauri";

let fileContent = "";

async function readFile(path) {
    const result = await readTextFile(path);
    console.log(result);
}
const openFile = async () => {
  const result = await open({
    defaultPath: ".",
    filters: [{ name: "PGN Files", extensions: ["pgn"] }],
    multiple: false,
  });

  if (result) {
    console.log(result);
    await(readFile(result));
  }
};

async function openAndReadFile() {
    const result = await open({
        defaultPath: ".",
        filters: [{ name: "PGN Files", extensions: ["pgn"] }],
        multiple: false,
    });

    if (result) {
        fileContent = await invoke('read_file', { filename: result });
    }
}
</script>

<h1 class="text-center">PGN Parser</h1>
<Button href="/">Go to HOME</Button>

<br />
<br />
<Button on:click={openFile}>Open file</Button>

<br />
<br />
<Button on:click={openAndReadFile}>Open file via Rust</Button>
<br />
{#if fileContent}
    {fileContent}
{/if}