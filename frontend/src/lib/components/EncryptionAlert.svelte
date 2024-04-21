<script lang="ts">
  import * as Alert from "$lib/components/ui/alert";
  export let error: { message: string | undefined } | string | undefined;
  export let title: string = "Failed to Encrypt";
</script>

<Alert.Root variant="destructive" class="space-y-2 rounded">
  <Alert.Title>{title}</Alert.Title>
  <Alert.Description>
    {#if error}
      {#if typeof error == "object"}
        {error.message}
      {:else}
        {error}
      {/if}
    {:else}
      Unexpected error
    {/if}
  </Alert.Description>
</Alert.Root>
