<script lang="ts">
  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift,
  } from "@floating-ui/dom";
  import {
    AppBar,
    AppShell,
    Drawer,
    getDrawerStore,
    initializeStores,
    storePopup,
  } from "@skeletonlabs/skeleton";
  import "../app.pcss";
  import Login from "./login.svelte";
  import Navigation from "./navigation.svelte";

  initializeStores();
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

  const drawerStore = getDrawerStore();

  function drawerOpen() {
    drawerStore.open({});
  }
</script>

<Drawer><Navigation /></Drawer>
<AppShell slotSidebarLeft="bg-surface-500/5 w-0 lg:w-64">
  <svelte:fragment slot="header">
    <AppBar>
      <svelte:fragment slot="lead">
        <button class="btn btn-sm mr-4 lg:hidden" on:click={drawerOpen}>
          <span>
            <svg viewBox="0 0 100 80" class="fill-token h-4 w-4">
              <rect width="100" height="20" />
              <rect y="30" width="100" height="20" />
              <rect y="60" width="100" height="20" />
            </svg>
          </span>
        </button>
        <strong class="text-xl">Cipherly</strong>
      </svelte:fragment>
      <svelte:fragment slot="trail">
        <Login />
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>

  <svelte:fragment slot="sidebarLeft">
    <Navigation />
  </svelte:fragment>

  <svelte:fragment slot="footer">
    <div class="mt-24 text-center text-sm text-gray-500">
      Made with ❤︎ by
      <a href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
    </div>
  </svelte:fragment>

  <div class="p-4">
    <slot />
  </div>
</AppShell>
