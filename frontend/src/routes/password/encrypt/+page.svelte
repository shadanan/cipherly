<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import { Textarea } from "$lib/components/ui/textarea";

  let plainText = "";
  let password = "";
  let payload: Promise<string> | null = null;

  async function encrypt(plainText: string, password: string): Promise<string> {
    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);

    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plainText),
      key,
      iv,
    );

    return Cipherly.encodePasswordPayload({ s: salt, iv: iv, ct: cipherText });
  }
</script>

<div class="space-y-8">
  <div
    class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
  >
    <div>
      <h1 class="text-xl font-bold text-foreground">
        Password Based Encryption
      </h1>
    </div>

    <form
      class="space-y-6"
      on:submit|preventDefault={() => (payload = encrypt(plainText, password))}
    >
      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="plainText">Plaintext</Label
        >
        <Textarea
          required
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          id="plainText"
          bind:value={plainText}
          placeholder="The plaintext secret to encrypt"
        />
      </div>

      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="password">Password</Label
        >
        <Input
          id="password"
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          placeholder="The password to use for encryption"
          bind:value={password}
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg font-bold" type="submit">
          Encrypt
        </Button>
      </div>
    </form>
  </div>

  {#if payload}
    <div
      class="border-background-foreground space-y-6 rounded-md border-2 bg-card p-8"
    >
      <div>
        <h1 class="text-xl font-bold text-foreground">Encrypted Content</h1>
      </div>

      {#await payload}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then payload}
        <div class="space-y-2">
          <Label
            for="payload"
            class="text-background-foreground text-sm uppercase tracking-wider"
          >
            Ciphertext Payload
          </Label>
          <Textarea
            class="focus-visible:ring-none disabled:opacity-1 border-2  border-muted text-base focus-visible:outline-none disabled:cursor-text disabled:text-green-600"
            id="payload"
            disabled
            value={payload}
            placeholder="The plain text secret to encrypt"
          />
        </div>
        <div class="space-x-2 pt-4">
          <CopyText label="Ciphertext" text={payload} />
        </div>
      {:catch error}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Failed to Encrypt</Alert.Title>
          <Alert.Description>
            {error.message}
          </Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
