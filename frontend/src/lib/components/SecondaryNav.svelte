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

{#if items.length && $page.url.pathname !== "/"}
  <nav class="relative w-full flex justify-center p-2 rounded-md bg-gray-100 my-2">
    {#each items as item}
      {#if $page.url.pathname.startsWith(item.matches) && item.href}
        <a href={item.disabled ? "/" : item.href} class="flex-1 flex justify-center rounded-md">
          <div
            class={cn(
              " w-full flex justify-center hover:text-accent-foreground group  items-center rounded-md px-2 py-2 text-base font-medium",
              $page.url.pathname == ensureTrailingSlash(item.href)
                ? "bg-white shadow-sm  text-black font-bold border-gray-300"
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
