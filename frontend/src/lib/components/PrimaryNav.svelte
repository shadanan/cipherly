<script lang="ts">
  import { page } from "$app/stores";
  import { cn } from "$lib/utils";

  export let items: {
    href: string;
    title: string;
    disabled?: boolean;
    soon?: boolean;
    icon: ConstructorOfATypedSvelteComponent;
  }[];
</script>

<div class="border-accent border-b-4">
  <nav
    class="-mb-px flex space-x-2 md:space-x-8 justify-center"
    aria-label="Tabs"
  >
    {#each items as item}
      {#if item.href}
        <a
          class={cn(
            "flex items-center border-transparent whitespace-nowrap -mb-[3px] py-2 px-1 border-b-4 font-semibold text-base md:text-lg",
            $page.url.pathname !== "/" &&
              item.href.startsWith($page.url.pathname)
              ? "border-primary border-b-4 text-primary"
              : "text-muted-foreground hover:text-foreground border-accent hover:border-muted-foreground",
            item.disabled && "cursor-not-allowed opacity-80",
          )}
          href={item.disabled ? "/" : item.href}
        >
          {#if item.icon}
            <svelte:component this={item.icon} class="mr-2 h-4 w-4" />
          {/if}
          <span>{item.title}</span>
        </a>
      {/if}
    {/each}
  </nav>
</div>
