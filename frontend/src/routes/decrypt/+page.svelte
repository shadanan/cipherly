<script lang="ts">
  import { renderLoginButton, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import Section from "$lib/components/Section.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { getError, hasError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";
  import { onMount } from "svelte";
  import { ZodError, z } from "zod";
  import Auth from "./auth.svelte";
  import Password from "./password.svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  const DecryptFormSchema = z
    .string()
    .min(1)
    .transform((encodedPayload, ctx) => {
      try {
        const payload = Cipherly.decodePayload(encodedPayload);
        if (
          !Cipherly.isPasswordPayload(payload) &&
          !Cipherly.isAuthPayload(payload)
        ) {
          throw `Invalid Cipherly payload: ${payload}`;
        }
        return payload;
      } catch (err) {
        console.log(err);
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Invalid Cipherly payload",
          path: ["payload"],
        });
      }
      return null;
    });
  type DecryptFormData = z.infer<typeof DecryptFormSchema>;

  let encodedPayload: string | null;
  let payload: DecryptFormData = null;

  let validationError: z.ZodError | null;
  let password = "";
  let loading = false;
  let plainText: string | null = null;
  let decryptionError: any = null;

  if (location.hash) {
    encodedPayload = location.href;
  }

  $: {
    decryptionError = null;
    plainText = null;
    if (encodedPayload && encodedPayload.trim() !== "") {
      try {
        payload = DecryptFormSchema.parse(encodedPayload);
        validationError = null;
      } catch (err) {
        validationError = err as ZodError;
        payload = null;
      }
    } else {
      validationError = null;
    }
  }

  async function decrypt(payload: DecryptFormData): Promise<string | null> {
    if (payload?.es === Cipherly.EncryptionScheme.Auth) {
      return await Cipherly.authDecrypt(payload, $token!);
    }
    if (payload?.es === Cipherly.EncryptionScheme.Password) {
      return await Cipherly.passwordDecrypt(payload, password);
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
      on:submit|preventDefault={async () => {
        plainText = null;
        if (validationError) return;

        loading = true;
        try {
          plainText = await decrypt(payload);
          decryptionError = null;
        } catch (e) {
          decryptionError = e;
        } finally {
          loading = false;
        }
      }}
    >
      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="payload">Ciphertext Payload</Label
        >

        {#if hasError(validationError, "payload")}
          {@const payloadErr = getError(validationError, "payload")}
          <p class="flex items-center space-x-1 text-xs text-destructive">
            <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
            <span>{payloadErr}</span>
          </p>
        {/if}

        <Textarea
          required
          minlength={1}
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          id="payload"
          bind:value={encodedPayload}
          placeholder="The ciphertext payload to be decrypted"
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

  {#if plainText || decryptionError}
    <Section title="Decrypted Content">
      {#if loading}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {/if}

      {#if decryptionError}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>
            {#if decryptionError.status === 401}
              Unauthorized
            {:else if decryptionError.name === "OperationError"}
              Incorrect Password
            {:else}
              {JSON.stringify(decryptionError)}
            {/if}
          </Alert.Description>
        </Alert.Root>
      {/if}

      {#if plainText}
        <div class="space-y-2">
          <Label
            class="text-background-foreground text-sm uppercase tracking-wider"
            for="plainText"
          >
            Decrypted Plaintext
          </Label>
          <Textarea
            class="disabled:opacity-1 border-2 border-muted text-base focus-visible:outline-none focus-visible:ring-0 disabled:cursor-text disabled:text-green-600"
            id="plainText"
            disabled
            value={plainText}
            placeholder="The decrypted plaintext"
          />
        </div>
        <CopyText label="Plaintext" text={plainText} />
      {/if}
    </Section>
  {/if}
</div>
