<script lang="ts">
  import "../app.pcss";
  import SideNav from "$/lib/components/SideNav.svelte";
  import Main from "$/lib/components/Main.svelte";
  import Navbar from "$/lib/components/Navbar.svelte";
  import Banner from "$/lib/components/Banner.svelte";
  import { currentUser } from "$lib/auth";
  import config from "$/lib/config";
  import TabsNav from "$/lib/components/TabsNav.svelte";
</script>

<div class="min-h-full">
  <!-- <Navbar user={$currentUser} /> -->
  <section class="text-gray-600 pt-20 bg-image">
    <Banner />
    <TabsNav items={config.site.mainNav} />
    <Main>
      <div class="container grid flex-1 gap-12 md:grid-cols-[200px_1fr]">
        <aside class="hidden w-[200px] flex-col md:flex">
          <SideNav items={config.site.sidebarNav} />
        </aside>
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
<!-- 
<main>
  <div class="space-y-6 pb-16">
    <div class="flex flex-col space-y-8">
      <aside class="bg-slate-400 p-2 flex">
        <Sidebar
          items={[
            { title: "Home", href: "/" },
            { title: "Password Encrypt", href: "/password/encrypt/" },
            { title: "Password Decrypt", href: "/password/" },
            { title: "Authorization Encrypt", href: "/auth/encrypt/" },
            { title: "Authorization Decrypt", href: "/auth/" },
          ]}
        />
        <div class="flex-auto"></div>
        <Login />
      </aside>
      <div class="flex-1 p-2">
        <slot />
      </div>
    </div>

    <div class="mt-24 text-gray-500 text-sm text-center">
      Made with ❤︎ by
      <a href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
    </div>
  </div>
</main> -->
