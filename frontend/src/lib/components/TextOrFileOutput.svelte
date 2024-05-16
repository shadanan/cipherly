<script lang="ts">
  import { decodeUtf8 } from "$lib/cipherly";
  import CopyText from "./CopyText.svelte";
  import EncryptionAlert from "./EncryptionAlert.svelte";
  import { Button } from "./ui/button";
  import { Label } from "./ui/label";
  import { Skeleton } from "./ui/skeleton";
  import { Textarea } from "./ui/textarea";

  export let kind: string;
  export let data: Promise<Uint8Array[]>;
  export let name: string | null = null;

  function save(data: Uint8Array[], name: string) {
    const blob = new Blob(data, { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = name;
    a.click();
  }

  if (name !== null) {
    data.then((data) => save(data, name));
  }
</script>

<div>
  {#await data}
    <div class="space-y-6 py-6">
      <Skeleton class="h-20 w-full" />
      <Skeleton class="h-10 w-full" />
    </div>
  {:then data}
    {#if !name}
      {@const text = data.map(decodeUtf8).join("")}
      <Label
        for="payload"
        class="text-background-foreground text-sm uppercase tracking-wider"
      >
        Ciphertext Payload
      </Label>
      <Textarea
        class="focus-visible:ring-none disabled:opacity-1 border-2  border-muted text-base focus-visible:outline-none disabled:cursor-text disabled:text-green-600"
        id="payload"
        disabled
        value={text}
        placeholder="The plain text secret to encrypt"
      />
      <div class="space-x-2 pt-4">
        <CopyText label="Ciphertext" {text} />
      </div>
    {:else}
      <Button on:click={() => save(data, name)}>
        Download {name}
      </Button>
    {/if}
  {:catch error}
    <EncryptionAlert title={`Failed to ${kind}`} {error} />
  {/await}
</div>
