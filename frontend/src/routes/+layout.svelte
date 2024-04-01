<script lang="ts">
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import SecondaryNav from "$/lib/components/SecondaryNav.svelte";
  import Main from "$/lib/components/Main.svelte";
  import Banner from "$/lib/components/Banner.svelte";
  import config from "$/lib/config";
  import PrimaryNav from "$/lib/components/PrimaryNav.svelte";
  import Sun from "lucide-svelte/icons/sun";
  import Moon from "lucide-svelte/icons/moon";

  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button/";
</script>

<ModeWatcher />
<div class="min-h-full">
  <Button on:click={toggleMode} variant="outline" size="icon" class="absolute top-5 right-5">
    <Sun class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
    <Moon class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
    <span class="sr-only">Toggle theme</span>
  </Button>

  <!-- <Navbar user={$currentUser} /> -->
  <section class="text-foregound pt-16 bg-background">
    <Banner />
    <PrimaryNav items={config.site.mainNav} />
    <Main>
      <div class="px-4 lg:px-10">
        <SecondaryNav items={config.site.sidebarNav} />
        <div class="flex w-full flex-1 flex-col overflow-hidden">
          <slot />
        </div>
      </div>
    </Main>
  </section>
  <div class="my-12 text-gray-400 text-sm text-center">
    Made with ♥ by
    <a class="underline-offset-4 underline" href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
  </div>
</div>
