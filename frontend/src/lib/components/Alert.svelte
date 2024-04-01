<script lang="ts">
  import clsx from "clsx";
  import { AlertCircle } from "lucide-svelte";
  export let title: string = "";
  export let description: string;
  export let type: "danger" | "success" | "warning" | "info";
</script>

<div
  class={clsx("mb-4 rounded p-4 text-sm", {
    " bg-red-100/80 text-red-700": type == "danger",
    " bg-green-100/80 text-green-700": type == "success",
    " bg-blue-100/80 text-blue-700": type == "info",
    " bg-yellow-100/80 text-yellow-700": type == "warning",
  })}
  role="alert"
>
  {#if title && title != ""}
    <span class="font-medium">
      {title}
    </span>
  {/if}

  {description}
</div>
