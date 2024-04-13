<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import Chip from "$lib/components/Chip.svelte";
  import CopyText from "$lib/components/CopyText.svelte";
  import Section from "$lib/components/Section.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";

  let emails: string[] = [];
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
</script>

<div class="space-y-8">
  <Section title="Auth Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        payload = encrypt(plainText, emails);
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
          for="emails"
          class="text-background-foreground text-sm uppercase tracking-wider"
        >
          Authorized Emails
        </Label>
        <Chip
          id="emails"
          bind:values={emails}
          placeholder="List of email addresses authorized to decrypt"
        />
      </div>

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg font-bold" type="submit">
          Encrypt
        </Button>
      </div>
    </form>
  </Section>

  {#if payload}
    <Section title="Encrypted Content">
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
    </Section>
  {/if}
</div>
