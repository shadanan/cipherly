<script lang="ts">
  let secret = "";
  let authorizedUsers = "";
  let promise: Promise<any> | null = null;

  async function encrypt() {
    const body = {
      dek: "123456",
      authorized_users: ["user1@gmail.com", "user2@gmail.com"],
    };

    const res = await fetch("/api/encrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });

    const encryptResponse = await res.json();

    if (res.ok) {
      return encryptResponse["envelope_header"];
    } else {
      throw new Error(encryptResponse["envelope_header"]);
    }
  }

  function handleClick() {
    promise = encrypt();
  }
</script>

<main>
  <input bind:value={secret} placeholder="Enter your secret here" />
  <input
    bind:value={authorizedUsers}
    placeholder="Enter authorized users here"
  />
  <button on:click={handleClick}> Encrypt </button>

  {#if promise !== null}
    {#await promise}
      <p>Encrypting</p>
    {:then number}
      <p>{number}</p>
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
  {/if}
</main>

<style>
</style>
