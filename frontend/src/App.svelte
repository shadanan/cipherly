<script lang="ts">
  let secret = "";
  let authorizedUsers = "";

  type Envelope = {
    header: string;
  };

  let promise: Promise<String> | null = null;

  async function encrypt(
    secret: string,
    authorizedUsers: string[]
  ): Promise<String> {
    const dek = await window.crypto.subtle.generateKey(
      { name: "AES-GCM", length: 256 },
      true,
      ["encrypt", "decrypt"]
    );
    const iv = window.crypto.getRandomValues(new Uint8Array(12));

    const encryptedSecret = await window.crypto.subtle.encrypt(
      { name: "AES-GCM", iv: iv },
      dek,
      new TextEncoder().encode(secret)
    );
    const encodedSecret = btoa(
      String.fromCharCode(...new Uint8Array(encryptedSecret))
    );

    const encodedDek = await window.crypto.subtle.exportKey("jwk", dek);

    const res = await fetch("/api/encrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        dek: encodedDek,
        iv: Array.from(iv),
        authorized_users: authorizedUsers,
      }),
    });

    const envelope: Envelope = await res.json();
    return envelope.header + "." + encodedSecret;
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
      {:then payload}
        <p>{payload}</p>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {/if}
  </div>
</main>

<style>
</style>
