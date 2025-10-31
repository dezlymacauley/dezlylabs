1. Start with a sketch of the layout so that you know what
   you are working towards. Use one color to outline the rows,
   and another colour to outline the columns.

2. Create a div with the class `grid-container` and put all of the HTML
   elements inside, then group the HTML elements on the page into groups,
   based on the number of rows the layout has.

```
<div class="grid-container">

  <!-- Group 1: Title -->
  <header style="border: 1px solid red;">Title</header>

  <!-- Group 2: Navigation bar -->
  <nav style="border: 1px solid red;">
    <ul>
      <li>Home</li>
      <li>About</li>
      <li>Contact</li>
    </ul>
  </nav>

  <!-- Group 3: The Article and Side Bar -->
  <main style="border: 1px solid red;">
    <article>
      <h2>Header</h2>
      <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Vero, et reprehenderit blanditiis nulla ratione eveniet, voluptatum labore hic iste quisquam sint delectus neque deserunt quibusdam voluptatem quia enim. Vitae ratione dolorem aut nisi, rem quidem quis enim, perferendis quae hic doloribus dolor cupiditate repellat magni omnis laboriosam, nulla maxime! Iusto.</p>
    </article>
  </main>
  <aside>Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat molestias voluptatibus odit repellat consequatur temporibus sint at. Unde totam ex deleniti eligendi nulla hic omnis?</aside>

  <!-- Group 4: The Footer -->
  <footer style="border: 1px solid red;">Closing Notes</footer>

</div>
```

3. Add `style="border: 1px solid red;"`



_______________________________________________________________________________
### A good starting point

page.css
```css
.grid-container {
	display: grid;

	grid-template-areas:
		"site-name site-name"
		"navbar    sidebar"
		"latest-story sidebar"
		"footer    footer";

  /* 4 rows */
	grid-template-rows: auto auto auto auto;
  
  /* 2 columns */
  grid-template-columns: auto auto;
  
}

.site-name {
	grid-area: site-name;
}

.navbar {
	grid-area: navbar;
}

.sidebar {
	grid-area: sidebar;
}

.latest-story {
	grid-area: latest-story;
}

.footer {
	grid-area: footer;
}
```

+page.svelte
```svelte
<script>
	import "./page.css";
</script>

<svelte:head>
	<title>Grid Page Layout</title>
	<meta name="description" content="A basic home page that uses Grid" />
</svelte:head>

<div class="grid-container">

	<header class="site-name" style="border: 2px solid purple;">Space Punks</header>

	<nav class="navbar" style="border: 2px solid purple;">
		<ul>
			<li>Home</li>
			<li>About</li>
			<li>Contact</li>
		</ul>
	</nav>

	<main class="latest-story" style="border: 2px solid purple;">
		<article>
			<h2>Breaking News</h2>
			<p>
				Lorem ipsum dolor sit amet consectetur adipisicing elit. Vero, et reprehenderit blanditiis
				nulla ratione eveniet, voluptatum labore hic iste quisquam sint delectus neque deserunt
				quibusdam voluptatem quia enim. Vitae ratione dolorem aut nisi, rem quidem quis enim,
				perferendis quae hic doloribus dolor cupiditate repellat magni omnis laboriosam, nulla
				maxime! Iusto.
			</p>
		</article>
	</main>

	<aside class="sidebar">
		More Stories: Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat molestias
		voluptatibus odit repellat consequatur temporibus sint at. Unde totam ex deleniti eligendi nulla
		hic omnis?
	</aside>

	<footer class="footer" style="border: 2px solid purple;">Social Media Accounts</footer>
</div>
```
_______________________________________________________________________________
### Then modify the `auto` setting depending on what you want.

_______________________________________________________________________________
