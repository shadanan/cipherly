<script lang="ts">
  import { encodeUtf8 } from "$lib/cipherly";
  import FileDrop from "filedrop-svelte";
  import { filesize } from "filesize";
  import { FileText, HardDriveUpload, XCircle } from "lucide-svelte";
  import { Textarea } from "./ui/textarea";

  export let loading: boolean = false;
  export let data: Uint8Array = new Uint8Array();
  export let filename: string | null = null;
  export let placeholder: string;

  let text: string;
  let file: File | null;

  function resolveData() {
    if (!!text) {
      data = encodeUtf8(text);
      filename = null;
    } else if (!!file) {
      loading = true;
      file.arrayBuffer().then((v) => {
        data = new Uint8Array(v);
        loading = false;
      });
      filename = file.name;
    } else {
      data = new Uint8Array();
      filename = null;
    }
  }

  $: {
    text, file;
    resolveData();
  }
</script>

{#if file}
  <div
    class="border px-3 py-4 rounded-md flex bg-accent justify-between items-center"
  >
    <div class="flex items-center space-x-3">
      <FileText class="w-5 text-slate-400" />
      <div class="space-y-[-3px]">
        <div class="text-md">{file.name}</div>
        <div class="text-xs text-muted-foreground">
          {filesize(file.size)}
        </div>
      </div>
    </div>
    <button
      on:click={() => {
        file = null;
      }}
    >
      <XCircle class="w-5 text-red-800 cursor-pointer" />
    </button>
  </div>
{:else}
  <Textarea
    class="border-2 text-base text-foreground focus:ring-0 focus-visible:ring-0"
    placeholder={`Enter the ${placeholder} here`}
    bind:value={text}
  />

  {#if !text}
    <FileDrop
      clickToUpload={true}
      fileLimit={1}
      multiple={false}
      on:filedrop|once={(e) => {
        text = "";
        file = e.detail.files?.accepted[0];
      }}
    >
      <div
        class="cursor-pointer py-6 px-3 border border-dashed mt-4 rounded-sm bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-700 text-slate-500"
      >
        <div class="flex items-center space-x-3">
          <HardDriveUpload />
          <div>
            <div>Or drag and drop a {placeholder} file here</div>
            <div class="text-primary underline underline-offset-4">
              Browse files
            </div>
          </div>
        </div>
      </div>
    </FileDrop>
  {/if}
{/if}
