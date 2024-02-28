<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let envelope = "";
  let password = "";
  let plaintext: Promise<string> | null = null;

  if (location.hash) {
    envelope = location.hash.slice(1);
  }

  async function decrypt(envelope: string, password: string): Promise<string> {
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();

    const { salt, iv, ciphertext } = Cipherly.decodePasswordEnvelope(envelope);
    const key = await Cipherly.deriveKey(encoder.encode(password), salt);
    const secret = await Cipherly.decrypt(ciphertext, key, iv);

    return decoder.decode(secret);
  }
</script>

<h1 class="text-4xl font-extrabold">Password Based Decryption</h1>

<div class="mt-4">
  <Label for="envelope">Ciphertext Envelope</Label>
  <Textarea
    id="envelope"
    bind:value={envelope}
    placeholder="The ciphertext envelope to be decrypted"
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
      on:click={() => (plaintext = decrypt(envelope, password))}
    >
      Decrypt
    </Button>
  </div>
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
