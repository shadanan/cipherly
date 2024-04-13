<script lang="ts">
  import { Info, X } from "lucide-svelte";
  import { Badge } from "./ui/badge";
  import { Button } from "./ui/button";
  import { Input } from "./ui/input";

  export let id: string;
  export let values: Set<string> = new Set();
  export let placeholder = "";

  let input: string = "";

  function add(value: string) {
    values.add(value);
    values = values;
  }

  function remove(value: string) {
    values.delete(value);
    values = values;
  }

  function addAndClear() {
    if (input === "") {
      return;
    }
    add(input);
    input = "";
  }
</script>

<p class="flex items-center space-x-1 text-xs text-blue-500">
  <Info class="inline-block h-[12px] w-[12px]"></Info>
  <span>Press Enter after each entry</span>
</p>

<Input
  {id}
  class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
  {placeholder}
  required={values.size === 0}
  bind:value={input}
  on:blur={addAndClear}
  on:keydown={(e) => {
    if (e.key === "Enter") {
      e.preventDefault();
      addAndClear();
    }
  }}
/>

{#if values.size > 0}
  <div class="flex flex-wrap pt-2">
    {#each values as value}
      <Badge variant="secondary" class="mb-2 mr-2 space-x-1 text-sm">
        <span>{value}</span>
        <Button
          class="m-0 h-4 w-4 p-0 "
          variant="ghost"
          on:click={() => remove(value)}
        >
          <X class="cursor-pointer text-gray-400 hover:text-gray-500" />
        </Button>
      </Badge>
    {/each}
  </div>
{/if}
