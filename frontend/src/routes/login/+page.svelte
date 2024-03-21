<script lang="ts">
  import { onMount } from "svelte";
  let googleReady = false;
  let mounted = false;

  onMount(() => {
    mounted = true;
    if (!googleReady) {
      return;
    }
    if (sessionStorage.getItem("credential") !== null) {
      return;
    }
    displaySignInButton();
  });

  function googleLoaded() {
    googleReady = true;
    if (!mounted) {
      return;
    }
    if (sessionStorage.getItem("credential") !== null) {
      return;
    }
    displaySignInButton();
  }

  function displaySignInButton() {
    google.accounts.id.initialize({
      client_id:
        "981002175662-g8jr2n89bptsn8n9ds1fn5edfheojr7i.apps.googleusercontent.com",
      callback: handleCredentialResponse,
    });
    google.accounts.id.renderButton(
      document.getElementById("buttonDiv"),
      { theme: "outline", size: "large" } // customization attributes
    );
    google.accounts.id.prompt(); // also display the One Tap dialog
  }

  function handleCredentialResponse(response) {
    sessionStorage["credential"] = response["credential"];
  }
</script>

<svelte:head>
  <script
    src="https://accounts.google.com/gsi/client"
    async
    defer
    on:load={googleLoaded}
  ></script>
</svelte:head>
