import { encode as pack, decode as unpack } from "@msgpack/msgpack";
import { Base64 } from "js-base64";

function decodeBase64(data: string): Uint8Array {
  return Base64.toUint8Array(data);
}

function encodeBase64(data: Uint8Array): string {
  return Base64.fromUint8Array(data, true);
}

function encodeMessagePack(obj: any): string {
  return encodeBase64(pack(obj));
}

function decodeMessagePack(data: string) {
  return unpack(decodeBase64(data));
}

export function encodeUtf8(data: string): Uint8Array {
  return new TextEncoder().encode(data);
}

export function decodeUtf8(data: Uint8Array): string {
  return new TextDecoder().decode(data);
}

export function generateSalt(): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(16));
}

export async function generateKey(): Promise<CryptoKey> {
  return crypto.subtle.generateKey(
    {
      name: "AES-GCM",
      length: 256,
    },
    true,
    ["encrypt", "decrypt"],
  );
}

export function generateIv(): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(12));
}

function decryptUrl(hash: string): string {
  return `${location.protocol}//${location.host}/decrypt/#${hash}`;
}

function extractHash(data: string): string {
  const hashPos = data.indexOf("#");
  if (hashPos !== -1) {
    return data.slice(hashPos + 1);
  }
  return data;
}

export async function deriveKey(
  password: Uint8Array,
  salt: Uint8Array,
): Promise<CryptoKey> {
  const keyMaterial = await crypto.subtle.importKey(
    "raw",
    password,
    { name: "PBKDF2" },
    false,
    ["deriveKey"],
  );

  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt,
      iterations: 100000,
      hash: "SHA-256",
    },
    keyMaterial,
    { name: "AES-GCM", length: 256 },
    true,
    ["encrypt", "decrypt"],
  );
}

export async function encrypt(
  data: Uint8Array,
  key: CryptoKey,
  iv: Uint8Array,
): Promise<Uint8Array> {
  return new Uint8Array(
    await crypto.subtle.encrypt({ name: "AES-GCM", iv }, key, data),
  );
}

export async function decrypt(
  data: Uint8Array,
  key: CryptoKey,
  iv: Uint8Array,
): Promise<Uint8Array> {
  return new Uint8Array(
    await crypto.subtle.decrypt({ name: "AES-GCM", iv }, key, data),
  );
}

export enum EncryptionScheme {
  Password = 0,
  Auth = 1,
}

type PayloadHeader = {
  es: EncryptionScheme;
};

type PasswordBody = {
  s: Uint8Array; // salt
  iv: Uint8Array; // initialization vector
  ct: Uint8Array; // ciphertext
};

export type PasswordPayload = PayloadHeader & PasswordBody;

export function isPasswordPayload(payload: any): payload is PasswordPayload {
  return (
    typeof payload === "object" &&
    "es" in payload &&
    payload.es === EncryptionScheme.Password &&
    "s" in payload &&
    "iv" in payload &&
    "ct" in payload
  );
}

export function encodePasswordPayload(payload: PasswordBody): string {
  const msgPackPayload: PasswordPayload = {
    es: EncryptionScheme.Password,
    ...payload,
  };
  return decryptUrl(encodeMessagePack(msgPackPayload));
}

export function decodePasswordPayload(data: string): PasswordPayload {
  return decodeMessagePack(extractHash(data)) as PasswordPayload;
}

export async function passwordEncrypt(
  plainText: string,
  password: string,
): Promise<string> {
  const salt = generateSalt();
  const key = await deriveKey(encodeUtf8(password), salt);

  const iv = generateIv();
  const cipherText = await encrypt(encodeUtf8(plainText), key, iv);

  return encodePasswordPayload({ s: salt, iv: iv, ct: cipherText });
}

export async function passwordDecrypt(
  payload: Payload,
  password: string,
): Promise<string> {
  const { s: salt, iv: iv, ct: cipherText } = payload as PasswordPayload;
  const key = await deriveKey(encodeUtf8(password), salt);
  const plainText = await decrypt(cipherText, key, iv);
  return decodeUtf8(plainText);
}

type AuthBody = {
  k: string; // kid
  n: Uint8Array; // nonce
  se: Uint8Array; // sealed envelope
  iv: Uint8Array; // initialization vector
  ct: Uint8Array; // ciphertext
};

export type AuthPayload = PayloadHeader & AuthBody;

export function isAuthPayload(payload: any): payload is AuthPayload {
  return (
    typeof payload === "object" &&
    "es" in payload &&
    payload.es === EncryptionScheme.Auth &&
    "k" in payload &&
    "n" in payload &&
    "se" in payload &&
    "iv" in payload &&
    "ct" in payload
  );
}

export function encodeAuthPayload(payload: AuthBody): string {
  const msgPackPayload: AuthPayload = {
    es: EncryptionScheme.Auth,
    ...payload,
  };
  return decryptUrl(encodeMessagePack(msgPackPayload));
}

export function decodeAuthPayload(data: string): AuthPayload {
  return decodeMessagePack(extractHash(data)) as AuthPayload;
}

export type Payload = PasswordPayload | AuthPayload;

export function decodePayload(data: string): Payload {
  return decodeMessagePack(extractHash(data)) as Payload;
}

type Envelope = {
  dek: CryptoKey;
  emails: string[];
};

type SealedEnvelope = {
  kid: string;
  nonce: Uint8Array;
  data: Uint8Array;
};

export async function seal(envelope: Envelope): Promise<SealedEnvelope> {
  const encodedDek = await crypto.subtle.exportKey("raw", envelope.dek);
  const response = await fetch("/api/seal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      dek: encodeBase64(new Uint8Array(encodedDek)),
      emails: envelope.emails,
    }),
  });
  if (!response.ok) {
    throw { code: response.status, message: response.statusText };
  }
  const result = await response.json();
  return {
    kid: result.kid,
    nonce: decodeBase64(result.nonce),
    data: decodeBase64(result.data),
  };
}

export async function unseal(
  envelope: SealedEnvelope,
  token: string,
): Promise<Envelope> {
  const response = await fetch("/api/unseal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + token,
    },
    body: JSON.stringify({
      kid: envelope.kid,
      nonce: encodeBase64(envelope.nonce),
      data: encodeBase64(envelope.data),
    }),
  });
  if (!response.ok) {
    throw response;
  }
  const result = await response.json();
  const dek = await crypto.subtle.importKey(
    "raw",
    decodeBase64(result.dek),
    { name: "AES-GCM" },
    true,
    ["encrypt", "decrypt"],
  );

  return { dek, emails: result.emails };
}

export async function authEncrypt(
  plainText: string,
  emails: string[],
): Promise<string> {
  const dek = await generateKey();
  const iv = generateIv();
  const cipherText = await encrypt(encodeUtf8(plainText), dek, iv);
  const { kid, nonce, data } = await seal({ dek, emails });
  return encodeAuthPayload({
    k: kid,
    n: nonce,
    se: data,
    iv: iv,
    ct: cipherText,
  });
}

export async function authDecrypt(
  payload: Payload,
  token: string,
): Promise<string> {
  const {
    k: kid,
    n: nonce,
    se: data,
    iv: iv,
    ct: cipherText,
  } = payload as AuthPayload;
  const envelope = await unseal({ kid, nonce, data }, token);
  const plainText = await decrypt(cipherText, envelope.dek, iv);
  return decodeUtf8(plainText);
}
