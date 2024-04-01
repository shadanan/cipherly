<script lang="ts">
  import "../app.pcss";
  import SecondaryNav from "$/lib/components/SecondaryNav.svelte";
  import DarkModeToggle from "$/lib/components/DarkModeToggle.svelte";
  import Main from "$/lib/components/Main.svelte";
  import Banner from "$/lib/components/Banner.svelte";
  import config from "$/lib/config";
  import PrimaryNav from "$/lib/components/PrimaryNav.svelte";
  import Footer from "$/lib/components/Footer.svelte";
  import { ModeWatcher } from "mode-watcher";
</script>

<ModeWatcher />
<div class="min-h-full">
  <DarkModeToggle />
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
  <Footer />
</div>
