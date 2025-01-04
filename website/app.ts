import { SefDemo } from './components/sef-demo'
import { SefDemoCanvas } from './components/sef-demo-canvas'
import { SefDemoCanvasGrid } from './components/sef-demo-canvas-grid'

window.customElements.define('sef-demo', SefDemo)
window.customElements.define('sef-demo-canvas', SefDemoCanvas)
window.customElements.define('sef-demo-canvas-grid', SefDemoCanvasGrid)

if (new URL(window.location.href).searchParams.has('debug')) {
  document.querySelectorAll('.debug').forEach(el => el.classList.remove('debug'))
}
