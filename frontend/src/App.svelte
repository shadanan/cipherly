<script lang="ts">
  let secret = "";
  let authorizedUsers = "";

  type Envelope = {
    header: string;
  };

  let promise: Promise<Envelope> | null = null;

  async function encrypt(
    secret: string,
    authorizedUsers: string[]
  ): Promise<Envelope> {
    const res = await fetch("/api/encrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        dek: "123456",
        authorized_users: authorizedUsers,
      }),
    });

    return await res.json();
  }
</script>

<main>
  <div>
    <input bind:value={secret} placeholder="Enter your secret here" />
  </div>
  <div>
    <input
      bind:value={authorizedUsers}
      placeholder="Enter authorized users here"
    />
  </div>
  <div>
    <button
      on:click={() => (promise = encrypt(secret, authorizedUsers.split(",")))}
      >Encrypt</button
    >
  </div>
  <div>
    {#if promise !== null}
      {#await promise}
        <p>Encrypting</p>
      {:then envelope}
        <p>{envelope.header}</p>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {/if}
  </div>
</main>

<style>
</style>
