<script lang="ts">
  import { encodePayload, passwordEncrypt } from "$lib/cipherly";
  import Section from "$lib/components/Section.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import ValidationError from "$lib/components/ValidationError.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { z } from "zod";

  const PasswordEncryptFormSchema = z
    .object({
      data: z.instanceof(Uint8Array),
      filename: z.string().nullable(),
      password: z.string().default(""),
    })
    .refine(({ data, filename }) => data.length !== 0 || filename !== null, {
      message: "Either text or file input must be present",
      path: ["plainText"],
    });
  type PasswordEncryptFormData = z.input<typeof PasswordEncryptFormSchema>;

  let formData: PasswordEncryptFormData = {
    data: new Uint8Array(),
    filename: null,
    password: "",
  };

  let error: z.ZodError | null;
  let payload: Promise<Uint8Array> | null = null;

  $: {
    formData;
    payload = null;
    error = null;
  }
</script>

<div class="space-y-8">
  <Section title="Password Encrypt">
    <form
      class="space-y-6"
      on:submit|preventDefault={() => {
        const result = PasswordEncryptFormSchema.safeParse(formData);
        if (!result.success) {
          error = result.error;
          payload = null;
          return;
        }
        error = null;
        payload = passwordEncrypt(
          result.data.data,
          result.data.password,
          result.data.filename ? result.data.filename : undefined,
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

        <ValidationError {error} path="plainText" />
        <TextOrFileInput
          bind:data={formData.data}
          bind:filename={formData.filename}
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
      data={payload.then((data) => encodePayload(data, !!formData.filename))}
      name={formData.filename ? formData.filename + ".cly" : null}
    />
  {/if}
</div>
