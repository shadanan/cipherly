<script lang="ts">
  import { login, logout, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";

  let payload = "";
  let plainText: Promise<string> | null = null;

  if (location.hash) {
    payload = location.hash.slice(1);
  }

  async function decrypt(payload: string): Promise<string> {
    if ($token === null) {
      logout();
      throw new Error("Cannot decrypt without logging in.");
    }
    const { sealedEnvelope, cipherText } = Cipherly.decodeAuthPayload(payload);
    const envelope = await Cipherly.unseal(sealedEnvelope, $token);
    const plainText = await Cipherly.decrypt(
      cipherText,
      envelope.dek,
      envelope.iv
    );
    return Cipherly.decodeUtf8(plainText);
  }
</script>

<h1 class="h1">Authorization Based Decryption</h1>

<div class="mt-4">
  <label class="label" for="payload">Ciphertext Payload</label>
  <textarea
    id="payload"
    class="textarea"
    bind:value={payload}
    placeholder="The ciphertext payload to be decrypted"
  />
</div>

{#if $token === null}
  <div class="mt-4">
    <button class="btn variant-filled" type="button" on:click={login}>
      Login to Decrypt
    </button>
  </div>
{:else}
  <div class="mt-4">
    <button
      class="btn variant-filled"
      type="button"
      on:click={() => (plainText = decrypt(payload))}
    >
      Decrypt
    </button>
  </div>
{/if}

{#if plainText}
  <hr class="mt-8 mb-8" />
  {#await plainText}
    <div class="mt-8">Decrypting...</div>
  {:then plainText}
    <label class="label" for="plainText">Decrypted Plaintext</label>
    <div id="plainText" class="p-3 mb-2 border rounded-md font-mono">
      {plainText}
    </div>
    <button
      class="btn variant-filled"
      type="button"
      on:click={() => navigator.clipboard.writeText(plainText)}
    >
      Copy Plaintext
    </button>
  {:catch err}
    {#if err.code === 401}
      <aside class="alert variant-ghost-error">
        <div class="alert-message">
          <h3 class="h3">Unauthorized</h3>
          <p>You are not authorized to decrypt this secret.</p>
        </div>
      </aside>
    {:else}
      <aside class="alert variant-ghost-error">
        <div class="alert-message">
          <h3 class="h3">Failed to Decrypt</h3>
          <p>Ciphertext may be invalid.</p>
        </div>
      </aside>
    {/if}
  {/await}
{/if}
