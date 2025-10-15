<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<!-- 

<svelte:head></svelte:head> is a special tag in Svelte that is used 
to add elements into the <head></head> of `src/app.html`

-->
<svelte:head>
	<link rel="icon" href={favicon} />
  <title>Svelte Syntax</title>
</svelte:head>

<!-- 

`?.` is the optional chaining operator in JS / TS

It simply means try to do something, and if the result is undefined,
or null, then don't throw an error.

-->
{@render children?.()}
