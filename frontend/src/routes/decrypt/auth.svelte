<script lang="ts">
  import googleLogo from "$lib/assets/google.svg";
  import { logout, renderLoginButton, token, user } from "$lib/auth";
  import Avatar from "$lib/components/Avatar.svelte";
  import { Button } from "$lib/components/ui/button";
  import Skeleton from "$lib/components/ui/skeleton/skeleton.svelte";
  import { onMount } from "svelte";

  onMount(() => {
    renderLoginButton(document.getElementById("login-button"));
  });
</script>

<div class={$user === null ? "" : "hidden"}>
  <div id="login-button" class="w-[200px]" style="color-scheme:light">
    <Skeleton class="h-10" />
  </div>
</div>

{#if $user !== null && $token !== null}
  <div class="flex items-center space-x-4">
    <div
      class="flex items-center space-x-4 rounded-3xl bg-muted px-4 py-2 text-muted-foreground"
    >
      <img src={googleLogo} alt="Google" class="h-4 w-4" />
      <p>
        Logged in as {$user?.name}
      </p>
      <Avatar user={$user} class="h-6 w-6" />
    </div>
    <Button on:click={() => logout()} variant="secondary">Logout</Button>
  </div>
{/if}
