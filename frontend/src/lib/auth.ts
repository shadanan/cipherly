import { GoogleOAuthProvider } from "google-oauth-gsi";
import { jwtDecode } from "jwt-decode";
import { derived, writable } from "svelte/store";

const LOCAL_STORAGE_KEY = "CIPHERLY.CREDENTIAL";

const googleProvider = new GoogleOAuthProvider({
  clientId:
    "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
});

export const token = writable<string | null>(
  sessionStorage.getItem(LOCAL_STORAGE_KEY) || null,
);

export function logout() {
  token.set(null);
  sessionStorage.removeItem(LOCAL_STORAGE_KEY);
}

export function login() {
  console.log("login -> googleProvider.useGoogleOneTapLogin");

  googleProvider.useGoogleOneTapLogin({
    cancel_on_tap_outside: true,
    onSuccess: (res) => {
      if (!res.credential) {
        console.error("Credential is missing", res);
        return;
      }
      token.set(res.credential);
      sessionStorage.setItem(LOCAL_STORAGE_KEY, res.credential);
    },
    onError() {
      console.error("Error useGoogleOneTapLogin");
    },
  })();
}

export const currentUser = derived(token, ($jwt) => {
  if ($jwt === null) {
    return undefined;
  }
  try {
    return jwtDecode($jwt) as App.Locals["user"];
  } catch (error) {
    console.error("Failed to decode JWT", error);
    return null;
  }
});
