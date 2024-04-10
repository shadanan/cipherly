<script lang="ts">
  import * as Cipherly from "$lib/cipherly";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";

  export let payload: Cipherly.PasswordPayload;
  export let plainText: Promise<string> | null = null;
  let password = "";

  async function decrypt(
    payload: Cipherly.PasswordPayload,
    password: string,
  ): Promise<string> {
    const { s: salt, iv: iv, ct: cipherText } = payload;
    const key = await Cipherly.deriveKey(Cipherly.encodeUtf8(password), salt);
    const plainText = await Cipherly.decrypt(cipherText, key, iv);
    return Cipherly.decodeUtf8(plainText);
  }
</script>

<div class="space-y-2">
  <Label
    class="text-background-foreground text-sm uppercase tracking-wider"
    for="password">Password</Label
  >
  <Input
    id="password"
    class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
    type="password"
    placeholder="The password to use for decryption"
    bind:value={password}
  />
</div>

<div class="pt-4">
  <Button
    class="min-w-[140px] text-lg font-bold"
    on:click={() => (plainText = decrypt(payload, password))}>Decrypt</Button
  >
</div>
