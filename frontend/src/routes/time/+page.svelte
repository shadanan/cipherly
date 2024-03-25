<script lang="ts">
  import { token } from "$lib/auth";
  import Button from "$lib/components/ui/button/button.svelte";

  let time: Promise<Time> | null = null;

  type Time = {
    time: string;
  };

  async function fetchTime(): Promise<Time> {
    const response = await fetch("/api/time", {
      headers: { Authorization: "Bearer " + $token },
    });
    if (!response.ok) {
      throw { code: response.status, message: response.statusText };
    }
    const result = await response.json();
    return result;
  }
</script>

<Button on:click={() => (time = fetchTime())}>Get Time</Button>

{#if time}
  {#await time}
    <div>Fetching time...</div>
  {:then time}
    <div>Current time: {time.time}</div>
  {:catch err}
    {#if err.code === 401}
      <div>Unauthorized</div>
    {:else}
      <div>Failed to fetch time: {err}</div>
    {/if}
  {/await}
{/if}
