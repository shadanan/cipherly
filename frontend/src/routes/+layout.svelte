<script lang="ts">
  import "../app.pcss";
  import Login from "./login.svelte";
  import Sidebar from "./sidebar.svelte";
</script>

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
</main>
