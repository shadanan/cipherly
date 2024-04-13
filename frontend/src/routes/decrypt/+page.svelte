<script lang="ts">
  import { renderLoginButton } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import CopyText from "$lib/components/CopyText.svelte";
  import Section from "$lib/components/Section.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { onMount } from "svelte";
  import Auth from "./auth.svelte";
  import Password from "./password.svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  let encodedPayload = "";
  let plainText: Promise<string> | null = null;
  let error: any = null;

  if (location.hash) {
    encodedPayload = location.href;
  }

  let payload: Cipherly.Payload | null;
  $: {
    error = null;
    payload = null;
    plainText = null;
    try {
      payload =
        encodedPayload === "" ? null : Cipherly.decodePayload(encodedPayload);
    } catch (err) {
      console.log(err);
      error = err;
    }
  }

  function passwordPayload(
    payload: Cipherly.Payload,
  ): Cipherly.PasswordPayload {
    return payload as Cipherly.PasswordPayload;
  }

  function authPayload(payload: Cipherly.Payload): Cipherly.AuthPayload {
    return payload as Cipherly.AuthPayload;
  }
</script>

<div class="space-y-8">
  <Section
    title="{payload ? Cipherly.EncryptionScheme[payload.es] : ''} Decrypt"
  >
    <div class="space-y-2">
      <Label
        class="text-background-foreground text-sm uppercase tracking-wider"
        for="payload">Ciphertext Payload</Label
      >
      <Textarea
        required
        class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
        id="payload"
        bind:value={encodedPayload}
        placeholder="The ciphertext payload to be decrypted"
      />
      {#if error}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Invalid Payload</Alert.Title>
        </Alert.Root>
      {/if}
    </div>

    {#if payload?.es === Cipherly.EncryptionScheme.Password}
      <Password bind:plainText payload={passwordPayload(payload)} />
    {:else if payload?.es === Cipherly.EncryptionScheme.Auth}
      <Auth bind:plainText payload={authPayload(payload)} />
    {/if}
  </Section>

  {#if plainText}
    <Section title="Decrypted Content">
      {#await plainText}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
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
      {:catch err}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          <Alert.Title>Failed to Decrypt</Alert.Title>
          <Alert.Description>
            {#if err.status === 401}
              Unauthorized
            {:else if err.name === "OperationError"}
              Invalid password
            {/if}
          </Alert.Description>
        </Alert.Root>
      {/await}
    </Section>
  {/if}
</div>
