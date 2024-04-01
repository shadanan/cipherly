<script lang="ts">
  import { currentUser, login, logout, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import Google from "$lib/components/icons/Google.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Skeleton } from "$/lib/components/ui/skeleton";

  let payload = "";
  let plainText: Promise<string> | null = null;
  let copiedPlaintext: boolean;

  if (location.hash) {
    payload = location.hash.slice(1);
  }

  async function decrypt(payload: string): Promise<string> {
    if ($token === null) {
      logout();
      throw new Error("Cannot decrypt without logging in.");
    }
    const { sealedEnvelope, cipherText } = Cipherly.decodeAuthPayload(payload);
    const envelope = await Cipherly.unseal(sealedEnvelope, $token);
    const plainText = await Cipherly.decrypt(cipherText, envelope.dek, envelope.iv);
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
  $: console.log({ token: $token, currentUser: $currentUser });
</script>

<div class="space-y-8">
  <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
    <div>
      <h1 class="text-xl font-bold">Authorization based decryption</h1>
      <p class="text-base text-gray-400">
        Nostrud ad in nulla nisi incididunt dolor sint proident dolore qui labore aute.
      </p>
    </div>

    {#if $token === null}
      <div class="pt-4">
        <Button class="space-x-2 flex min-w-[140px] text-lg" variant="secondary" type="button" on:click={login}>
          <Google class="w-4" />
          <span>Login to decrypt</span>
        </Button>
      </div>
    {:else}
      <form class="space-y-6" on:submit|preventDefault={() => (plainText = decrypt(payload))}>
        <div class="space-y-2">
          <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plainText">Ciphertext Envelope</Label>
          <Textarea
            required
            class="text-base border-2 border-gray-300 focus-visible:ring-0"
            id="payload"
            bind:value={payload}
            placeholder="The ciphertext payload to be decrypted"
          />
        </div>

        <div class="pt-4">
          <Button class="min-w-[140px] text-lg" type="submit">Decrypt</Button>
        </div>
      </form>

      {#if plainText}
        <div class="p-10 border-2 border-gray-300 rounded-md space-y-6 bg-white">
          <div>
            <h1 class="text-xl font-bold">Encrypted content</h1>
          </div>
          {#await plainText}
            <div class="py-6 space-y-6">
              <Skeleton class="h-20 w-full" />
              <Skeleton class="h-10 w-full" />
            </div>
          {:then plainText}
            <div class="space-y-2">
              <Label for="plainText" class="uppercase text-gray-500 tracking-wider text-sm">
                Decrypted Plaintext
              </Label>
              <Textarea
                class="text-base border-2 border-gray-300 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
                id="plainText"
                disabled
                value={plainText}
                placeholder="The decrypted plainText"
              />
            </div>

            <Button
              variant="secondary"
              class="min-w-[140px]"
              type="button"
              on:click={() => copyPlaintext(plainText)}
            >
              {#if copiedPlaintext}
                Copied!
              {:else}
                Copy Plaintext
              {/if}
            </Button>
          {:catch err}
            <Alert.Root variant="destructive">
              {#if err.code === 401}
                <Alert.Title>Unauthorized</Alert.Title>
                <Alert.Description>You are not authorized to decrypt this secret.</Alert.Description>
              {:else}
                <Alert.Title>Failed to Decrypt</Alert.Title>
                <Alert.Description>Password is incorrect or ciphertext is invalid.</Alert.Description>
              {/if}
            </Alert.Root>
          {/await}
        </div>
      {/if}
    {/if}
  </div>
</div>
