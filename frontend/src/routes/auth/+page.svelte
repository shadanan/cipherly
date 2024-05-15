<script lang="ts">
  import {
    authEncrypt,
    decryptUrl,
    encodeBase64,
    encodeUtf8,
  } from "$lib/cipherly";
  import Chip from "$lib/components/Chip.svelte";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { getError, hasError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { z } from "zod";

  const AuthEncryptFormSchema = z
    .object({
      text: z.string(),
      file: z.instanceof(File).nullable(),
      emails: z.array(z.string().email()).min(1),
    })
    .transform(async ({ text, file, emails }, ctx) => {
      let data: Uint8Array;
      if (file !== null) {
        data = new Uint8Array(await file.arrayBuffer());
      } else if (text.length > 0) {
        data = encodeUtf8(text);
      } else {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Either text or file input must be present",
          path: ["text", "file"],
        });
        return z.NEVER;
      }
      return { data, filename: file?.name, emails };
    });
  type AuthEncryptFormData = z.input<typeof AuthEncryptFormSchema>;

  let formData: AuthEncryptFormData = {
    text: "",
    file: null,
    emails: [],
  };

  let validationError: z.ZodError | null;
  let payload: Promise<Uint8Array> | null = null;

  $: {
    formData;
    payload = null;
    validationError = null;
  }
</script>

<div class="space-y-8">
  <Section title="Auth Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={async () => {
        const result = await AuthEncryptFormSchema.safeParseAsync(formData);
        validationError = result.success ? null : result.error;
        if (!result.success) {
          validationError = result.error;
          payload = null;
          return;
        }
        validationError = null;
        payload = authEncrypt(
          result.data.data,
          result.data.emails,
          result.data.filename,
        );
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

        <TextOrFileInput
          bind:text={formData.text}
          bind:file={formData.file}
          placeholder="plaintext secret"
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
    <TextOrFileOutput
      kind="Encrypt"
      data={Promise.all([
        decryptUrl(),
        formData.file ? payload : payload.then(encodeBase64).then(encodeUtf8),
      ])}
    />
  {/if}
</div>
