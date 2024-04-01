<script lang="ts">
  import type { HTMLAttributes, HTMLTextareaAttributes } from "svelte/elements";
  import { cn } from "$lib/utils";

  type $$Props = HTMLAttributes<HTMLHeadingElement> & {
    user: App.Locals["user"];
  };

  let className: $$Props["class"] = undefined;
  export { className as class };
  export let user: $$Props["user"];
</script>

<div class={cn("overflow-clip rounded-full", className)}>
  <img class={`object-fit rounded-full`} src={user.picture} alt={user.email} referrerpolicy="no-referrer" />
</div>
