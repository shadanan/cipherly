<script lang="ts">
  import { authEncrypt, encodePayload } from "$lib/cipherly";
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
      data: z.instanceof(Uint8Array),
      filename: z.string().nullable(),
      emails: z.array(z.string().email()).min(1),
    })
    .refine(({ data, filename }) => data.length !== 0 || filename !== null, {
      message: "Either text or file input must be present",
      path: ["plainText"],
    });
  type AuthEncryptFormData = z.input<typeof AuthEncryptFormSchema>;

  let formData: AuthEncryptFormData = {
    data: new Uint8Array(),
    filename: null,
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
      on:submit|preventDefault={() => {
        const result = AuthEncryptFormSchema.safeParse(formData);
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
          result.data.filename ? result.data.filename : undefined,
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
          bind:data={formData.data}
          bind:filename={formData.filename}
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
      data={payload.then((data) => encodePayload(data, !!formData.filename))}
      name={formData.filename ? formData.filename + ".cly" : null}
    />
  {/if}
</div>
