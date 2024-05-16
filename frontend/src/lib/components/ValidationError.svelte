<script lang="ts">
  import { AlertCircle } from "lucide-svelte";
  import type { ZodError } from "zod";

  export let path: string;
  export let error: ZodError | null;

  let message: string | undefined;
  $: message = error?.issues.find((e) => e.path.includes(path))?.message;
</script>

{#if message}
  <p class="flex items-center space-x-1 text-xs text-destructive">
    <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
    <span>{message}</span>
  </p>
{/if}
