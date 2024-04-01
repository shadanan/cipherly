<script lang="ts">
  import { Button } from "$lib/components/ui/button";

  let copied: boolean;

  export let label: string;
  export let text: string;

  function copyToClipboard(text: string | null): void {
    if (!text) return;
    navigator.clipboard.writeText(text);
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 500);
  }
</script>

<Button
  variant="secondary"
  class="min-w-[140px]"
  type="button"
  on:click={() => copyToClipboard(text)}
>
  {#if copied}
    Copied!
  {:else}
    Copy {label}
  {/if}
</Button>
