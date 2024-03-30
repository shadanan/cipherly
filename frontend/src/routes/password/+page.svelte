<script lang="ts">
  import * as Cipherly from "$lib/cipherly";

  let payload = "";
  let password = "";
  let plainText: Promise<string> | null = null;

  if (location.hash) {
    payload = location.hash.slice(1);
  }

  async function decrypt(payload: string, password: string): Promise<string> {
    const { salt, iv, cipherText } = Cipherly.decodePasswordPayload(payload);
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);
    const plainText = await Cipherly.decrypt(cipherText, key, iv);
    return Cipherly.decodeUtf8(plainText);
  }
</script>

<h1 class="h1">Password Based Decryption</h1>

<div class="mt-4">
  <label class="label" for="payload">Ciphertext Payload</label>
  <textarea
    id="payload"
    class="textarea"
    bind:value={payload}
    placeholder="The ciphertext payload to be decrypted"
  />
</div>

<div class="mt-4">
  <label class="label" for="password">Password</label>
  <div class="flex space-x-2">
    <input
      class="input"
      type="password"
      placeholder="The password to use for decryption"
      bind:value={password}
    />
    <button
      type="button"
      class="variant-filled btn"
      on:click={() => (plainText = decrypt(payload, password))}
    >
      Decrypt
    </button>
  </div>
</div>

{#if plainText}
  <hr class="mb-8 mt-8" />
  {#await plainText}
    <div class="mt-8">Decrypting...</div>
  {:then plainText}
    <label class="label" for="plainText">Decrypted Plaintext</label>
    <aside class="alert variant-ghost mb-2">
      <div class="alert-message">
        <p id="plainText">{plainText}</p>
      </div>
    </aside>
    <button
      type="button"
      class="variant-filled btn"
      on:click={() => navigator.clipboard.writeText(plainText)}
    >
      Copy Plaintext
    </button>
  {:catch}
    <aside class="alert variant-ghost-error">
      <div class="alert-message">
        <h3 class="h3">Failed to Decrypt</h3>
        <p>Password is incorrect or ciphertext is invalid.</p>
      </div>
    </aside>
  {/await}
{/if}
