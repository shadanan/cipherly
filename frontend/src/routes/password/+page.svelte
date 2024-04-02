<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";

  let payload = "";
  let password = "";
  let plainText: Promise<string> | null = null;

  if (location.hash) {
    payload = location.href;
  }

  async function decrypt(payload: string, password: string): Promise<string> {
    const { salt, iv, cipherText } = Cipherly.decodePasswordPayload(payload);
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);
    const plainText = await Cipherly.decrypt(cipherText, key, iv);
    return Cipherly.decodeUtf8(plainText);
  }
</script>

<div class="space-y-8">
  <div
    class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
  >
    <div>
      <h1 class="text-xl font-bold text-foreground">
        Password Based Decryption
      </h1>
    </div>

    <form
      class="space-y-6"
      on:submit|preventDefault={() => (plainText = decrypt(payload, password))}
    >
      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="payload">Ciphertext Payload</Label
        >
        <Textarea
          id="payload"
          required
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          bind:value={payload}
          placeholder="The ciphertext payload to be decrypted"
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
          type="password"
          placeholder="The password to use for decryption"
          bind:value={password}
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg font-bold" type="submit">
          Decrypt
        </Button>
      </div>
    </form>
  </div>

  {#if plainText}
    <div
      class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
    >
      <div>
        <h1 class="text-xl font-bold text-foreground">Decrypted Content</h1>
      </div>
      {#await plainText}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
        <div class="space-y-2">
          <Label
            class="text-background-foreground text-sm uppercase tracking-wider"
            for="plainText"
          >
            Decrypted Plaintext
          </Label>
          <Textarea
            class="disabled:opacity-1 border-2 border-muted text-base focus-visible:outline-none focus-visible:ring-0 disabled:cursor-text disabled:text-green-600"
            id="plainText"
            disabled
            value={plainText}
            placeholder="The decrypted plaintext"
          />
        </div>
        <CopyText label="Plaintext" text={plainText} />
      {:catch}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>
            Password is incorrect or ciphertext is invalid.
          </Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
