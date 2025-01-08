<template>
  <div
    class="
      grid min-h-dvh py-4
      bg-linear-135 from-cyan-300 to-emerald-100 bg-fixed
      text-center text-indigo-900
    ">
    <div class="m-auto">
      <h1 class="mb-8 font-display text-6xl">784</h1>
      <app-demo />
      <p class="mt-12 text-sm">
        <a
          class="underline decoration-dotted"
          href="https://github.com/dansalias/784"
          target="_blank"
        >
          Source on GitHub
        </a>
      </p>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import AppDemo from './AppDemo.vue'
</script>
