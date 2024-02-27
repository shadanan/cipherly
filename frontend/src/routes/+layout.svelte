<script>
  import "../app.pcss";
  import Sidebar from "./Sidebar.svelte";
</script>

<main>
  <div class="p-5">
    <div class="lg:grid lg:grid-cols-4">
      <div class="lg:col-span-1 p-5">
        <Sidebar
          items={[
            { title: "Home", href: "/" },
            { title: "Encrypt", href: "/password/encrypt" },
            { title: "Decrypt", href: "/password" },
          ]}
        />
      </div>
      <div class="lg:col-span-3 lg:border-l p-5">
        <slot />
      </div>
    </div>
  </div>

  <div class="mt-24 text-gray-500 text-sm text-center">
    Made with ❤︎ by
    <a href="https://www.youtube.com/@friendlytl">The FriendlyTL</a>
  </div>
</main>
