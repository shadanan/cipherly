import { Lock, KeyRound, UserRoundCheck, Unlock } from "lucide-svelte";

const config = {
  site: {
    name: "Cipherly",
    description:
      "Aute mollit ipsum enim aliquip consequat ad fugiat nostrud enim exercitation sunt commodo irure ullamco enim.",
    mainNav: [
      {
        title: "Password Encryption",
        href: "/password/encrypt/",
        icon: KeyRound,
      },
      {
        title: "Auth Encryption",
        href: "/auth/encrypt",
        icon: UserRoundCheck,
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
