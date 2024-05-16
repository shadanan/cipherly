<script lang="ts">
  import { renderLoginButton, token } from "$lib/auth";
  import {
    EncryptionScheme,
    authDecrypt,
    decodePayload,
    isAuthPayload,
    isPasswordPayload,
    passwordDecrypt,
    type Payload,
  } from "$lib/cipherly";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import ValidationError from "$lib/components/ValidationError.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { onMount } from "svelte";
  import { z } from "zod";
  import Auth from "./auth.svelte";
  import Password from "./password.svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  const DecryptFormSchema = z
    .object({
      data: z.instanceof(Uint8Array),
      filename: z.string().nullable(),
    })
    .transform(({ data, filename }, ctx) => {
      if (data.length !== 0 || filename !== null) {
        try {
          return decodePayload(data, !!filename);
        } catch (error) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "Invalid Cipherly payload",
            path: ["payload"],
            fatal: true,
          });
        }
      }
      return null;
    })
    .refine(
      (payload) =>
        payload === null ||
        isPasswordPayload(payload) ||
        isAuthPayload(payload),
      { message: "Invalid Cipherly payload", path: ["payload"] },
    );
  type DecryptFormData = z.input<typeof DecryptFormSchema>;
  type DecryptPayload = z.output<typeof DecryptFormSchema>;

  let formData: DecryptFormData = {
    data: new Uint8Array(),
    filename: null,
  };
  let payload: DecryptPayload = null;
  let error: z.ZodError | null;
  let password = "";
  let plainText: Promise<Uint8Array[]> | null = null;

  $: {
    formData;
    plainText = null;
    DecryptFormSchema.safeParseAsync(formData).then((p) => {
      if (p.success) {
        payload = p.data;
        error = null;
      } else {
        payload = null;
        error = p.error;
      }
    });
  }

  async function decrypt(payload: DecryptPayload): Promise<Uint8Array[]> {
    if (payload?.es === EncryptionScheme.Auth) {
      return [await authDecrypt(payload, $token!)];
    }
    if (payload?.es === EncryptionScheme.Password) {
      return [await passwordDecrypt(payload, password)];
    }
    throw new Error("Invalid Encryption Scheme");
  }

  function encryptionTitle(payload: Payload | null): string {
    let baseTitle = "Decrypt";
    if (!payload) return baseTitle;
    const scheme = EncryptionScheme[payload.es];
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
        if (error) return;
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

        <ValidationError {error} path="payload" />
        <TextOrFileInput
          text={location.hash ? location.href : ""}
          bind:data={formData.data}
          bind:filename={formData.filename}
          placeholder="ciphertext payload"
        />
      </div>

      {#if payload?.es === EncryptionScheme.Password}
        <Password bind:value={password} />
      {:else if payload?.es === EncryptionScheme.Auth}
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
    <TextOrFileOutput kind="Decrypt" data={plainText} name={payload?.fn} />
  {/if}
</div>
