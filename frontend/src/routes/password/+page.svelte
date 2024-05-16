<script lang="ts">
  import { encodePayload, encodeUtf8, passwordEncrypt } from "$lib/cipherly";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { getError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { z } from "zod";

  const PasswordEncryptFormSchema = z
    .object({
      text: z.string(),
      file: z.instanceof(File).nullable(),
      password: z.string().default(""),
    })
    .transform(async ({ text, file, password }, ctx) => {
      let data: Uint8Array;
      if (file !== null) {
        data = new Uint8Array(await file.arrayBuffer());
      } else if (text.length > 0) {
        data = encodeUtf8(text);
      } else {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Either text or file input must be present",
          path: ["plainText"],
        });
        return z.NEVER;
      }
      return { data, filename: file?.name, password };
    });
  type PasswordEncryptFormData = z.input<typeof PasswordEncryptFormSchema>;

  let formData: PasswordEncryptFormData = {
    text: "",
    file: null,
    password: "",
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
  <Section title="Password Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={async () => {
        const result = await PasswordEncryptFormSchema.safeParseAsync(formData);
        if (!result.success) {
          validationError = result.error;
          payload = null;
          return;
        }
        validationError = null;
        payload = passwordEncrypt(
          result.data.data,
          result.data.password,
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
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="password">Password</Label
        >
        <Input
          id="password"
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          placeholder="The password to use for encryption"
          bind:value={formData.password}
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
      data={payload.then((data) => encodePayload(data, !!formData.file))}
      name={formData.file ? formData.file.name + ".cly" : null}
    />
  {/if}
</div>
