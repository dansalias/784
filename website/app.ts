import { createApp, reactive } from 'vue'
import App from './components/App.vue'

const state = reactive({
  isMobile: 'ontouchstart' in window,
  showDebug: new URL(window.location.href).searchParams.has('debug'),
})

export const useState = () => state

createApp(App).mount('#app')
