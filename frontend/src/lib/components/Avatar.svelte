<script lang="ts">
  import avatar from "gradient-avatar";
  export let user: App.Locals["user"];
  export let size: number;

  const userAvatar = (email: string) => avatar(email, size);
</script>

<div class="overflow-clip rounded-full">
  {#if user.picture}
    <img class={`object-cover h-[${size}px] w-[${size}px]`} src={user.picture} alt={user.email} />
  {:else}
    {@html userAvatar(user.email)}
  {/if}
</div>
