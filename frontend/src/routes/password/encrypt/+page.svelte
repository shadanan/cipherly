<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";

  let plainText = "";
  let password = "";
  let envelope: Promise<string> | null = null;
  let copiedCipherText: boolean;
  let copiedDecryptUrl: boolean;

  async function encrypt(plainText: string, password: string): Promise<string> {
    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);

    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(Cipherly.encodeUtf8(plainText), key, iv);

    return Cipherly.encodePasswordPayload({ salt, iv, cipherText });
  }

  function copyCipherText(envelope: string | null): void {
    if (!envelope) return;
    navigator.clipboard.writeText(envelope);
    copiedCipherText = true;
    setTimeout(() => {
      copiedCipherText = false;
    }, 500);
  }

  function copyDecryptUrl(url: string | null): void {
    if (!url) return;
    navigator.clipboard.writeText(url);
    copiedDecryptUrl = true;
    setTimeout(() => {
      copiedDecryptUrl = false;
    }, 500);
  }
</script>

<div class="space-y-8">
  <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
    <div>
      <h1 class="text-xl font-bold">Password based encryption</h1>
      <p class="text-base text-gray-400">
        Nostrud ad in nulla nisi incididunt dolor sint proident dolore qui labore aute.
      </p>
    </div>

    <form class="space-y-6" on:submit|preventDefault={() => (envelope = encrypt(plainText, password))}>
      <div class="space-y-2">
        <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plainText">Plaintext</Label>
        <Textarea
          required
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          id="plainText"
          bind:value={plainText}
          placeholder="The plainText secret to encrypt"
        />
      </div>

      <div class="space-y-2">
        <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plainText">Password</Label>
        <Input
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          placeholder="The password to use for encryption"
          bind:value={password}
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg" type="submit">Encrypt</Button>
      </div>
    </form>
  </div>

  {#if envelope}
    <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
      <div>
        <h1 class="text-xl font-bold">Encrypted content</h1>
      </div>

      {#await envelope}
        <div class="py-6 space-y-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then envelope}
        {@const url = Cipherly.passwordUrl() + envelope}

        <div class="space-y-2">
          <Label for="envelope" class="uppercase text-gray-500 tracking-wider text-sm">Ciphertext Envelope</Label>
          <Textarea
            class="text-base border-2 border-gray-300 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
            id="envelope"
            disabled
            value={envelope}
            placeholder="The plainText secret to encrypt"
          />
        </div>
        <div class="pt-4 space-x-2">
          <Button
            class="min-w-[140px]"
            variant="secondary"
            type="button"
            on:click={() => copyCipherText(envelope)}
          >
            {#if copiedCipherText}
              Copied!
            {:else}
              Copy Ciphertext
            {/if}
          </Button>
          <Button variant="secondary" class="min-w-[140px]" type="button" on:click={() => copyDecryptUrl(url)}>
            {#if copiedDecryptUrl}
              Copied!
            {:else}
              Copy Decrypt URL
            {/if}
          </Button>
        </div>
      {:catch error}
        <Alert.Root variant="destructive" class="space-y-2">
          <Alert.Title>Failed to Encrypt</Alert.Title>
          <Alert.Description>
            {error.message}
          </Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
