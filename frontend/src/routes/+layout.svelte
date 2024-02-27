<script>
  import "../app.pcss";
  import Sidebar from "./sidebar.svelte";
</script>

<main>
  <div class="space-y-6 pb-16">
    <div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
      <aside class="bg-slate-400 p-2 lg:w-1/5">
        <Sidebar
          items={[
            { title: "Home", href: "/" },
            { title: "Encrypt", href: "/password/encrypt" },
            { title: "Decrypt", href: "/password" },
          ]}
        />
      </aside>
      <div class="flex-1 p-2 lg:max-w-2xl">
        <slot />
      </div>
    </div>

    <div class="mt-24 text-gray-500 text-sm text-center">
      Made with ❤︎ by
      <a href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
    </div>
  </div>
</main>
