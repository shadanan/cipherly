<script lang="ts">
  import { cn } from "$lib/utils";
  import { page } from "$app/stores";

  export let items: {
    href: string;
    title: string;
    disabled?: boolean;
    soon?: boolean;
    icon: ConstructorOfATypedSvelteComponent;
  }[];
</script>

<div>
  <!-- Mobile -->
  <div class="sm:hidden">
    <label for="tabs" class="sr-only">Select a tab</label>
    <!-- TODO: Use an "onChange" listener to redirect the user to the selected tab URL. -->
    <select
      id="tabs"
      name="tabs"
      class="block h-[60px] w-full text-base border-gray-300 focus:ring-0 sm:text-sm border-l-0 border-r-0 focus:outline-none focus-border-none"
    >
      {#each items as item}
        {#if item.href}
          <option selected={item.href.startsWith($page.url.pathname.slice(0, -1))}>
            {item.title}
          </option>
        {/if}
      {/each}
    </select>
  </div>

  <div class="hidden sm:block">
    <div class="border-gray-200 border-b-4">
      <nav class="-mb-px flex space-x-8 justify-center" aria-label="Tabs">
        {#each items as item}
          {#if item.href}
            <a
              class={cn(
                "flex items-center border-transparent whitespace-nowrap -mb-[3px] py-4 px-1 border-b-4 font-bold text-lg",
                $page.url.pathname !== "/" && item.href.startsWith($page.url.pathname.slice(0, -1))
                  ? "border-primary border-b-4 text-primary"
                  : "text-gray-500 hover:text-gray-700 hover:border-gray-300",
                item.disabled && "cursor-not-allowed opacity-80"
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
  </div>
</div>
