<script lang="ts">
  import { authEncrypt } from "$lib/cipherly";
  import Chip from "$lib/components/Chip.svelte";
  import CopyText from "$lib/components/CopyText.svelte";
  import EncryptionAlert from "$lib/components/EncryptionAlert.svelte";
  import Section from "$lib/components/Section.svelte";
  import PlainTextInput from "$lib/components/PlainTextInput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { getError, hasError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { z } from "zod";

  const AuthEncryptFormSchema = z
    .object({
      emails: z.array(z.string().email()).min(1),
      plainText: z.string().optional(),
      plainFile: z.custom<File | undefined>(),
    })
    .refine(({ plainFile, plainText }) => !!plainText || !!plainFile, {
      message: "Either a plain text or a file must be provided.",
      path: ["plainText"],
    });
  type AuthEncryptFormData = z.infer<typeof AuthEncryptFormSchema>;

  let validationError: z.ZodError | null;
  let formData: AuthEncryptFormData = {
    plainText: "",
    plainFile: undefined,
    emails: [],
  };

  function validateFormData(formData: AuthEncryptFormData): boolean {
    const validationResult = AuthEncryptFormSchema.safeParse(formData);
    validationError = validationResult.success ? null : validationResult.error;
    return validationResult.success;
  }

  let payload: Promise<string> | null = null;
</script>

<div class="space-y-8">
  <Section title="Auth Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        if (validateFormData(formData)) {
          payload = authEncrypt(
            formData.plainText,
            formData.plainFile,
            formData.emails,
          );
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

        <PlainTextInput
          bind:plainText={formData.plainText}
          bind:plainFile={formData.plainFile}
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
        <EncryptionAlert {error} />
      {/await}
    </Section>
  {/if}
</div>
