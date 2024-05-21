<script lang="ts">
  import {
    EncryptionScheme,
    Payload,
    authDecrypt,
    decodePayload,
    isAuthPayload,
    isPasswordPayload,
    passwordDecrypt,
  } from "$lib/cipherly";
  import Label from "$lib/components/Label.svelte";
  import TextOrFileInput from "$lib/components/TextOrFileInput.svelte";
  import TextOrFileOutput from "$lib/components/TextOrFileOutput.svelte";
  import ValidationError from "$lib/components/ValidationError.svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { z } from "zod";
  import Auth from "./auth.svelte";

  const InputData = z
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
      return z.NEVER;
    });
  type InputData = z.input<typeof InputData>;

  let inputData: InputData = {
    data: new Uint8Array(),
    filename: null,
  };

  const DecryptData = z
    .object({
      payload: Payload.nullable(),
      password: z.string().default(""),
      token: z.string().nullable(),
    })
    .transform(({ payload, password, token }, ctx) => {
      if (isPasswordPayload(payload)) {
        if (!password) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "Password must be present",
            path: ["password"],
            fatal: true,
          });
          return z.NEVER;
        }
        return { payload, password };
      }
      if (isAuthPayload(payload)) {
        if (!token) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            message: "User must be authorized",
            path: ["token"],
            fatal: true,
          });
          return z.NEVER;
        }
        return { payload, token };
      }
      ctx.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Payload must be present",
        path: ["payload"],
        fatal: true,
      });
      return z.NEVER;
    });
  type DecryptData = z.input<typeof DecryptData>;
  type Output = z.infer<typeof DecryptData>;

  let decryptData: DecryptData = {
    payload: null,
    password: "",
    token: null,
  };

  let error: z.ZodError | null;
  let plainText: Promise<Uint8Array[]> | null = null;

  $: {
    inputData;
    plainText = null;
    InputData.safeParseAsync(inputData).then((p) => {
      if (p.success) {
        decryptData.payload = p.data;
        error = null;
      } else {
        decryptData.payload = null;
        error = p.error;
      }
    });
  }

  function decrypt() {
    plainText = null;
    if (error) return;
    const parsed = DecryptData.safeParse(decryptData);
    if (!parsed.success) {
      error = parsed.error;
      plainText = null;
      return;
    }
    error = null;
    const parsedData: Output = parsed.data;
    if (isAuthPayload(parsed.data.payload)) {
      plainText = Promise.all([
        authDecrypt(parsed.data.payload, parsed.data.token!),
      ]);
    } else if (isPasswordPayload(parsed.data.payload)) {
      plainText = Promise.all([
        passwordDecrypt(parsed.data.payload, parsed.data.password!),
      ]);
    }
  }
</script>

<div class="space-y-8 p-1">
  <form class="space-y-4" on:submit|preventDefault={decrypt}>
    <div>
      <Label for="payload">Ciphertext Payload</Label>
      <ValidationError {error} path="payload" />
      <TextOrFileInput
        text={location.hash ? location.href : ""}
        bind:data={inputData.data}
        bind:filename={inputData.filename}
        placeholder="ciphertext payload"
      />
    </div>

    {#if decryptData.payload?.es === EncryptionScheme.Password}
      <div>
        <Label for="password">Password</Label>
        <ValidationError {error} path="password" />
        <Input
          id="password"
          class="border-2 border-muted text-base text-foreground focus:ring-0 focus-visible:ring-0"
          type="password"
          placeholder="The password to use for decryption"
          bind:value={decryptData.password}
        />
      </div>
    {:else if decryptData.payload?.es === EncryptionScheme.Auth}
      <div>
        <ValidationError {error} path="token" />
        <Auth bind:token={decryptData.token} />
      </div>
    {/if}

    <Button class="min-w-[140px] text-lg font-bold" type="submit">
      Decrypt
    </Button>
  </form>

  {#if plainText}
    <TextOrFileOutput
      kind="Decrypt"
      data={plainText}
      name={decryptData.payload?.fn}
    />
  {/if}
</div>
