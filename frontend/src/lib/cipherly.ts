import { encode as pack, decode as unpack } from "@msgpack/msgpack";
import { Base64 } from "js-base64";

export function passwordUrl(): string {
  return `${location.protocol}//${location.host}/password/#`;
}

export function generateSalt(): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(16));
}

export function generateIv(): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(12));
}

function encode(obj: any): string {
  return Base64.fromUint8Array(pack(obj), true);
}

function decode(data: string) {
  return unpack(Base64.toUint8Array(data));
}

type PasswordEnvelope = {
  salt: Uint8Array;
  iv: Uint8Array;
  ciphertext: Uint8Array;
};

export function encodePasswordEnvelope(envelope: PasswordEnvelope): string {
  return encode(envelope);
}

export function decodePasswordEnvelope(hash: string): PasswordEnvelope {
  return decode(hash) as PasswordEnvelope;
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
