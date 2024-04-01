<script lang="ts">
  import { cn, ensureTrailingSlash } from "$lib/utils";
  import { page } from "$app/stores";

  export let items: {
    href: string;
    title: string;
    matches: string;
    disabled?: boolean;
    soon?: boolean;
    icon: ConstructorOfATypedSvelteComponent;
  }[];
</script>

{#if items.length}
  <nav class="relative w-full flex justify-center space-x-4">
    {#each items as item}
      {#if $page.url.pathname.startsWith(item.matches) && item.href}
        <a href={item.disabled ? "/" : item.href}>
          <div
            class={cn(
              "w-[150px] flex justify-center hover:bg-accent hover:text-accent-foreground group  items-center rounded-md px-4 py-3 text-base font-medium",
              $page.url.pathname == ensureTrailingSlash(item.href)
                ? "bg-accent text-black font-bold border border-gray-300"
                : "transparent text-gray-400",
              item.disabled && "cursor-not-allowed opacity-80"
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
