<script lang="ts">
  import { currentUser, login, logout } from "$lib/auth";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger
    ><Avatar.Root>
      <Avatar.Image src={$currentUser?.picture} />
      <Avatar.Fallback>LY</Avatar.Fallback>
    </Avatar.Root></DropdownMenu.Trigger
  >
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      {#if $currentUser === null}
        <DropdownMenu.Item on:click={login}>Login</DropdownMenu.Item>
      {:else}
        <DropdownMenu.Label>{$currentUser?.name}</DropdownMenu.Label>
        <DropdownMenu.Item>{$currentUser?.email}</DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item on:click={logout}>Logout</DropdownMenu.Item>
      {/if}
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
