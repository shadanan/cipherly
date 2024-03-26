import { GoogleOAuthProvider } from "google-oauth-gsi";
import { jwtDecode } from "jwt-decode";
import { derived, writable } from "svelte/store";

type Claims = {
  name: string;
  email: string;
  picture: string;
};

const googleProvider = new GoogleOAuthProvider({
  clientId:
    "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
});

export const token = writable<string | null>(
  sessionStorage.getItem("credential") || null,
);

export function logout() {
  token.set(null);
  sessionStorage.removeItem("credential");
}

export function login() {
  googleProvider.useGoogleOneTapLogin({
    cancel_on_tap_outside: true,
    onSuccess: (res) => {
      if (!res.credential) {
        console.error("Credential is missing", res);
        return;
      }
      token.set(res.credential);
      sessionStorage.setItem("credential", res.credential);
    },
  })();
}

export const claims = derived(token, ($jwt) => {
  if ($jwt === null) {
    return null;
  }
  return jwtDecode($jwt) as Claims;
});
