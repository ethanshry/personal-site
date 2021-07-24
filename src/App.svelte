<script>
  import { onMount } from "svelte";
  let padded = (elements) => {
    let max = elements.reduce(
      (max, curr) => (curr.length > max ? curr.length : max),
      0
    );
    console.log(max);
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
  console.log(options);
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

<main>
  <h1>Hello!</h1>
  <div class="container">
    <div>I am working hard to&nbsp;</div>
    <div id="rolling-text" />
  </div>
  <p>Check back soon!</p>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Anonymous+Pro:ital,wght@0,400;0,700;1,400&display=swap");

  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  div,
  p {
    font-family: "Anonymous Pro", monospace;
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
  /*
  @keyframes rolling {
    100% {
      transform: rotate(360deg);
      opacity: 0;
    }
  }*/

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
