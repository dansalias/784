<template>
  <div class="relative">
    <div class="
      relative z-20 inline-flex flex-col overflow-hidden
      bg-white border border-gray-400 shadow-lg rounded-md
      text-left
    ">
      <p class="pl-4 leading-10">Draw a digit from 0-9:</p>
      <app-demo-canvas class="mx-4 mb-3" ref="canvas" v-model="input" />
      <div
        class="flex justify-between bg-linear-180 from-white from-50% to-gray-200 to-50%"
      >
        <p class="grow pl-4 leading-8 rounded-tr-lg bg-gray-200 shadow-inner relative">
          Prediction:
          <span class="font-bold" v-show="prediction !== null">{{ prediction }}</span>
          <span class="absolute -right-2 bottom-0 w-4 h-4 bg-linear-180 from-gray-200/0 to-gray-200 pointer-events-none"></span>
        </p>
        <button
          class="px-4 rounded-bl-lg bg-white text-blue-400 text-sm font-semibold cursor-pointer z-10"
          @click="$refs.canvas.erase()"
        >
          <div
            class="flex items-center gap-1.5"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="12"
              height="12"
              viewBox="0 0 16 16"
              class="fill-none stroke-[1.8] stroke-current"
              style="stroke-linejoin: round; stroke-linecap: round;"
            >
              <path d="M 14 14 H 6 L 2 10 L 10 2 L 14 6 L 6 14 M 9 11 L 5 7"></path>
            </svg>
            <span>
              ERASE
            </span>
          </div>
        </button>
      </div>
    </div>
    <div
      class="
        absolute inset-x-4 bottom-0 px-2 pt-2 z-10
        flex justify-between
        rounded-b-md border border-gray-400 border-t-0
        shadow-lg bg-gray-100/50
        text-sm leading-6 text-gray-700
        transition-transform
      "
      :class="{ 'translate-y-6': modelBytesTotal && !model.length }"
    >
      <div>
        Loading model...
      </div>
      <div>
        {{ formatKB(modelBytesReceived, false) }} /
        {{ formatKB(modelBytesTotal) }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed, onMounted, ref } from 'vue'
  import { fprop } from '../lib/network'
  import { useModel } from '../helpers/network'
  import AppDemoCanvas from './AppDemoCanvas.vue'

  const formatKB = (
    bytes: number,
    showUnit: boolean = true,
  ) => `${ Math.floor(bytes / 1024) }${ showUnit ? 'KB' : '' }`

  const { loadModel, modelBytesReceived, modelBytesTotal, model } = useModel()

  const input = ref([])

  const prediction = computed(() => {
    if (!model.value.length || !input.value.length) {
      return null
    }

    const output = fprop(model.value, input.value)

    return output
      .reduce((maxIndex, currentValue, currentIndex) =>
        currentValue > output[maxIndex] ? currentIndex: maxIndex, 0
      ).toString()
  })

  onMounted(() => {
    loadModel()
  })
</script>
