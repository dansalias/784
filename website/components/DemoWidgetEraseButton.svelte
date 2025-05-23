<script lang="ts">
  const { onclick } = $props()
</script>

<button
  class="erase-button"
  onclick={onclick}
>
  <svg
    class="icon"
    xmlns="http://www.w3.org/2000/svg"
    width="14"
    height="14"
    viewBox="0 0 16 16"
  >
    <path d="M 14 14 H 6 L 2 10 L 10 2 L 14 6 L 6 14 M 9 11 L 5 7"></path>
  </svg>
  <span>
    ERASE
  </span>
</button>

<style>
  .erase-button {
    z-index: 10;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 1rem;

    background: var(--white);
    border-radius: 0 0 0 0.75rem;

    font-weight: 600;
    color: var(--blue);

    > .icon {
      fill: none;
      stroke: currentColor;
      stroke-width: 2px;
      stroke-linejoin: round;
      stroke-linecap: round;
    }
  }
</style>
