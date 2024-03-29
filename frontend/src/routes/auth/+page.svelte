<script lang="ts">
  import { login, logout, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

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

<h1 class="text-4xl font-extrabold">Authorization Based Decryption</h1>

<div class="mt-4">
  <Label for="payload">Ciphertext Payload</Label>
  <Textarea
    id="payload"
    bind:value={payload}
    placeholder="The ciphertext payload to be decrypted"
  />
</div>

{#if $token === null}
  <div class="mt-4">
    <Button type="button" on:click={login}>Login to Decrypt</Button>
  </div>
{:else}
  <div class="mt-4">
    <Button type="button" on:click={() => (plainText = decrypt(payload))}>
      Decrypt
    </Button>
  </div>
{/if}

{#if plainText}
  <Separator class="mt-8 mb-8" />
  {#await plainText}
    <div class="mt-8">Decrypting...</div>
  {:then plainText}
    <Label for="plainText">Decrypted Plaintext</Label>
    <div id="plainText" class="p-3 mb-2 border rounded-md font-mono">
      {plainText}
    </div>
    <Button
      type="button"
      on:click={() => navigator.clipboard.writeText(plainText)}
    >
      Copy Plaintext
    </Button>
  {:catch err}
    <Alert.Root variant="destructive">
      {#if err.code === 401}
        <Alert.Title>Unauthorized</Alert.Title>
        <Alert.Description>
          You are not authorized to decrypt this secret.
        </Alert.Description>
      {:else}
        <Alert.Title>Failed to Decrypt</Alert.Title>
        <Alert.Description>
          Password is incorrect or ciphertext is invalid.
        </Alert.Description>
      {/if}
    </Alert.Root>
  {/await}
{/if}
