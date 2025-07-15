<script lang="ts">
  import DemoWidget from './components/DemoWidget.svelte'
</script>

<section class="app">
  <header>
    <h1>784</h1>
  </header>

  <main>
    <DemoWidget />
  </main>

  <a
    href="/book"
    class="book-link"
  >
    READ: An Introduction to Neural Networks
  </a>

  <footer>
    <a
      class="external"
      href="https://github.com/dansalias/784"
      target="_blank"
    >
      Source on GitHub
    </a>
  </footer>
</section>

<style>
  .app {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;

    > main {
      margin: 0 0 1rem;
    }

    > .book-link {
      text-decoration: none;
    }

    > .book-link:hover {
      text-decoration: underline;
    }
  }
</style>
