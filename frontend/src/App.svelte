<script lang="ts">
  let display = "loading";

  async function load() {
    const resp = await fetch("/api/hello");
    const json = await resp.json();
    display = json;
  }
</script>

<main>
  <button on:click={() => load()}>Load</button>

  {JSON.stringify(display)}
</main>

<style>
</style>
