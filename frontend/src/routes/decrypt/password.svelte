<script lang="ts">
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import { AlertCircle } from "lucide-svelte";

  export let error: string | null;
  export let value = "";
</script>

<div class="space-y-2">
  <Label
    class="text-background-foreground text-sm uppercase tracking-wider"
    for="password">Password</Label
  >

  {#if error != null && error != ""}
    <p class="flex items-center space-x-1 text-xs text-destructive">
      <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
      <span>
        {error}
      </span>
    </p>
  {/if}

  <Input
    id="password"
    class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
    type="password"
    placeholder="The password to use for decryption"
    bind:value
    required
  />
</div>
