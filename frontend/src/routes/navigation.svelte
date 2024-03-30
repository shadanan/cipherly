<script lang="ts">
  import { getDrawerStore } from "@skeletonlabs/skeleton";

  const drawerStore = getDrawerStore();

  function drawerClose() {
    drawerStore.close();
  }
</script>

<nav class="list-nav p-4">
  <ul>
    <li><a href="/" on:click={drawerClose}>Home</a></li>
    <li>
      <a href="/password/encrypt/" on:click={drawerClose}>Password Encrypt</a>
    </li>
    <li><a href="/password/" on:click={drawerClose}>Password Decrypt</a></li>
    <li>
      <a href="/auth/encrypt/" on:click={drawerClose}>Authorization Encrypt</a>
    </li>
    <li><a href="/auth/" on:click={drawerClose}>Authorization Decrypt</a></li>
  </ul>
</nav>
