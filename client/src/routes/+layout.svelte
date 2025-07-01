<!-- src/routes/+layout.svelte -->
<script>
	import '../app.css';

	function handleLogin() {
		window.location.href = '/connect-supabase/login';
	}
</script>

<div class="app">
	<!-- Header -->
	<header class="header">
		<div class="header-container">
			<div class="header-content">
				<!-- Title on the left -->
				<h1 class="site-title">Supabase M&M</h1>

				<nav class="nav" aria-label="Main navigation">
					<a href="/" class="nav-link" aria-current="page"> Home </a>
					<a href="/migrate" class="nav-link"> Migrate </a>
					<a href="/monitor" class="nav-link"> Monitor </a>
					<a href="/metrics" class="nav-link"> Metrics </a>
				</nav>

				<button class="login-btn" onclick={handleLogin} aria-label="Sign in with Supabase">
					Sign in with Supabase
				</button>
			</div>
		</div>
	</header>

	<!-- Main content -->
	<main class="main">
		<slot />
	</main>
</div>

<style>
	:global(*, *::before, *::after) {
		box-sizing: border-box;
	}

	:global(body) {
		margin: 0;
		font-family:
			system-ui,
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			sans-serif;
		line-height: 1.6;
		color: #2d3748;
		background-color: #f7fafc;
	}

	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		background-color: #f7fafc;
	}

	/* Header Styles */
	.header {
		background-color: #ffffff;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		position: sticky;
		top: 0;
		z-index: 100;
		border-bottom: 1px solid #e2e8f0;
	}

	.header-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 1rem;
	}

	.header-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 0;
		gap: 1rem;
	}

	.site-title {
		font-size: 1.875rem;
		font-weight: 700;
		color: #1a202c;
		margin: 0;
		letter-spacing: -0.025em;
	}

	/* Navigation Styles */
	.nav {
		display: flex;
		gap: 2rem;
		align-items: center;
	}

	.nav-link {
		position: relative;
		padding: 0.5rem 0;
		font-weight: 500;
		color: #4a5568;
		text-decoration: none;
		border-bottom: 2px solid transparent;
		transition: all 0.2s ease-in-out;
	}

	.nav-link:hover {
		color: #1a202c;
		border-bottom-color: #1a202c;
	}

	.nav-link:focus {
		outline: none;
		color: #1a202c;
		border-bottom-color: #3182ce;
		box-shadow: 0 0 0 3px rgba(49, 130, 206, 0.3);
		border-radius: 2px;
	}

	.nav-link[aria-current='page'] {
		color: #1a202c;
		border-bottom-color: #3182ce;
	}

	/* Login Button Styles */
	.login-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.75rem 1.5rem;
		background-color: #3182ce;
		color: white;
		border: 2px solid #3182ce;
		border-radius: 0.5rem;
		font-size: 0.95rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		white-space: nowrap;
	}

	.login-btn:hover {
		background-color: #2c5282;
		border-color: #2c5282;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(49, 130, 206, 0.3);
	}

	.login-btn:active {
		transform: translateY(0);
		box-shadow: 0 2px 4px rgba(49, 130, 206, 0.3);
	}

	.login-btn:focus {
		outline: none;
		box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.3);
	}

	/* Main Content */
	.main {
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	/* Mobile Responsive */
	@media (max-width: 768px) {
		.header-content {
			flex-direction: column;
			align-items: stretch;
			gap: 1rem;
			padding: 1rem 0;
		}

		.site-title {
			text-align: center;
			font-size: 1.5rem;
		}

		.nav {
			justify-content: center;
			gap: 1.5rem;
			flex-wrap: wrap;
		}

		.nav-link {
			font-size: 0.95rem;
		}

		.login-btn {
			align-self: center;
			width: auto;
			min-width: 200px;
		}
	}

	@media (max-width: 480px) {
		.header-container {
			padding: 0 0.75rem;
		}

		.nav {
			gap: 1rem;
		}

		.nav-link {
			font-size: 0.9rem;
			padding: 0.25rem 0;
		}

		.site-title {
			font-size: 1.25rem;
		}

		.login-btn {
			font-size: 0.9rem;
			padding: 0.65rem 1.25rem;
			min-width: 180px;
		}
	}

	/* Tablet and Small Desktop */
	@media (min-width: 769px) and (max-width: 1024px) {
		.header-container {
			padding: 0 1.5rem;
		}

		.nav {
			gap: 1.5rem;
		}
	}

	/* Large Desktop */
	@media (min-width: 1200px) {
		.header-container {
			padding: 0 2rem;
		}
	}

	/* High Contrast Mode */
	@media (prefers-contrast: high) {
		.header {
			border-bottom-width: 2px;
		}

		.nav-link {
			border-bottom-width: 3px;
		}

		.login-btn {
			border-width: 3px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.nav-link,
		.login-btn {
			transition: none;
		}

		.login-btn:hover {
			transform: none;
		}

		.login-btn:active {
			transform: none;
		}
	}

	/* Print Styles */
	@media print {
		.header {
			box-shadow: none;
			border-bottom: 2px solid #000;
		}

		.login-btn {
			display: none;
		}

		.nav-link {
			color: #000 !important;
		}

		.site-title {
			color: #000 !important;
		}
	}

	/* Focus Management for Keyboard Navigation */
	@media (prefers-reduced-motion: no-preference) {
		.nav-link:focus,
		.login-btn:focus {
			transition: box-shadow 0.15s ease-in-out;
		}
	}
</style>
