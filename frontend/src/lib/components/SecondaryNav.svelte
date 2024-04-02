<script lang="ts">
  import { cn } from "$lib/utils";
  import { page } from "$app/stores";

  export let items: {
    href: string;
    title: string;
    parent: string;
    disabled?: boolean;
    soon?: boolean;
    icon: ConstructorOfATypedSvelteComponent;
  }[];
</script>

{#if items.length && $page.url.pathname !== "/"}
  <nav class="relative my-2 flex w-full justify-center rounded-md py-1">
    {#each items as item}
      {#if $page.url.pathname.startsWith(item.parent) && item.href}
        <a
          href={item.disabled ? "/" : item.href}
          class="flex flex-1 justify-center rounded-md"
        >
          <div
            class={cn(
              " flex w-full items-center justify-center rounded-md px-2 py-3 text-base font-medium hover:text-accent-foreground",
              $page.url.pathname == item.href
                ? "bg-secondary font-bold text-foreground"
                : "transparent text-muted-foreground",
              item.disabled && "cursor-not-allowed opacity-80",
            )}
          >
            <svelte:component this={item.icon} class="mr-2 h-4 w-4" />
            <span>{item.title}</span>
          </div>
        </a>
      {/if}
    {/each}
  </nav>
{/if}
