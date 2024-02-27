<script lang="ts">
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import { cubicInOut } from "svelte/easing";
  import { crossfade } from "svelte/transition";

  export let items: { href?: string; title: string }[];

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });
</script>

<nav class="flex space-x-2 lg:flex-col lg:space-x-0 lg:space-y-1">
  {#each items as item}
    {#if item.href === undefined}
      <h2 class="mb-2 px-4 text-lg font-semibold tracking-tight">
        {item.title}
      </h2>
    {:else}
      {@const isActive = $page.url.pathname === item.href}

      <Button
        href={item.href}
        variant="ghost"
        class="relative justify-start hover:bg-transparent"
        data-sveltekit-noscroll
      >
        {#if isActive}
          <div
            class="absolute inset-0 rounded-md bg-muted"
            in:send={{ key: "active-sidebar-tab" }}
            out:receive={{ key: "active-sidebar-tab" }}
          />
        {/if}
        <div class="relative">
          {item.title}
        </div>
      </Button>
    {/if}
  {/each}
</nav>
