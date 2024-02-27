<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let secret = "";
  let password = "";
  let ciphertext: Promise<string> | null = null;

  async function encrypt(secret: string, password: string): Promise<string> {
    const encoder = new TextEncoder();

    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(encoder.encode(password), salt);

    const iv = Cipherly.generateIv();
    const ciphertext = await Cipherly.encrypt(encoder.encode(secret), key, iv);

    const envelope = Cipherly.encodePasswordEnvelope({ salt, iv, ciphertext });
    return envelope;
  }
</script>

<h1 class="text-4xl font-extrabold">Password Based Encryption</h1>

<div class="mt-4">
  <Label for="plaintext">Plaintext</Label>
  <Textarea
    id="plaintext"
    bind:value={secret}
    placeholder="The plaintext secret to encrypt"
  />
</div>

<div class="mt-4">
  <Label for="plaintext">Password</Label>
  <div class="flex space-x-2">
    <Input
      type="password"
      placeholder="The password to use for encryption"
      bind:value={password}
    />
    <Button
      type="button"
      on:click={() => (ciphertext = encrypt(secret, password))}
    >
      Encrypt
    </Button>
  </div>
</div>

{#if ciphertext}
  <Separator class="mt-8 mb-8" />
  {#await ciphertext}
    <div class="mt-8">Encrypting...</div>
  {:then envelope}
    {@const url = Cipherly.passwordUrl() + envelope}
    <Label for="secret">Encrypted Ciphertext</Label>
    <div id="secret" class="p-3 mb-2 border rounded-md font-mono">
      <a href={url}>{envelope}</a>
    </div>
    <Button
      type="button"
      on:click={() => navigator.clipboard.writeText(envelope)}
    >
      Copy Ciphertext
    </Button>
    <Button type="button" on:click={() => navigator.clipboard.writeText(url)}>
      Copy Decrypt URL
    </Button>
  {:catch error}
    <Alert.Root variant="destructive">
      <Alert.Title>Failed to Encrypt</Alert.Title>
      <Alert.Description>
        {error.message}
      </Alert.Description>
    </Alert.Root>
  {/await}
{/if}
