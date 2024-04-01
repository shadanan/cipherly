<script lang="ts">
  import "../app.pcss";
  import SecondaryNav from "$/lib/components/SecondaryNav.svelte";
  import Main from "$/lib/components/Main.svelte";
  import Banner from "$/lib/components/Banner.svelte";
  import config from "$/lib/config";
  import PrimaryNav from "$/lib/components/PrimaryNav.svelte";
</script>

<div class="min-h-full">
  <!-- <Navbar user={$currentUser} /> -->
  <section class="text-gray-600 pt-16 bg-image">
    <Banner />
    <PrimaryNav items={config.site.mainNav} />
    <Main>
      <div class="container">
        <SecondaryNav items={config.site.sidebarNav} />
        <main class="flex w-full flex-1 flex-col overflow-hidden">
          <slot />
        </main>
      </div>
    </Main>
  </section>
  <div class="my-12 text-gray-400 text-sm text-center">
    Made with ♥ by
    <a class="underline-offset-4 underline" href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
  </div>
</div>
