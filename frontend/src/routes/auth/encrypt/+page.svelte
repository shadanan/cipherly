<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Info, X } from "lucide-svelte";

  let authorizedEmails = new Set<string>([]);
  let email = "";
  let plainText = "";
  let payload: Promise<string> | null = null;

  async function encrypt(plainText: string, emails: string[]): Promise<string> {
    const dek = await Cipherly.generateKey();
    const iv = Cipherly.generateIv();
    const cipherText = await Cipherly.encrypt(
      Cipherly.encodeUtf8(plainText),
      dek,
      iv,
    );
    const { kid, nonce, data } = await Cipherly.seal({ dek, emails });
    return Cipherly.encodeAuthPayload({
      k: kid,
      n: nonce,
      se: data,
      iv: iv,
      ct: cipherText,
    });
  }

  function appendAndClearEmail() {
    if (email === "") {
      return;
    }
    authorizedEmails = new Set([...authorizedEmails, email]);
    email = "";
  }
</script>

<div class="space-y-8">
  <div
    class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
  >
    <div>
      <h1 class="text-xl font-bold text-foreground">
        Authorization Based Encryption
      </h1>
    </div>

    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        appendAndClearEmail();
        payload = encrypt(plainText, Array.from(authorizedEmails));
      }}
    >
      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="plainText"
        >
          Plaintext
        </Label>
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
          for="email"
          class="text-background-foreground text-sm uppercase tracking-wider"
        >
          Authorized Emails
        </Label>

        <p class="flex items-center space-x-1 text-xs text-blue-500">
          <Info class="inline-block h-[12px] w-[12px]"></Info>
          <span>Click Enter after typing each email</span>
        </p>
        <Input
          id="email"
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          placeholder="Recipient email address"
          type="email"
          required={authorizedEmails.size === 0}
          bind:value={email}
          on:keydown={(e) => {
            if (e.key === "Enter") {
              appendAndClearEmail();
              e.preventDefault();
            }
          }}
        />

        {#if authorizedEmails.size > 0}
          <div class="flex flex-wrap pt-2">
            {#each Array.from(authorizedEmails) as email}
              <Badge variant="secondary" class="mb-2 mr-2 space-x-1 text-sm">
                <span>{email}</span>
                <Button
                  class="m-0 h-4 w-4 p-0 "
                  variant="ghost"
                  on:click={() => {
                    authorizedEmails = new Set(
                      Array.from(authorizedEmails).filter((x) => x !== email),
                    );
                  }}
                >
                  <X class="cursor-pointer text-gray-400 hover:text-gray-500" />
                </Button>
              </Badge>
            {/each}
          </div>
        {:else}
          <p class="text-sm text-gray-400">No emails selected</p>
        {/if}
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
      class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
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
            class="disabled:opacity-1 border-2 border-muted text-base focus-visible:outline-none focus-visible:ring-0 disabled:cursor-text disabled:text-green-600"
            id="payload"
            disabled
            value={payload}
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
