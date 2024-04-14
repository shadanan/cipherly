<script lang="ts">
  import { z } from "zod";
  import { AlertCircle } from "lucide-svelte";
  import * as Cipherly from "$lib/cipherly";
  import Chip from "$lib/components/Chip.svelte";
  import CopyText from "$lib/components/CopyText.svelte";
  import Section from "$lib/components/Section.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { getError, hasError } from "$lib/form";

  const AuthEncryptFormSchema = z.object({
    emails: z.array(z.string().email().endsWith("@gmail.com")),
    plainText: z.string().min(1).max(20),
  });
  type AuthEncryptFormData = z.infer<typeof AuthEncryptFormSchema>;

  let validationError: z.ZodError | null;
  let formData: AuthEncryptFormData = {
    plainText: "",
    emails: [],
  };

  function validateFormData(formData: AuthEncryptFormData): boolean {
    const validationResult = AuthEncryptFormSchema.safeParse(formData);
    validationError = validationResult.success ? null : validationResult.error;
    return validationResult.success;
  }

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
        if (validateFormData(formData)) {
          payload = encrypt(formData.plainText, formData.emails);
        }
      }}
    >
      <div class="space-y-2">
        <Label
          for="plainText"
          class="text-background-foreground text-sm uppercase tracking-wider"
        >
          Plaintext
        </Label>

        <!-- PlainText Validation Error  -->
        {#if getError(validationError, "plainText")}
          <p class="flex items-center space-x-1 text-xs text-destructive">
            <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
            <span>{getError(validationError, "plainText")}</span>
          </p>
        {/if}

        <Textarea
          required
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          id="plainText"
          bind:value={formData.plainText}
          placeholder="The plaintext secret to encrypt"
        />
      </div>

      <div class="space-y-2">
        <!-- Emails Label -->
        <Label
          for="emails"
          class="text-background-foreground text-sm uppercase tracking-wider"
        >
          Authorized Emails
        </Label>

        <!-- Emails Validation Error -->
        {#if hasError(validationError, "emails")}
          <p class="flex items-center space-x-1 text-xs text-destructive">
            <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
            <span>
              {getError(validationError, "emails")}: {formData.emails.join(
                ", ",
              )}
            </span>
          </p>
        {/if}

        <!-- Emails Input Chip  -->
        <Chip
          id="emails"
          bind:values={formData.emails}
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
