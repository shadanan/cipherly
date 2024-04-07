<script lang="ts">
  import YouTubeVideoCard from "$/lib/components/YouTubeVideoCard.svelte";
  import config from "$lib/config";
</script>

<section class="space-y-10 py-4 text-lg md:py-10">
  <div class="space-y-4">
    <h1 class="pb-1 text-3xl font-bold">What is Cipherly?</h1>
    <p>{config.site.description}</p>
    <p>
      It uses the browser's built-in <a
        target="_blank"
        href="https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto"
        class="text-primary underline underline-offset-4">SubtleCrypto</a
      > library to encrypt and decrypt the secret without ever sending the secret
      to the server.
    </p>
  </div>

  <div class="space-y-4">
    <h1 class="pb-1 text-3xl font-bold">How can I learn more?</h1>
    <p>
      You can follow along on the <a
        class="text-primary underline underline-offset-4"
        href={config.site.youTubeChannelUrl}
      >
        Friendly TL YouTube channel!
      </a>
    </p>
    <div class="flex flex-wrap">
      {#each config.site.youTubeVideos as video}
        <YouTubeVideoCard {video} />
      {/each}
    </div>
  </div>
</section>
