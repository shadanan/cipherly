import { KeyRound, Lock, Unlock, User } from "lucide-svelte";

const config = {
  site: {
    name: "Cipherly",
    primaryNavItems: [
      {
        title: "Password Encryption",
        href: "/password/encrypt/",
        icon: KeyRound,
      },
      {
        title: "Auth Encryption",
        href: "/auth/encrypt/",
        icon: User,
      },
    ],
    secondaryNavItems: [
      {
        title: "Encrypt",
        href: "/password/encrypt/",
        parent: "/password/",
        icon: Lock,
      },
      {
        title: "Decrypt",
        href: "/password/",
        parent: "/password/",
        icon: Unlock,
      },
      {
        title: "Encrypt",
        href: "/auth/encrypt/",
        parent: "/auth/",
        icon: Lock,
      },
      {
        title: "Decrypt",
        href: "/auth/",
        parent: "/auth/",
        icon: Unlock,
      },
    ],
  },
  routes: {
    LOGIN: "/login",
  },
};

export default config;
