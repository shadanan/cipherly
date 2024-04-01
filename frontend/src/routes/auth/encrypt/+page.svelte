<script lang="ts">
  import { Badge } from "$/lib/components/ui/badge";
  import { Skeleton } from "$/lib/components/ui/skeleton";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Info, X } from "lucide-svelte";

  let authorizedEmails = new Set<string>([]);
  let email = "";
  let plaintext = "";
  let payload: Promise<string> | null = null;
  let copiedCipherText: boolean;
  let copiedDecryptUrl: boolean;

  async function encrypt(plainText: string, emails: string[]): Promise<string> {
    const dek = await Cipherly.generateKey();
    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(Cipherly.encodeUtf8(plainText), dek, iv);
    const sealedEnvelope = await Cipherly.seal({ dek, iv, emails });
    return Cipherly.encodeAuthPayload({ sealedEnvelope, cipherText });
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
  <div class="p-8 border-2 border-gray-300 rounded-md space-y-6 bg-white">
    <div>
      <h1 class="text-xl font-bold">Authorization based encryption</h1>
    </div>

    <form
      class="space-y-6"
      on:submit|preventDefault={() => (payload = encrypt(plaintext, Array.from(authorizedEmails)))}
    >
      <div class="space-y-2">
        <Label class="uppercase text-gray-500 tracking-wider text-sm" for="plaintext">Ciphertext Envelope</Label>
        <Textarea
          required
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          id="plaintext"
          bind:value={plaintext}
          placeholder="The plaintext secret to encrypt"
        />
      </div>

      <div class="space-y-2">
        <Label for="email" class="uppercase text-gray-500 tracking-wider text-sm">Authorized Emails</Label>

        <p class="text-xs flex items-center space-x-1 text-blue-500">
          <Info class="h-[12px] w-[12px] inline-block"></Info>
          <span>Click Enter after typing each email</span>
        </p>
        <Input
          id="email"
          class="text-base border-2 border-gray-300 focus-visible:ring-0"
          placeholder="Recipient email address"
          type="email"
          required={authorizedEmails.size === 0}
          bind:value={email}
          on:keydown={(e) => {
            if (e.key === "Enter") {
              authorizedEmails = new Set([...authorizedEmails, email]);
              email = "";
              e.preventDefault();
            }
          }}
        />

        {#if authorizedEmails.size > 0}
          <div class="pt-2 flex flex-wrap">
            {#each Array.from(authorizedEmails) as email}
              <Badge variant="secondary" class="text-sm space-x-1 mr-2 mb-2">
                <span>{email}</span>
                <Button
                  class="h-4 w-4 p-0 m-0 "
                  variant="ghost"
                  on:click={() => {
                    console.log("clicked!", authorizedEmails);
                    authorizedEmails = new Set(Array.from(authorizedEmails).filter((x) => x !== email));
                  }}
                >
                  <X class="text-gray-400 cursor-pointer hover:text-gray-500" />
                </Button>
              </Badge>
            {/each}
          </div>
        {:else}
          <p class="text-sm text-gray-400">No emails selected</p>
        {/if}
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg" type="submit">Encrypt</Button>
      </div>
    </form>
  </div>

  {#if payload}
    <div class="p-8 border-2 border-gray-300 rounded-md space-y-6 bg-white">
      <div>
        <h1 class="text-xl font-bold">Encrypted content</h1>
      </div>

      {#await payload}
        <div class="py-6 space-y-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then payload}
        {@const url = Cipherly.authUrl() + payload}

        <div class="space-y-2">
          <Label for="payload" class="uppercase text-gray-500 tracking-wider text-sm">Ciphertext Envelope</Label>
          <Textarea
            class="text-base border-2 border-gray-300 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
            id="payload"
            disabled
            value={payload}
          />
        </div>

        <div class="pt-4 space-x-2">
          <Button class="min-w-[140px]" variant="secondary" type="button" on:click={() => copyCipherText(payload)}>
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
        <Alert.Root variant="destructive">
          <Alert.Title>Failed to Encrypt</Alert.Title>
          <Alert.Description>
            {error.message}
          </Alert.Description>
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
