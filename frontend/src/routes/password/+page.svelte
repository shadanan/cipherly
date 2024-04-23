<script lang="ts">
  import { passwordEncrypt } from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import EncryptionAlert from "$lib/components/EncryptionAlert.svelte";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import { Textarea } from "$lib/components/ui/textarea";
  import { getError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { z } from "zod";

  const PasswordEncryptFormSchema = z.object({
    plainText: z.instanceof(Uint8Array).refine((v) => v.length > 0),
    password: z.string().default(""),
  });

  type PasswordEncryptFormData = z.infer<typeof PasswordEncryptFormSchema>;

  let validationError: z.ZodError | null;
  let formData: PasswordEncryptFormData = {
    plainText: new Uint8Array(),
    password: "",
  };

  function validateFormData(formData: PasswordEncryptFormData): boolean {
    const validationResult = PasswordEncryptFormSchema.safeParse(formData);
    validationError = validationResult.success ? null : validationResult.error;
    return validationResult.success;
  }
  let payload: Promise<string> | null = null;
</script>

<div class="space-y-8">
  <Section title="Password Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        if (validateFormData(formData)) {
          payload = passwordEncrypt(formData.plainText, formData.password);
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

        <TextOrFileInput
          bind:data={formData.plainText}
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
            class="focus-visible:ring-none disabled:opacity-1 border-2  border-muted text-base focus-visible:outline-none disabled:cursor-text disabled:text-green-600"
            id="payload"
            disabled
            value={payload}
            placeholder="The plain text secret to encrypt"
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
