import { describe, expect, it, vi } from "vitest";
import {
  EncryptionScheme,
  decodeUtf8,
  encodeUtf8,
  exportedForTesting,
} from "./cipherly";
const {
  decodeAuthPayload,
  decodePasswordPayload,
  decrypt,
  deriveKey,
  encodeAuthPayload,
  encodePasswordPayload,
  encrypt,
  generateSalt,
  generateKey,
  generateIv,
} = exportedForTesting;

describe("cipherly", () => {
  it("encodeUtf8 / decodeUtf8 succeeds", () => {
    const data = "hello";
    const encoded = encodeUtf8(data);
    expect(decodeUtf8(encoded)).toBe(data);
  });

  it("generateSalt generates a 16-byte salt", () => {
    const salt = generateSalt();
    expect(salt.length).toBe(16);
  });

  it("generateKey generates a 256-bit AES-GCM key", async () => {
    const key = await generateKey();
    expect(key.algorithm.name).toBe("AES-GCM");
    expect(key.usages).toEqual(["encrypt", "decrypt"]);
  });

  it("generateIv generates a 12-byte initialization vector", () => {
    const iv = generateIv();
    expect(iv.length).toBe(12);
  });

  it("encodePasswordPayload / decodePasswordPayload succeeds", () => {
    vi.stubGlobal("location", {
      protocol: "https:",
      host: "cipherly.app",
    });
    const actual = {
      s: generateSalt(),
      iv: generateIv(),
      ct: encodeUtf8("hello"),
    };
    const encoded = encodePasswordPayload(actual);
    const expected = { es: EncryptionScheme.Password, ...actual };
    expect(decodePasswordPayload(encoded)).toEqual(expected);
  });

  it("deriveKey derives a key from a password and salt", async () => {
    const password = encodeUtf8("password");
    const salt = generateSalt();
    const key = await deriveKey(password, salt);
    expect(key.algorithm.name).toBe("AES-GCM");
    expect(key.usages).toEqual(["encrypt", "decrypt"]);
  });

  it("encrypt / decrypt succeeds", async () => {
    const key = await generateKey();
    const iv = generateIv();
    const plaintext = encodeUtf8("hello");
    const ciphertext = await encrypt(plaintext, key, iv);
    const decrypted = await decrypt(ciphertext, key, iv);
    expect(decrypted).toEqual(plaintext);
  });

  it("encodeAuthPayload / decodeAuthPayload succeeds", () => {
    vi.stubGlobal("location", {
      protocol: "https:",
      host: "cipherly.app",
    });
    const actual = {
      k: "key",
      n: new Uint8Array(16),
      se: new Uint8Array(16),
      iv: new Uint8Array(12),
      ct: new Uint8Array(16),
    };
    const encoded = encodeAuthPayload(actual);
    const expected = {
      es: EncryptionScheme.Auth,
      fn: null,
      ...actual,
    };
    expect(decodeAuthPayload(encoded)).toEqual(expected);
  });

  // TODO: Unit tests for seal / unseal with a mock of the backend
});
