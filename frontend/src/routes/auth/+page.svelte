<script lang="ts">
  import { currentUser, login, logout, token } from "$lib/auth";
  import * as Cipherly from "$lib/cipherly";
  import * as Alert from "$lib/components/ui/alert";
  import Google from "$lib/components/icons/Google.svelte";
  import Avatar from "$lib/components/Avatar.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Skeleton } from "$/lib/components/ui/skeleton";

  let payload = "";
  let plainText: Promise<string> | null = null;
  let copiedPlaintext: boolean;

  if (location.hash) {
    payload = location.hash.slice(1);
  }

  async function decrypt(payload: string): Promise<string> {
    if ($token === null) {
      logout();
      throw new Error("Cannot decrypt without logging in.");
    }
    const { sealedEnvelope, cipherText } = Cipherly.decodeAuthPayload(payload);
    const envelope = await Cipherly.unseal(sealedEnvelope, $token);
    const plainText = await Cipherly.decrypt(cipherText, envelope.dek, envelope.iv);
    return Cipherly.decodeUtf8(plainText);
  }

  function copyPlaintext(url: string | null): void {
    if (!url) return;
    navigator.clipboard.writeText(url);
    copiedPlaintext = true;
    setTimeout(() => {
      copiedPlaintext = false;
    }, 500);
  }
  $: console.log({ token: $token, currentUser: $currentUser });
</script>

<div class="space-y-8">
  <div class="p-8 border-2 border-background-foreground rounded-md space-y-6 bg-card">
    <div>
      <h1 class="text-xl font-bold text-foreground">Authorization based decryption</h1>
    </div>

    {#if $token === null}
      <div class="pt-4">
        <Button class="space-x-2 flex min-w-[140px]" variant="secondary" type="button" on:click={login}>
          <Google class="w-4" />
          <span>Login to decrypt</span>
        </Button>
      </div>
    {:else}
      {#if $currentUser}
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-4 bg-muted px-4 py-2 rounded-3xl text-muted-foreground">
            <Google class="h-4 w-4" />
            <p>
              Logged in as {$currentUser?.name}
            </p>
            <Avatar user={$currentUser} class="h-6 w-6" />
          </div>
          <Button on:click={() => logout()} variant="secondary">Logout</Button>
        </div>
      {/if}

      <form class="space-y-6" on:submit|preventDefault={() => (plainText = decrypt(payload))}>
        <div class="space-y-2">
          <Label class="uppercase text-background-foreground tracking-wider text-sm" for="plainText"
            >Ciphertext Envelope</Label
          >
          <Textarea
            required
            class="text-base border-2 border-muted text-foreground focus:ring-0 focus-visible:ring-0"
            id="payload"
            bind:value={payload}
            placeholder="The ciphertext payload to be decrypted"
          />
        </div>

        <div class="pt-4">
          <Button class="min-w-[140px] text-lg font-bold" type="submit">Decrypt</Button>
        </div>
      </form>
    {/if}
  </div>
  {#if plainText}
    <div class="p-8 border-2 border-background-foreground rounded-md space-y-6 bg-background">
      <div>
        <h1 class="text-xl font-bold text-foreground">Encrypted content</h1>
      </div>
      {#await plainText}
        <div class="py-6 space-y-6">
          <Skeleton class="h-20 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:then plainText}
        <div class="space-y-2">
          <Label for="plainText" class="uppercase text-background-foreground tracking-wider text-sm">
            Decrypted Plaintext
          </Label>
          <Textarea
            class="text-base border-2 border-muted focus:ring-0 focus-visible:ring-0 disabled:cursor-text disabled:text-green-600 disabled:opacity-1"
            id="plainText"
            disabled
            value={plainText}
            placeholder="The decrypted plainText"
          />
        </div>

        <Button variant="secondary" class="min-w-[140px]" type="button" on:click={() => copyPlaintext(plainText)}>
          {#if copiedPlaintext}
            Copied!
          {:else}
            Copy Plaintext
          {/if}
        </Button>
      {:catch err}
        <Alert.Root variant="destructive" class="rounded space-y-2">
          {#if err.code === 401}
            <Alert.Title>Unauthorized</Alert.Title>
            <Alert.Description>You are not authorized to decrypt this secret.</Alert.Description>
          {:else}
            <Alert.Title>Failed to Decrypt</Alert.Title>
            <Alert.Description>Password is incorrect or ciphertext is invalid.</Alert.Description>
          {/if}
        </Alert.Root>
      {/await}
    </div>
  {/if}
</div>
