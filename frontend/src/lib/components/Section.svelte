<script lang="ts">
  export let title: string;
</script>

<div
  class="border-background-foreground space-y-6 rounded-md border-2 bg-background p-8"
>
  <div>
    <h1 class="text-xl font-bold text-foreground">{title}</h1>
  </div>
  <slot />
</div>
