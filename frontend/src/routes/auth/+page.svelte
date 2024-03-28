<script lang="ts">
  import { logout, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let envelope = "";
  let plaintext: Promise<string> | null = null;

  if (location.hash) {
    envelope = location.hash.slice(1);
  }

  async function decrypt(encodedEnvelope: string): Promise<string> {
    if ($token === null) {
      logout();
      throw new Error("Cannot decrypt without logging in.");
    }
    const envelope = Cipherly.decodeAuthEnvelope(encodedEnvelope);
    const header = await Cipherly.kekDecrypt(envelope.kekEncryptedDek, $token);
    const plaintext = await Cipherly.decrypt(
      envelope.cipherText,
      header.dek,
      header.iv
    );
    return Cipherly.decodeUtf8(plaintext);
  }
</script>

<h1 class="text-4xl font-extrabold">Authorization Based Decryption</h1>

<div class="mt-4">
  <Label for="envelope">Ciphertext Envelope</Label>
  <Textarea
    id="envelope"
    bind:value={envelope}
    placeholder="The ciphertext envelope to be decrypted"
  />
</div>

<div class="mt-4">
  <Button type="button" on:click={() => (plaintext = decrypt(envelope))}>
    Decrypt
  </Button>
</div>

{#if plaintext}
  <Separator class="mt-8 mb-8" />
  {#await plaintext}
    <div class="mt-8">Decrypting...</div>
  {:then plaintext}
    <Label for="plaintext">Decrypted Plaintext</Label>
    <div id="plaintext" class="p-3 mb-2 border rounded-md font-mono">
      {plaintext}
    </div>
    <Button
      type="button"
      on:click={() => navigator.clipboard.writeText(plaintext)}
    >
      Copy Plaintext
    </Button>
  {:catch}
    <Alert.Root variant="destructive">
      <Alert.Title>Failed to Decrypt</Alert.Title>
      <Alert.Description>
        Password is incorrect or ciphertext is invalid.
      </Alert.Description>
    </Alert.Root>
  {/await}
{/if}
