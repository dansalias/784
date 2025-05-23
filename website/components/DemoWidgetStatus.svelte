<script lang="ts">
  import { formatKB } from '../util/format'

  const { modelBytesReceived, modelBytesTotal } = $props()
</script>

<aside class="status">
  <div>Loading model...</div>
  <div>
    {formatKB(modelBytesReceived, false)} /
    {formatKB(modelBytesTotal)} KB
  </div>
</aside>

<style>
 .status {
    position: absolute;
    inset: auto 1rem 0;
    padding: 0.5rem 0.5rem 0;
    display: flex;
    justify-content: space-between;

    border-top: none;
    border-radius: 0 0 0.5rem 0.5rem;

    line-height: 1.5rem;
    font-size: 0.875rem;
  }
</style>
