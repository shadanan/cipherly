<script lang="ts">
  import { renderLoginButton, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import EncryptionAlert from "$lib/components/EncryptionAlert.svelte";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { getError, hasError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { onMount } from "svelte";
  import { z } from "zod";
  import Auth from "./auth.svelte";
  import Password from "./password.svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  const DecryptFormSchema = z
    .object({
      text: z.string(),
      file: z.instanceof(File).nullable(),
    })
    .transform(async ({ text, file }, ctx) => {
      const expectedHostPath = location.href.split("#", 2)[0];
      if (file !== null) {
        const data = new Uint8Array(await file.arrayBuffer());
        const endOfUrl = data.indexOf(0x23);
        if (endOfUrl === -1) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "Payload is missing URL header",
            path: ["file"],
          });
          return null;
        }
        const hostPath = Cipherly.decodeUtf8(data.subarray(0, endOfUrl));
        if (hostPath !== expectedHostPath) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "Payload is not intended for this Cipherly instance",
            path: ["file"],
          });
          return null;
        }
        const payload = Cipherly.decodePayload(data.subarray(endOfUrl + 1));
        return payload;
      }

      if (text.length > 0) {
        const [hostPath, encodedPayload] = text.split("#", 2);
        if (hostPath !== expectedHostPath) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "Payload is not intended for this Cipherly instance",
            path: ["text"],
          });
          return null;
        }
        const payload = Cipherly.decodePayload(
          Cipherly.decodeBase64(encodedPayload),
        );
        return payload;
      }

      return null;
    })
    .refine(
      (payload) =>
        Cipherly.isPasswordPayload(payload) || Cipherly.isAuthPayload(payload),
      "Invalid Cipherly payload",
    );
  type DecryptFormData = z.input<typeof DecryptFormSchema>;
  type DecryptPayload = z.output<typeof DecryptFormSchema>;

  let formData: DecryptFormData = {
    text: location.hash ? location.href : "",
    file: null,
  };
  let payload: DecryptPayload = null;
  $: {
    formData;
    DecryptFormSchema.safeParseAsync(formData).then(
      (p) => (payload = p.success ? p.data : null),
    );
  }

  let validationError: z.ZodError | null;
  let password = "";
  let plainText: Promise<Uint8Array> | null = null;

  async function decrypt(payload: DecryptPayload): Promise<Uint8Array> {
    console.log(payload);
    if (payload?.es === Cipherly.EncryptionScheme.Auth) {
      return Cipherly.authDecrypt(payload, $token!);
    }
    if (payload?.es === Cipherly.EncryptionScheme.Password) {
      return Cipherly.passwordDecrypt(payload, password);
    }
    throw new Error("Invalid Encryption Scheme");
  }

  function encryptionTitle(payload: Cipherly.Payload | null): string {
    let baseTitle = "Decrypt";
    if (!payload) return baseTitle;
    const scheme = Cipherly.EncryptionScheme[payload.es];
    if (!scheme) return baseTitle;
    return `${scheme} ${baseTitle}`;
  }
</script>

<div class="space-y-8">
  <Section title={encryptionTitle(payload)}>
    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        plainText = null;
        if (validationError) return;
        plainText = decrypt(payload);
      }}
    >
      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="payload"
        >
          Ciphertext Payload
        </Label>

        {#if hasError(validationError, "payload")}
          {@const payloadErr = getError(validationError, "payload")}
          <p class="flex items-center space-x-1 text-xs text-destructive">
            <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
            <span>{payloadErr}</span>
          </p>
        {/if}

        <TextOrFileInput
          bind:text={formData.text}
          bind:file={formData.file}
          placeholder="ciphertext payload"
        />
      </div>

      {#if payload?.es === Cipherly.EncryptionScheme.Password}
        <Password bind:value={password} />
      {:else if payload?.es === Cipherly.EncryptionScheme.Auth}
        <Auth />
      {/if}

      <div class="pt-4">
        <Button class="min-w-[140px] text-lg font-bold" type="submit">
          Decrypt
        </Button>
      </div>
    </form>
  </Section>

  {#if plainText}
    <Section title="Decrypted Content">
      {#await plainText}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
        <TextOrFileOutput data={[plainText]} name={payload?.fn} />
      {:catch error}
        <EncryptionAlert title="Failed to Decrypt" {error} />
      {/await}
    </Section>
  {/if}
</div>
