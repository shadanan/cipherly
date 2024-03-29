<script lang="ts">
  import { claims, login, logout } from "$lib/auth";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    <Avatar.Root>
      <Avatar.Image src={$claims?.picture} />
      <Avatar.Fallback>?</Avatar.Fallback>
    </Avatar.Root>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      {#if $claims === null}
        <DropdownMenu.Item on:click={login}>Login</DropdownMenu.Item>
      {:else}
        <DropdownMenu.Label>{$claims?.name}</DropdownMenu.Label>
        <DropdownMenu.Item>{$claims?.email}</DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item on:click={logout}>Logout</DropdownMenu.Item>
      {/if}
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
