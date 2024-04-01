import { Lock, KeyRound, Unlock, User } from "lucide-svelte";

const config = {
  site: {
    name: "Cipherly",
    mainNav: [
      {
        title: "Password Encryption",
        href: "/password/encrypt/",
        icon: KeyRound,
      },
      {
        title: "Auth Encryption",
        href: "/auth/encrypt",
        icon: User,
      },
    ],
    sidebarNav: [
      {
        matches: "/password",
        title: "Encrypt",
        href: "/password/encrypt/",
        icon: Lock,
      },
      {
        matches: "/password",
        title: "Decrypt",
        href: "/password/",
        icon: Unlock,
      },
      {
        matches: "/auth",
        title: "Encrypt",
        href: "/auth/encrypt",
        icon: Lock,
      },
      {
        matches: "/auth",
        title: "Decrypt",
        href: "/auth/",
        icon: Unlock,
      },
    ],
  },
  routes: {
    LOGIN: "/login",
  },
};

export default config;
