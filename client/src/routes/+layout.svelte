<!-- src/routes/+layout.svelte -->
<script>
	import '../app.css';

	function handleLogin() {
		window.location.href = '/connect-supabase/login';
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<header class="bg-white shadow-md">
		<div class="container mx-auto px-4">
			<div class="flex items-center justify-between py-4">
				<!-- Title on the left -->
				<h1 class="text-2xl font-bold text-gray-800">Supabase M&M</h1>

				<nav class="flex space-x-8">
					<a
						href="/"
						class="border-b-2 border-transparent pb-1 font-medium text-gray-600 transition duration-200 hover:border-gray-900 hover:text-gray-900"
					>
						Home
					</a>
					<a
						href="/migrate"
						class="border-b-2 border-transparent pb-1 font-medium text-gray-600 transition duration-200 hover:border-gray-900 hover:text-gray-900"
					>
						Migrate
					</a>
					<a
						href="/monitor"
						class="border-b-2 border-transparent pb-1 font-medium text-gray-600 transition duration-200 hover:border-gray-900 hover:text-gray-900"
					>
						Monitor
					</a>
					<a
						href="/metrics"
						class="border-b-2 border-transparent pb-1 font-medium text-gray-600 transition duration-200 hover:border-gray-900 hover:text-gray-900"
					>
						Metrics
					</a>
				</nav>

				<button class="btn btn-primary" onclick={handleLogin}> Sign in with Supabase </button>
			</div>
		</div>
	</header>

	<!-- Main content -->
	<main>
		<slot />
	</main>
</div>
