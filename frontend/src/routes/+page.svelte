<script lang="ts">
  import Youtube from "$/lib/components/icons/Youtube.svelte";
  import config from "$lib/config";
</script>

<section class="space-y-10 py-4 text-lg md:py-10">
  <div class="space-y-4">
    <h1 class="pb-1 text-3xl font-bold">What is Cipherly?</h1>
    <p>
      Cipherly is a browser-based encryption tool for password and auth based
      encryption/decryption.
    </p>
    <p>
      It uses the browser's built-in <a
        target="_blank"
        href="https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto"
        class="text-primary underline underline-offset-4">SubtleCrypto</a
      > library to provide encryption and decryption of text using a password. The
      password is never sent to the server, and the encryption and decryption is
      done entirely in the browser.
    </p>
    <p>This tool is a work in progress.</p>
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
      {#each config.site.videos as video}
        <a
          class="mb-6 mr-6 w-full sm:w-auto sm:max-w-[16rem]"
          target="_blank"
          href={video.href}
          referrerpolicy="no-referrer"
        >
          <div
            class="overflow-hidden rounded border border-border shadow-sm hover:shadow"
          >
            <img class="object-cover" src={video.thumbnail} alt="" srcset="" />
            <div class="flex items-center space-x-4 px-4">
              <Youtube class="h-16 w-16" />
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
