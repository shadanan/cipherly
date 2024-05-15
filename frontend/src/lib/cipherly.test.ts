import { encode as encodeMessagePack } from "@msgpack/msgpack";
import { describe, expect, it, vi } from "vitest";
import * as Cipherly from "./cipherly";

describe("cipherly", () => {
  it("encodeUtf8 / decodeUtf8 succeeds", () => {
    const data = "hello";
    const encoded = Cipherly.encodeUtf8(data);
    expect(Cipherly.decodeUtf8(encoded)).toBe(data);
  });

  it("generateSalt generates a 16-byte salt", () => {
    const salt = Cipherly.generateSalt();
    expect(salt.length).toBe(16);
  });

  it("generateKey generates a 256-bit AES-GCM key", async () => {
    const key = await Cipherly.generateKey();
    expect(key.algorithm.name).toBe("AES-GCM");
    expect(key.usages).toEqual(["encrypt", "decrypt"]);
  });

  it("generateIv generates a 12-byte initialization vector", () => {
    const iv = Cipherly.generateIv();
    expect(iv.length).toBe(12);
  });

  it("encodePasswordPayload / decodePasswordPayload succeeds", () => {
    vi.stubGlobal("location", {
      protocol: "https:",
      host: "cipherly.app",
    });
    const actual = {
      s: Cipherly.generateSalt(),
      iv: Cipherly.generateIv(),
      ct: Cipherly.encodeUtf8("hello"),
    };
    const encoded = Cipherly.encodePasswordPayload(actual);
    const expected = { es: Cipherly.EncryptionScheme.Password, ...actual };
    expect(Cipherly.decodePasswordPayload(encoded)).toEqual(expected);
  });

  it("deriveKey derives a key from a password and salt", async () => {
    const password = Cipherly.encodeUtf8("password");
    const salt = Cipherly.generateSalt();
    const key = await Cipherly.deriveKey(password, salt);
    expect(key.algorithm.name).toBe("AES-GCM");
    expect(key.usages).toEqual(["encrypt", "decrypt"]);
  });

  it("encrypt / decrypt succeeds", async () => {
    const key = await Cipherly.generateKey();
    const iv = Cipherly.generateIv();
    const plaintext = Cipherly.encodeUtf8("hello");
    const ciphertext = await Cipherly.encrypt(plaintext, key, iv);
    const decrypted = await Cipherly.decrypt(ciphertext, key, iv);
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
    const encoded = Cipherly.encodeAuthPayload(actual);
    const expected = {
      es: Cipherly.EncryptionScheme.Auth,
      fn: null,
      ...actual,
    };
    expect(Cipherly.decodeAuthPayload(encoded)).toEqual(expected);
  });

  it("message pack", () => {
    console.log(
      encodeMessagePack({ a: undefined, b: 1 }, { ignoreUndefined: true }),
    );
  });

  // TODO: Unit tests for seal / unseal with a mock of the backend
});
