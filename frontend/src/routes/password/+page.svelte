<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let ciphertext = "";
  let password = "";
  let plaintext: Promise<string> | null = null;

  if (location.hash) {
    ciphertext = location.hash.slice(1);
  }

  async function decrypt(
    ciphertext: string,
    password: string
  ): Promise<string> {
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();

    const envelope = Cipherly.decodePasswordEnvelope(ciphertext);
    const key = await Cipherly.deriveKey(
      encoder.encode(password),
      envelope.salt
    );
    const secret = await Cipherly.decrypt(
      envelope.ciphertext,
      key,
      envelope.iv
    );

    return decoder.decode(secret);
  }
</script>

<h1 class="text-4xl font-extrabold">Password Based Decryption</h1>

<div class="mt-4">
  <Label for="plaintext">Ciphertext</Label>
  <Textarea
    id="plaintext"
    bind:value={ciphertext}
    placeholder="The ciphertext to be decrypted"
  />
</div>

<div class="mt-4">
  <Label for="plaintext">Password</Label>
  <div class="flex space-x-2">
    <Input
      type="password"
      placeholder="The password to use for decryption"
      bind:value={password}
    />
    <Button
      type="button"
      on:click={() => (plaintext = decrypt(ciphertext, password))}
    >
      Decrypt
    </Button>
  </div>
</div>

{#if plaintext}
  <Separator class="mt-8 mb-8" />
  {#await plaintext}
    <div class="mt-8">Decrypting...</div>
  {:then secret}
    <Label for="secret">Decrypted Plaintext</Label>
    <div id="secret" class="p-3 mb-2 border rounded-md font-mono">{secret}</div>
    <Button
      type="button"
      on:click={() => navigator.clipboard.writeText(secret)}
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
