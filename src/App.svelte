<script>
  import { onMount } from "svelte";
  import Blog from "./Blog.svelte";
  import Home from "./Home.svelte";
  import Projects from "./Projects.svelte";
  let padded = (elements) => {
    let max = elements.reduce(
      (max, curr) => (curr.length > max ? curr.length : max),
      0
    );
    return elements.map((e) => {
      while (e.length < max) {
        e = `${e} `;
      }
      return e;
    });
  };

  let options = padded([
    "bring you value",
    "impart knowledge",
    "share my story",
  ]);
  let currOption = 0;
  let textChanger = () =>
    setInterval(() => {
      document.getElementById("rolling-text").innerHTML = options[currOption];
      currOption = ++currOption % options.length;
    }, 3000);
  onMount(() => {
    document.getElementById("rolling-text").innerHTML = options[currOption];
    textChanger();
  });

  export let activeTab = 0;

  // curried function for updating activeTab
  export let setTab = (tabNo) => () => {
    console.log("ok");
    activeTab = tabNo;
  };
</script>

<main>
  <navbar>
    <header>ethanshry.com</header>
    <navitems>
      <navitem
        class="navitem {activeTab == 0 ? 'active' : ''}"
        on:click={setTab(0)}
      >
        Home
      </navitem>
      <navitem
        class="navitem {activeTab == 1 ? 'active' : ''}"
        on:click={setTab(1)}
      >
        Projects
      </navitem>
      <navitem
        class="navitem {activeTab == 2 ? 'active' : ''}"
        on:click={setTab(2)}
      >
        Blog
      </navitem>
    </navitems>
  </navbar>
  <content>
    {#if activeTab === 2}
      <Blog />
    {:else if activeTab === 1}
      <Projects />
    {:else}
      <Home />
    {/if}
  </content>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  navbar {
    display: flex;
    justify-content: space-between;
  }

  header {
    color: var(--theme);
    font-size: 1.2em;
  }

  navitems {
    display: none;
    justify-content: space-evenly;
    min-width: 50%;
    cursor: pointer;
  }

  navitem:not(.active):hover {
    transition: border-bottom 1.5s;
    border-bottom: 2px solid var(--theme);
  }

  navitem.active {
    transition: border-bottom 2s;
    border-bottom: 2px solid var(--theme);
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
    navitems {
      display: flex;
    }
  }
</style>
