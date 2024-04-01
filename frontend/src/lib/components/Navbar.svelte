<script lang="ts">
  import { clickOutside } from "$lib/utils";
  import { login, logout } from "$lib/auth";
  import clsx from "clsx";
  import Avatar from "./Avatar.svelte";
  import logo from "$lib/assets/cipherly-white.png";
  import config from "$lib/config";
  import { Button } from "./ui/button";

  export let user: App.Locals["user"] | undefined | null;

  // Menu toggle action
  let showMenu = false;
</script>

<nav class="fixed z-30 w-full border-b border-gray-200 bg-background">
  <div class="flex h-14 items-center justify-between px-3 py-3 lg:px-5 lg:pl-3">
    <div class="ml-2 flex items-center justify-start">
      <a href="/" class="flex items-center space-x-1">
        <img alt="The project logo" src={logo} class="w-6" />
        <span class="text-md self-center whitespace-nowrap font-extrabold uppercase tracking-wider">
          {config.site.name}
        </span>
      </a>
    </div>
    <div class="flex items-center">
      {#if user}
        <!-- Profile -->
        <div class="animate-in fade-in relative ml-3 flex items-center duration-300">
          <div>
            <button
              type="button"
              class="flex rounded-full text-sm focus:ring-4 focus:ring-gray-300"
              id="user-menu-button-2"
              aria-expanded="false"
              data-dropdown-toggle="dropdown-2"
              use:clickOutside={() => {
                showMenu = false;
              }}
              on:click={() => {
                showMenu = !showMenu;
              }}
            >
              <span class="sr-only">Open user menu</span>
              <div class="h-8 w-8 rounded-full bg-background">
                <Avatar {user} size={32} />
              </div>
            </button>
          </div>
          <!-- Dropdown menu -->
          <div
            class={clsx(
              "fixed right-5 top-9 z-50 my-4 list-none divide-y divide-gray-100 rounded border bg-background text-base shadow-lg",
              {
                visible: showMenu,
                invisible: !showMenu,
              }
            )}
            id="dropdown-2"
          >
            <div class="px-4 py-3" role="none">
              <!-- <p class="text-sm text-gray-900" role="none">
                {user.email}
              </p> -->
              <p class="truncate text-sm font-medium text-gray-900" role="none">
                {user?.email}
              </p>
            </div>
            <ul class="py-1" role="none">
              <li>
                <a
                  href="/settings"
                  class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  role="menuitem"
                >
                  Settings
                </a>
              </li>
              <li>
                <button
                  on:click={logout}
                  class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  role="menuitem"
                >
                  Log out
                </button>
              </li>
            </ul>
          </div>
        </div>
      {:else}
        <Button on:click={login} variant="ghost" class="">Login</Button>
      {/if}
    </div>
  </div>
</nav>
