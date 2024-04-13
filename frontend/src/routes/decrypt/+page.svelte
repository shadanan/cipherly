<script lang="ts">
  import { z } from "zod";
  import { renderLoginButton, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import Section from "$lib/components/Section.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { onMount } from "svelte";
  import Auth from "./auth.svelte";
  import Password from "./password.svelte";
  import { getError, hasError } from "$lib/form";
  import { AlertCircle } from "lucide-svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  const DecryptFormSchema = z.object({
    encodedPayload: z.string().min(1),
    payload: z
      .custom<Cipherly.Payload | null>()
      .refine((payload) => {
        // version validation
        return true; // valid
      }, "wrong version")
      .refine((payload) => {
        // validate another thing
        return true; // valid
      }, "something else"),
  });
  type DecryptFormData = z.infer<typeof DecryptFormSchema>;

  let validationError: z.ZodError | undefined;
  let formData: DecryptFormData = {
    encodedPayload: "",
    payload: null,
  };

  let loading = false;
  let plainText: string | null = null;
  let decryptionError: any = null;
  let password = "";

  if (location.hash) {
    formData.encodedPayload = location.href;
  }

  $: {
    try {
      formData.payload = formData.encodedPayload
        ? Cipherly.decodePayload(formData.encodedPayload)
        : null;
      decryptionError = null;
    } catch (err) {
      decryptionError = err;
    }
  }

  async function decrypt(formData: DecryptFormData): Promise<string | null> {
    formData.payload = formData.encodedPayload
      ? Cipherly.decodePayload(formData.encodedPayload)
      : null;

    if (formData.payload?.es === Cipherly.EncryptionScheme.Auth) {
      return await Cipherly.authDecrypt(formData.payload, $token as string);
    }
    if (formData.payload?.es === Cipherly.EncryptionScheme.Password) {
      return await Cipherly.passwordDecrypt(formData.payload, password);
    }
    return Promise.resolve(null);
  }

  function validateFormData(formData: DecryptFormData): boolean {
    const validationResult = DecryptFormSchema.safeParse(formData);
    validationError = validationResult.success
      ? undefined
      : validationResult.error;
    return validationResult.success;
  }
</script>

<div class="space-y-8">
  <Section
    title="{formData.payload
      ? Cipherly.EncryptionScheme[formData.payload.es]
      : ''} Decrypt"
  >
    <form
      class="space-y-6"
      on:submit|preventDefault={async () => {
        plainText = null;
        if (!validateFormData(formData)) return;
        loading = true;
        try {
          plainText = await decrypt(formData);
          decryptionError = null;
        } catch (e) {
          decryptionError = e;
        } finally {
          loading = false;
        }
      }}
    >
      {#if decryptionError && decryptionError.name !== "OperationError"}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>
            {#if decryptionError.status === 401}
              Unauthorized
            {:else}
              Invalid Payload
            {/if}
          </Alert.Description>
        </Alert.Root>
      {/if}

      <div class="space-y-2">
        <Label
          class="text-background-foreground text-sm uppercase tracking-wider"
          for="payload">Ciphertext Payload</Label
        >

        {#if hasError(validationError, "payload") || hasError(validationError, "encodedPayload")}
          {@const payloadErr = getError(validationError, "payload")}
          {@const encodedPayloadErr = getError(
            validationError,
            "encodedPayload",
          )}
          <p class="flex items-center space-x-1 text-xs text-destructive">
            <AlertCircle class="inline-block h-[12px] w-[12px]"></AlertCircle>
            {#if encodedPayloadErr}
              <span>{encodedPayloadErr}</span>
            {:else}
              <span>{payloadErr}</span>
            {/if}
          </p>
        {/if}

        <Textarea
          required
          minlength={1}
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          id="payload"
          bind:value={formData.encodedPayload}
          placeholder="The ciphertext payload to be decrypted"
        />
      </div>

      {#if formData.payload?.es === Cipherly.EncryptionScheme.Password}
        <Password
          bind:value={password}
          error={decryptionError && decryptionError.name === "OperationError"
            ? "Invalid Password"
            : null}
        />
      {:else if formData.payload?.es === Cipherly.EncryptionScheme.Auth}
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
      {#if loading}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
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
