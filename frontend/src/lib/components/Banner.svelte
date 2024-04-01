<script lang="ts">
  import logo from "$lib/assets/cipherly-white.png";
</script>

<div class="container px-5 mx-auto relative">
  <div class="flex flex-col text-center w-full mb-8 items-center space-y-4">
    <a href="/">
      <div class="flex items-center space-x-2">
        <img alt="The project logo" src={logo} class="w-10" />
        <div class="text-4xl font-extrabold uppercase">Cipherly</div>
      </div>
    </a>
    <!-- <p class="md:w-1/2 mx-auto leading-relaxed text-md text-gray-500">
      A browser-based encryption tool using <a
        target="_blank"
        href="https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto"
        class="underline underline-offset-4">SubtleCrypto</a
      > for password-based encryption/decryption with operations exclusively in the browser for enhanced security.
    </p> -->
  </div>
</div>
