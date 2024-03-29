<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import { Textarea } from "$lib/components/ui/textarea";

  let plainText = "";
  let email = "";
  let payload: Promise<string> | null = null;

  async function encrypt(plainText: string, email: string): Promise<string> {
    const dek = await Cipherly.generateKey();
    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plainText),
      dek,
      iv
    );
    const sealedEnvelope = await Cipherly.seal({ dek, iv, emails: [email] });
    return Cipherly.encodeAuthPayload({ sealedEnvelope, cipherText });
  }
</script>

<h1 class="text-4xl font-extrabold">Authorization Based Encryption</h1>

<div class="mt-4">
  <Label for="plainText">Plaintext</Label>
  <Textarea
    id="plainText"
    bind:value={plainText}
    placeholder="The plaintext secret to encrypt"
  />
</div>

<div class="mt-4">
  <Label for="emails">Authorized Emails</Label>
  <div class="flex space-x-2">
    <Input
      id="emails"
      type="text"
      placeholder="Emails of the users authorized to decrypt the secret"
      bind:value={email}
    />
    <Button
      type="button"
      on:click={() => (payload = encrypt(plainText, email))}
    >
      Encrypt
    </Button>
  </div>
</div>

{#if payload}
  <Separator class="mt-8 mb-8" />
  {#await payload}
    <div class="mt-8">Encrypting...</div>
  {:then payload}
    {@const url = Cipherly.authUrl() + payload}
    <Label for="payload">Ciphertext Payload</Label>
    <div id="payload" class="p-3 mb-2 border rounded-md font-mono">
      <a href={url}>{payload}</a>
    </div>
    <Button
      type="button"
      on:click={() => navigator.clipboard.writeText(payload)}
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
        {(console.log({ error }), "")}
      </Alert.Description>
    </Alert.Root>
  {/await}
{/if}
