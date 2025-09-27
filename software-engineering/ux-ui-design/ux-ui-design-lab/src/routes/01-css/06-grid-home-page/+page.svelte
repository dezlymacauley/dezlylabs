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
		<ul class="nav-list">
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
