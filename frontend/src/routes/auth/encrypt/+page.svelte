<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import { InputChip } from "@skeletonlabs/skeleton";

  let plainText = "";
  let emails: string[] = [];
  let payload: Promise<string> | null = null;

  async function encrypt(plainText: string, emails: string[]): Promise<string> {
    const dek = await Cipherly.generateKey();
    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plainText),
      dek,
      iv,
    );
    const sealedEnvelope = await Cipherly.seal({ dek, iv, emails });
    return Cipherly.encodeAuthPayload({ sealedEnvelope, cipherText });
  }
</script>

<h1 class="h1">Authorization Based Encryption</h1>

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
  <label class="label" for="emails">Authorized Emails</label>
  <InputChip
    bind:value={emails}
    name="chips"
    class="mb-2"
    placeholder="Emails of the users authorized to decrypt the secret"
  />
  <button
    type="button"
    class="variant-filled btn"
    on:click={() => (payload = encrypt(plainText, emails))}
  >
    Encrypt
  </button>
</div>

{#if payload}
  <hr class="mb-8 mt-8" />
  {#await payload}
    <div class="mt-8">Encrypting...</div>
  {:then payload}
    {@const url = Cipherly.authUrl() + payload}
    <label class="label" for="plainText">Ciphertext Payload</label>
    <aside class="alert variant-ghost mb-2">
      <div class="alert-message">
        <a href={url}>{payload}</a>
      </div>
    </aside>
    <button
      type="button"
      class="variant-filled btn"
      on:click={() => navigator.clipboard.writeText(payload)}
    >
      Copy Ciphertext
    </button>
    <button
      type="button"
      class="variant-filled btn"
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
