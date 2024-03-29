<script lang="ts">
  import * as Cipherly from "$lib/cipherly";

  let plainText = "";
  let password = "";
  let payload: Promise<string> | null = null;

  async function encrypt(plainText: string, password: string): Promise<string> {
    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);

    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plainText),
      key,
      iv
    );

    return Cipherly.encodePasswordPayload({ salt, iv, cipherText });
  }
</script>

<h1 class="h1">Password Based Encryption</h1>

<div class="mt-4">
  <label class="label" for="plainText">Plaintext</label>
  <textarea
    id="plainText"
    class="textarea"
    bind:value={plainText}
    placeholder="The plaintext secret to encrypt"
  />
</div>

<div class="mt-4">
  <label class="label" for="password">Password</label>
  <div class="flex space-x-2">
    <input
      id="password"
      type="password"
      class="input"
      placeholder="The password to use for encryption"
      bind:value={password}
    />
    <button
      type="button"
      class="btn variant-filled"
      on:click={() => (payload = encrypt(plainText, password))}
    >
      Encrypt
    </button>
  </div>
</div>

{#if payload}
  <hr class="mt-8 mb-8" />
  {#await payload}
    <div class="mt-8">Encrypting...</div>
  {:then payload}
    {@const url = Cipherly.passwordUrl() + payload}
    <label class="label" for="plainText">Ciphertext Payload</label>
    <aside class="alert variant-ghost mb-2">
      <div class="alert-message">
        <a href={url}>{payload}</a>
      </div>
    </aside>

    <button
      type="button"
      class="btn variant-filled"
      on:click={() => navigator.clipboard.writeText(payload)}
    >
      Copy Ciphertext
    </button>
    <button
      type="button"
      class="btn variant-filled"
      on:click={() => navigator.clipboard.writeText(url)}
    >
      Copy Decrypt URL
    </button>
  {:catch error}
    <aside class="alert variant-ghost-error">
      <div class="alert-message">
        <h3 class="h3">Failed to Encrypt</h3>
        <p>{error.message}</p>
      </div>
    </aside>
  {/await}
{/if}
