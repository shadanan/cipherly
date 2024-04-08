<script lang="ts">
  import googleLogo from "$lib/assets/google.svg";
  import { logout, renderLoginButton, token, user } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import Avatar from "$lib/components/Avatar.svelte";
  import CopyText from "$lib/components/CopyText.svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { Textarea } from "$lib/components/ui/textarea";
  import { onMount } from "svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });

  let payload = "";
  let plainText: Promise<string> | null = null;

  if (location.hash) {
    payload = location.href;
  }

  async function decrypt(payload: string): Promise<string> {
    if ($user === null || $token === null) {
      throw new Error("Cannot decrypt without logging in.");
    }
    const {
      k: kid,
      n: nonce,
      se: data,
      iv: iv,
      ct: cipherText,
    } = Cipherly.decodeAuthPayload(payload);
    const envelope = await Cipherly.unseal({ kid, nonce, data }, $token);
    const plainText = await Cipherly.decrypt(cipherText, envelope.dek, iv);
    return Cipherly.decodeUtf8(plainText);
  }
</script>

<div class="space-y-8">
  <div
    class="border-background-foreground space-y-6 rounded-md border-2 bg-card p-8"
  >
    <div>
      <h1 class="text-xl font-bold text-foreground">
        Authorization Based Decryption
      </h1>
    </div>

    <div class={$user === null ? "" : "hidden"}>
      <div id="login-button" class="w-[200px]" style="color-scheme:light">
        <Skeleton class="h-10" />
      </div>
    </div>

    {#if $user !== null}
      <div class="flex items-center space-x-4">
        <div
          class="flex items-center space-x-4 rounded-3xl bg-muted px-4 py-2 text-muted-foreground"
        >
          <img src={googleLogo} alt="Google" class="h-4 w-4" />
          <p>
            Logged in as {$user?.name}
          </p>
          <Avatar user={$user} class="h-6 w-6" />
        </div>
        <Button on:click={() => logout()} variant="secondary">Logout</Button>
      </div>

      <form
        class="space-y-6"
        on:submit|preventDefault={() => (plainText = decrypt(payload))}
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
            bind:value={payload}
            placeholder="The ciphertext payload to be decrypted"
          />
        </div>

        <div class="pt-4">
          <Button class="min-w-[140px] text-lg font-bold" type="submit"
            >Decrypt</Button
          >
        </div>
      </form>
    {/if}
  </div>
  {#if plainText}
    <div
      class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
    >
      <div>
        <h1 class="text-xl font-bold text-foreground">Decrypted Content</h1>
      </div>
      {#await plainText}
        <div class="space-y-6 py-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
        <div class="space-y-2">
          <Label
            for="plainText"
            class="text-background-foreground text-sm uppercase tracking-wider"
          >
            Decrypted Plaintext
          </Label>
          <Textarea
            class="disabled:opacity-1 border-2 border-muted text-base focus:ring-0 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600"
            id="plainText"
            disabled
            value={plainText}
            placeholder="The decrypted plaintext"
          />
        </div>
        <CopyText label="Plaintext" text={plainText} />
      {:catch err}
        <Alert.Root variant="destructive" class="space-y-2 rounded">
          {#if err.code === 401}
            <Alert.Title>Unauthorized</Alert.Title>
            <Alert.Description>
              You are not authorized to decrypt this secret.
            </Alert.Description>
          {:else}
            <Alert.Title>Failed to Decrypt</Alert.Title>
            <Alert.Description>Ciphertext is invalid.</Alert.Description>
          {/if}
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
