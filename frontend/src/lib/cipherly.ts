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

type PasswordEnvelope = {
  salt: Uint8Array;
  iv: Uint8Array;
  ciphertext: Uint8Array;
};

export function encodePasswordEnvelope(envelope: PasswordEnvelope): string {
  return encodeMessagePack(envelope);
}

export function decodePasswordEnvelope(hash: string): PasswordEnvelope {
  return decodeMessagePack(hash) as PasswordEnvelope;
}

export function passwordUrl(): string {
  return `${location.protocol}//${location.host}/password/#`;
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

type AuthEnvelope = {
  kekEncryptedDek: Uint8Array;
  cipherText: Uint8Array;
};

export function encodeAuthEnvelope(envelope: AuthEnvelope): string {
  return encodeMessagePack(envelope);
}

export function decodeAuthEnvelope(hash: string): AuthEnvelope {
  return decodeMessagePack(hash) as AuthEnvelope;
}

export function authUrl(): string {
  return `${location.protocol}//${location.host}/auth/#`;
}

export async function kekEncrypt(
  dek: CryptoKey,
  iv: Uint8Array,
  emails: string[],
): Promise<Uint8Array> {
  const encodedDek = await crypto.subtle.exportKey("raw", dek);
  const response = await fetch("/api/encrypt", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      dek: encodeBase64(new Uint8Array(encodedDek)),
      iv: encodeBase64(iv),
      authorized_users: emails,
    }),
  });
  if (!response.ok) {
    throw { code: response.status, message: response.statusText };
  }
  const result = await response.json();
  return decodeBase64(result.header);
}

type AuthHeader = {
  dek: CryptoKey;
  iv: Uint8Array;
};

export async function kekDecrypt(
  header: Uint8Array,
  token: string,
): Promise<AuthHeader> {
  const response = await fetch("/api/decrypt", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + token,
    },
    body: JSON.stringify({
      header: encodeBase64(header),
    }),
  });
  if (!response.ok) {
    throw { code: response.status, message: response.statusText };
  }
  const result = await response.json();
  const dek = await crypto.subtle.importKey(
    "raw",
    decodeBase64(result.dek),
    { name: "AES-GCM" },
    true,
    ["encrypt", "decrypt"],
  );

  return { dek, iv: decodeBase64(result.iv) };
}
