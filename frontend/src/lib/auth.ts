import { GoogleOAuthProvider, googleLogout } from "google-oauth-gsi";
import { jwtDecode } from "jwt-decode";
import { derived, writable } from "svelte/store";

export type User = {
  email: string;
  name: string;
  picture: string;
  exp: number;
};

const CREDENTIAL_KEY = "credential";

let googleProviderLoaded = false;

export const googleProvider = new GoogleOAuthProvider({
  clientId:
    "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
  onScriptLoadSuccess: () => {
    googleProviderLoaded = true;
  },
});

export const token = writable<string | null>(
  sessionStorage.getItem(CREDENTIAL_KEY) || null,
);

export function logout() {
  googleLogout();
  token.set(null);
  sessionStorage.removeItem(CREDENTIAL_KEY);
}

export const renderLoginButton = (element: HTMLElement | null) => {
  if (!element) {
    return;
  }
  if (!googleProviderLoaded) {
    setTimeout(() => renderLoginButton(element), 10);
    return;
  }
  googleProvider.useRenderButton({
    element,
    use_fedcm_for_prompt: true,
    onSuccess: (res) => {
      if (!res.credential) {
        console.error("Credential is missing", res);
        return;
      }
      token.set(res.credential);
      sessionStorage.setItem(CREDENTIAL_KEY, res.credential);
    },
    onError: () => {
      console.error("Error useGoogleOneTapLogin");
    },
    promptMomentNotification: (notification) => {
      console.log("promptMomentNotification", notification);
    },
  })();
};

export const user = derived(token, ($jwt) => {
  if ($jwt === null) {
    return null;
  }
  const user = jwtDecode($jwt) as User;
  if (user.exp * 1000 < Date.now()) {
    return null;
  }
  return user;
});
