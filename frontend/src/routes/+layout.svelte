<script lang="ts">
  import Banner from "$lib/components/Banner.svelte";
  import DarkModeToggle from "$lib/components/DarkModeToggle.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import Main from "$lib/components/Main.svelte";
  import PrimaryNav from "$lib/components/PrimaryNav.svelte";
  import { KeyRound, Unlock, User } from "lucide-svelte";
  import { ModeWatcher } from "mode-watcher";
  import "../app.pcss";
</script>

<ModeWatcher />
<div class="min-h-full">
  <DarkModeToggle />
  <section class="text-foregound bg-background pt-16">
    <Banner />
    <PrimaryNav
      items={[
        {
          title: "Decrypt",
          href: "/decrypt/",
          icon: Unlock,
        },
        {
          title: "Password Encrypt",
          href: "/password/",
          icon: KeyRound,
        },
        {
          title: "Auth Encrypt",
          href: "/auth/",
          icon: User,
        },
      ]}
    />
    <Main>
      <div class="px-4 lg:px-10">
        <div class="flex w-full flex-1 flex-col overflow-hidden">
          <slot />
        </div>
      </div>
    </Main>
  </section>
  <Footer />
</div>
