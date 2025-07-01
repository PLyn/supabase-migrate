<!-- src/routes/+page.svelte -->
<script>
	let count = $state(0);

	function onClick() {
		count += 1;
	}
</script>

<div class="container mx-auto px-4 py-8">
	<h1>Welcome to Leptos!</h1>
	<button onclick={onClick}>Click Me: {count}</button>
	<div>Test area below</div>
	<h1>Supabase Connect Example (Rust/Axum)</h1>
	<h1 class="text-3xl font-bold text-red-600">Hello world!</h1>
	<button class="btn btn-primary">test button</button>
	<p>This is an example of implementing Supabase OAuth integration.</p>
	<p>
		<a href="/connect-supabase/login" rel="external">Login with Supabase</a>
	</p>
	<p>
		<a href="/connect-supabase/projects" rel="external">View Projects (if authenticated)</a>
	</p>
</div>
