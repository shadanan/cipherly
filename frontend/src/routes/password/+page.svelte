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
  let plainText: Promise<string> | null = null;
  let copiedPlaintext: boolean;

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
  <div class="p-8 border-2 border-background-foreground rounded-md space-y-6 bg-background">
    <div>
      <h1 class="text-xl font-bold text-foreground">Password based decryption</h1>
    </div>

    <form class="space-y-6" on:submit|preventDefault={() => (plainText = decrypt(payload, password))}>
      <div class="space-y-2">
        <Label class="uppercase text-background-foreground tracking-wider text-sm" for="plainText"
          >Ciphertext Envelope</Label
        >
        <Textarea
          required
          class="text-base border-2 border-muted text-foreground focus:ring-0 focus-visible:ring-0"
          id="payload"
          bind:value={payload}
          placeholder="The ciphertext payload to be decrypted"
        />
      </div>

      <div class="space-y-2">
        <Label class="uppercase text-background-foreground tracking-wider text-sm" for="plainText">Password</Label>
        <Input
          class="text-base border-2 border-muted text-foreground focus:ring-0 focus-visible:ring-0"
          type="password"
          placeholder="The password to use for decryption"
          bind:value={password}
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg font-bold" type="submit">Decrypt</Button>
      </div>
    </form>
  </div>

  {#if plainText}
    <div class="p-8 border-2 border-background-foreground rounded-md space-y-6 bg-background">
      <div>
        <h1 class="text-xl font-bold text-foreground">Encrypted content</h1>
      </div>
      {#await plainText}
        <div class="py-6 space-y-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
        <div class="space-y-2">
          <Label class="uppercase text-background-foreground tracking-wider text-sm" for="plainText">
            Decrypted Plaintext
          </Label>
          <Textarea
            class="text-base border-2 border-muted focus-visible:ring-0 focus-visible:outline-none disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
            id="plainText"
            disabled
            value={plainText}
            placeholder="The decrypted plainText"
          />
        </div>
        <Button variant="secondary" class="min-w-[140px]" type="button" on:click={() => copyPlaintext(plainText)}>
          {#if copiedPlaintext}
            Copied!
          {:else}
            Copy Plaintext
          {/if}
        </Button>
      {:catch}
        <Alert.Root variant="destructive" class="rounded space-y-2">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>Password is incorrect or ciphertext is invalid.</Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
