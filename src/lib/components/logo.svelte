<script lang="ts">
  import { cn } from "@/lib/utils";
  import type { HTMLImgAttributes } from "svelte/elements";

  let { class: className, ...rest }: HTMLImgAttributes = $props();
</script>

<img
  alt="logo"
  src="/icon-512.png"
  class={cn("size-13 rounded-xl", className)}
  {...rest}
/>
