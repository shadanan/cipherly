<script lang="ts">
  import { decodeUtf8, save } from "$lib/cipherly";
  import CopyText from "./CopyText.svelte";
  import { Button } from "./ui/button";
  import { Label } from "./ui/label";
  import { Textarea } from "./ui/textarea";

  export let data: Uint8Array[];
  export let name: string | null = null;

  if (name !== null) {
    save(data, name);
  }
</script>

<div>
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
</div>
