<script lang="ts">
  import { Skeleton } from "$/lib/components/ui/skeleton";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";

  let payload = "";
  let password = "";
  let plaintext: Promise<string> | null = null;
  let copiedPlaintext: boolean;
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

  function copyPlaintext(url: string | null): void {
    if (!url) return;
    navigator.clipboard.writeText(url);
    copiedPlaintext = true;
    setTimeout(() => {
      copiedPlaintext = false;
    }, 500);
  }
</script>

<div class="space-y-8">
  <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
    <div>
      <h1 class="text-xl font-bold">Password based decryption</h1>
      <p class="text-base text-gray-400">
        Nostrud ad in nulla nisi incididunt dolor sint proident dolore qui labore aute.
      </p>
    </div>

    <form class="space-y-6" on:submit|preventDefault={() => (plaintext = decrypt(envelope, password))}>
      <div class="space-y-2">
        <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plaintext">Ciphertext Envelope</Label>
        <Textarea
          required
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          id="envelope"
          bind:value={envelope}
          placeholder="The ciphertext envelope to be decrypted"
        />
      </div>

      <div class="space-y-2">
        <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plaintext">Password</Label>
        <Input
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          type="password"
          placeholder="The password to use for decryption"
          bind:value={password}
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg" type="submit">Decrypt</Button>
      </div>
    </form>
  </div>

  {#if plaintext}
    <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
      <div>
        <h1 class="text-xl font-bold">Encrypted content</h1>
      </div>
      {#await plaintext}
        <div class="py-6 space-y-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plaintext}
        <div class="space-y-2">
          <Label class="uppercase text-gray-500 tracking-wider text-sm" for="envelope">Decrypted Plaintext</Label>
          <Textarea
            class="text-base border-2 border-gray-300 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
            id="plaintext"
            disabled
            value={plaintext}
            placeholder="The decrypted plaintext"
          />
        </div>
        <Button variant="secondary" class="min-w-[140px]" type="button" on:click={() => copyPlaintext(plaintext)}>
          {#if copiedPlaintext}
            Copied!
          {:else}
            Copy Plaintext
          {/if}
        </Button>
      {:catch}
        <Alert.Root variant="destructive" class="space-y-2">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>Password is incorrect or ciphertext is invalid.</Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
