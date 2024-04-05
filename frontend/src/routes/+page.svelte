<script lang="ts">
  import youTubeLogo from "$lib/assets/youtube.svg";
  import config from "$lib/config";
</script>

<section class="space-y-10 py-4 text-lg md:py-10">
  <div class="space-y-4">
    <h1 class="pb-1 text-3xl font-bold">What is Cipherly?</h1>
    <p>Cipherly is a browser-based encryption tool for sharing secrets.</p>
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
        href="https://www.youtube.com/@FriendlyTL"
      >
        Friendly TL YouTube channel!
      </a>
    </p>
    <div class="flex flex-wrap">
      {#each config.site.youTubeVideos as video}
        {@const videoUrl = `https://youtu.be/${video.id}`}
        {@const thumbnail = `https://i3.ytimg.com/vi/${video.id}/maxresdefault.jpg`}
        <a
          class="mb-6 mr-6 w-full sm:w-auto sm:max-w-[16rem]"
          target="_blank"
          href={videoUrl}
          referrerpolicy="no-referrer"
        >
          <div
            class="border-border overflow-hidden rounded border shadow-sm hover:shadow"
          >
            <img class="object-cover" src={thumbnail} alt="" srcset="" />
            <div class="flex items-center space-x-4 px-4">
              <img src={youTubeLogo} alt="YouTube" class="w-6 h-6" />
              <p class="text-sm font-medium">
                {video.title}
              </p>
            </div>
          </div>
        </a>
      {/each}
    </div>
  </div>
</section>
