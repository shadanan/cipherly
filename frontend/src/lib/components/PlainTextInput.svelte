<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { Textarea } from "./ui/textarea";
  import { cn } from "$lib/utils";
  import FileDrop from "filedrop-svelte";
  import { FileText, XCircle } from "lucide-svelte";
  import type { Files } from "filedrop-svelte";
  import { filesize } from "filesize";
  let uploadedFiles: Files | undefined;

  type $$Props = HTMLTextareaAttributes & {
    plainText: string | undefined;
    plainFile: File | undefined;
  };

  let className: $$Props["class"] = undefined;
  export { className as class };
  export let id: $$Props["id"] = "plainText";
  export let placeholder: $$Props["placeholder"] = "Enter the plaintext secret";
  export let plainText: $$Props["plainText"] = undefined;
  export let plainFile: $$Props["plainFile"] = undefined;
</script>

{#if plainFile}
  <div
    class="border px-3 py-4 rounded-md flex bg-accent justify-between items-center"
  >
    <div class="flex items-center space-x-3">
      <FileText class="w-5 text-slate-400" />
      <div class="space-y-[-3px]">
        <div class="text-md">{plainFile.name}</div>
        <div class="text-xs text-muted-foreground">
          2{filesize(plainFile.size)}
        </div>
      </div>
    </div>
    <button
      on:click={() => {
        plainFile = undefined;
        uploadedFiles = undefined;
      }}
    >
      <XCircle class="w-5 text-red-800 cursor-pointer" />
    </button>
  </div>
{:else}
  <Textarea
    class={cn(
      "border-2   text-base text-foreground focus:ring-0 focus-visible:ring-0 ",
      className,
    )}
    {id}
    {placeholder}
    bind:value={plainText}
  />

  <FileDrop
    clickToUpload={false}
    fileLimit={1}
    multiple={false}
    on:filedrop|once={(e) => {
      plainText = ""; // reset plainText
      uploadedFiles = e.detail.files;
      console.log(uploadedFiles);
      plainFile = uploadedFiles?.accepted[0];
    }}
  >
    <div
      class="cursor-pointer py-6 px-3 border border-dashed mt-4 rounded-sm bg-slate-50 dark:bg-slate-900 border-slate-200 dark:border-slate-700 text-slate-500"
    >
      Or, drag & drop or
      <span class="text-primary underline underline-offset-4"> select </span>
      to upload files
    </div>
  </FileDrop>
{/if}
