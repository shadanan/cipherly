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

type PasswordPayload = {
  salt: Uint8Array;
  iv: Uint8Array;
  cipherText: Uint8Array;
};

export function encodePasswordPayload(payload: PasswordPayload): string {
  return encodeMessagePack(payload);
}

export function decodePasswordPayload(hash: string): PasswordPayload {
  return decodeMessagePack(hash) as PasswordPayload;
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

type AuthPayload = {
  sealedEnvelope: Uint8Array;
  cipherText: Uint8Array;
};

export function encodeAuthPayload(payload: AuthPayload): string {
  return encodeMessagePack(payload);
}

export function decodeAuthPayload(hash: string): AuthPayload {
  return decodeMessagePack(hash) as AuthPayload;
}

export function authUrl(): string {
  return `${location.protocol}//${location.host}/auth/#`;
}

type Envelope = {
  dek: CryptoKey;
  iv: Uint8Array;
  emails: string[];
};

export async function seal(envelope: Envelope): Promise<Uint8Array> {
  const encodedDek = await crypto.subtle.exportKey("raw", envelope.dek);
  const response = await fetch("/api/seal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      dek: encodeBase64(new Uint8Array(encodedDek)),
      iv: encodeBase64(envelope.iv),
      emails: envelope.emails,
    }),
  });
  if (!response.ok) {
    throw { code: response.status, message: response.statusText };
  }
  const result = await response.json();
  return decodeBase64(result.data);
}

export async function unseal(
  data: Uint8Array,
  token: string,
): Promise<Envelope> {
  const response = await fetch("/api/unseal", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + token,
    },
    body: JSON.stringify({
      data: encodeBase64(data),
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

  return { dek, iv: decodeBase64(result.iv), emails: result.emails };
}
