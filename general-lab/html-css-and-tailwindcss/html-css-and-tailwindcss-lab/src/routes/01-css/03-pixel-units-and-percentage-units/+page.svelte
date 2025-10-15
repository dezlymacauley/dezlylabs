<script>
	import "./page.css";
</script>

<!---------------------------------------------------------------------------->

<!-- 

The two dive tags below are inside the `<body></body>` HTML tag.

The tag is not shown here because in Svelte,
everything inside a Svelte file (excluding the script and style tags),
is is inside the <body></body> tag.

-->
<div class="entire-body">The full width of the `body` html tag</div>
<div class="half-of-body">50 Percent of `body` container</div>
<!---------------------------------------------------------------------------->

<div class="container-one">
	<!-- These will take 25% of its parent, which is container-one  -->
	<div class="item">Item 1</div>
	<div class="item">Item 2</div>
	<div class="item">Item 3</div>
</div>
