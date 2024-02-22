<script lang="ts">
  type Envelope = {
    header: string;
  };

  type Dek = {
    dek: string;
    iv: string;
  };

  let secret = "";
  let authorizedUsers = "";
  let encryptedPayload = "";
  let encryptpromise: Promise<String> | null = null;
  let decryptpromise: Promise<String> | null = null;

  async function encrypt(
    secret: string,
    authorizedUsers: string[]
  ): Promise<String> {
    // --------------------- PART 1 --------------------------
    // 1. Generate a Data Encryption Key (dek) and Initialization Vector (iv)
    const dek = await window.crypto.subtle.generateKey(
      { name: "AES-GCM", length: 256 },
      true,
      ["encrypt", "decrypt"]
    );

    const iv = window.crypto.getRandomValues(new Uint8Array(12));

    // 2. Encrypt the Secret using dek and iv
    const encryptedSecret = await window.crypto.subtle.encrypt(
      { name: "AES-GCM", iv: iv },
      dek,
      new TextEncoder().encode(secret)
    );

    // 3. Encode the Encrypted Secret to a base64-encoded string
    const encodedSecret = btoa(
      String.fromCharCode(...new Uint8Array(encryptedSecret))
    )
      .replace(/\+/g, "-")
      .replace(/\//g, "_");

    // 4. Export the CryptoKey: dek to a portable format
    const encodedDek = btoa(
      String.fromCharCode(
        ...new Uint8Array(await window.crypto.subtle.exportKey("raw", dek))
      )
    );
    const encodedIv = btoa(String.fromCharCode(...iv));

    // --------------------- PART 2 ---------------------
    // Encrypt encodedDek using kek (via server)
    const res = await fetch("/api/encrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        dek: encodedDek,
        iv: encodedIv,
        authorized_users: authorizedUsers,
      }),
    });
    const envelopeEncrypted: Envelope = await res.json();

    // --------------------- PART 3 ---------------------
    // Result encrypted payload is the kek encrypted envelope + encodedSecret
    return envelopeEncrypted.header + "." + encodedSecret;
  }

  async function decrypt(encryptedPayload: string[]): Promise<String> {
    const envelopeEncryptedHeader = encryptedPayload[0];
    const encodedSecret = encryptedPayload[1];

    // --------------------- PART 1 ---------------------
    // Decrypt encoded Dek using kek (via server)
    const res = await fetch("/api/decrypt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        header: envelopeEncryptedHeader,
      }),
    });
    const envelopeDecrypted: Dek = await res.json();

    // --------------------- PART 2 ---------------------
    // Encode dek, iv, encodedSecret into the correct format before decrypting
    const iv = Uint8Array.from(atob(envelopeDecrypted.iv), (c) =>
      c.charCodeAt(0)
    );
    const dek = Uint8Array.from(atob(envelopeDecrypted.dek), (c) =>
      c.charCodeAt(0)
    );

    const cryptoKey = await window.crypto.subtle.importKey(
      "raw",
      dek,
      {
        name: "AES-GCM",
        length: 256,
      },
      true,
      ["encrypt", "decrypt"]
    );

    const decodedSecret = new Uint8Array(
      atob(encodedSecret.replace(/\-/g, "+").replace(/\_/g, "/"))
        .split("")
        .map((char) => char.charCodeAt(0))
    );

    // --------------------- PART 3 ---------------------
    // Decrypt secret using cryptoKey and iv
    const decryptedSecret = await window.crypto.subtle.decrypt(
      { name: "AES-GCM", iv: iv },
      cryptoKey,
      decodedSecret
    );

    const secretDecrypted = new TextDecoder().decode(decryptedSecret);
    return secretDecrypted;
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
      on:click={() =>
        (encryptpromise = encrypt(secret, authorizedUsers.split(",")))}
      >Encrypt</button
    >
  </div>
  <div>
    {#if encryptpromise !== null}
      {#await encryptpromise}
        <p>Encrypting</p>
      {:then payload}
        <p>{payload}</p>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {/if}
  </div>
  <div>
    <input
      bind:value={encryptedPayload}
      placeholder="Enter your Encrypted text here"
    />
  </div>
  <div>
    <button
      on:click={() => (decryptpromise = decrypt(encryptedPayload.split(".")))}
      >Decrypt</button
    >
  </div>
  <div>
    {#if decryptpromise !== null}
      {#await decryptpromise}
        <p>Decrypting</p>
      {:then Encyptedtext}
        <p>{Encyptedtext}</p>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {/if}
  </div>
</main>

<style>
</style>
