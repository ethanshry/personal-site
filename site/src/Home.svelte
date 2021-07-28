<script>
  import { onMount } from "svelte";
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
    "build a better user experience",
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
</script>

<home>
  <h1>Welcome!</h1>
  <div class="container">
    <div>I am working hard to&nbsp;</div>
    <div id="rolling-text" />
  </div>
  <p>Check back soon!</p>
</home>

<style>
  h1 {
    color: var(--theme);
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  div,
  p {
    font-family: var(--standard-text);
  }

  .container {
    display: flex;
    justify-content: center;
  }

  #rolling-text {
    display: inline-block;
    position: relative;
    white-space: pre;
    animation: rolling 3s forwards infinite;
  }

  @keyframes rolling {
    0% {
      top: -2em;
      opacity: 0;
    }
    20% {
      top: 0em;
      opacity: 1;
    }
    80% {
      top: 0em;
      opacity: 1;
    }
    100% {
      top: 2em;
      opacity: 0;
    }
  }
</style>
