<script lang="ts">
  import { cn } from "$lib/utils";
  import type { HTMLAttributes } from "svelte/elements";

  type $$Props = HTMLAttributes<HTMLDivElement> & {
    icon: ConstructorOfATypedSvelteComponent;
  };
  let className: $$Props["class"] = undefined;

  export let icon: ConstructorOfATypedSvelteComponent;
  export let size: string = "14px";
  export { className as class };
</script>

<div class={cn("flex items-center", className)}>
  <svelte:component
    this={icon}
    class="inline-block h-[{size}] w-[{size}] mr-1"
  />
  <slot />
</div>
