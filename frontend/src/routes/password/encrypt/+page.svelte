<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let plaintext = "";
  let password = "";
  let envelope: Promise<string> | null = null;

  async function encrypt(plaintext: string, password: string): Promise<string> {
    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);

    const iv = Cipherly.generateIv();
    const ciphertext = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plaintext),
      key,
      iv
    );

    return Cipherly.encodePasswordEnvelope({ salt, iv, ciphertext });
  }
</script>

<h1 class="text-4xl font-extrabold">Password Based Encryption</h1>

<div class="mt-4">
  <Label for="plaintext">Plaintext</Label>
  <Textarea
    id="plaintext"
    bind:value={plaintext}
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
      on:click={() => (envelope = encrypt(plaintext, password))}
    >
      Encrypt
    </Button>
  </div>
</div>

{#if envelope}
  <Separator class="mt-8 mb-8" />
  {#await envelope}
    <div class="mt-8">Encrypting...</div>
  {:then envelope}
    {@const url = Cipherly.passwordUrl() + envelope}
    <Label for="envelope">Ciphertext Envelope</Label>
    <div id="envelope" class="p-3 mb-2 border rounded-md font-mono">
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
