<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

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

<h1 class="text-4xl font-extrabold">Password Based Decryption</h1>

<div class="mt-4">
  <Label for="payload">Ciphertext Payload</Label>
  <Textarea
    id="payload"
    bind:value={payload}
    placeholder="The ciphertext payload to be decrypted"
  />
</div>

<div class="mt-4">
  <Label for="password">Password</Label>
  <div class="flex space-x-2">
    <Input
      type="password"
      placeholder="The password to use for decryption"
      bind:value={password}
    />
    <Button
      type="button"
      on:click={() => (plainText = decrypt(payload, password))}
    >
      Decrypt
    </Button>
  </div>
</div>

{#if plainText}
  <Separator class="mt-8 mb-8" />
  {#await plainText}
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
