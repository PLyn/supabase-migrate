<!-- src/routes/+page.svelte -->
<script>
	let count = $state(0);

	function onClick() {
		count += 1;
	}
</script>

<div class="container">
	<header class="hero">
		<h1 class="hero-title">Welcome to Leptos!</h1>
		<p class="hero-subtitle">A modern web application with Supabase integration</p>
	</header>

	<section class="demo-section">
		<h2 class="section-title">Interactive Demo</h2>
		<div class="counter-demo">
			<button onclick={onClick} class="btn btn-primary counter-btn">
				Click Me: <span class="counter-value">{count}</span>
			</button>
		</div>
	</section>

	<section class="test-section">
		<h2 class="section-title">Test Area</h2>
		<div class="test-content">
			<p class="test-description">This section demonstrates various UI components and styling.</p>
			<button class="btn btn-secondary">Test Button</button>
		</div>
	</section>

	<section class="supabase-section">
		<h2 class="section-title">Supabase Connect Example</h2>
		<div class="feature-description">
			<p>This is an example of implementing Supabase OAuth integration with Rust/Axum backend.</p>
		</div>

		<div class="action-cards">
			<div class="card">
				<h3 class="card-title">Authentication</h3>
				<p class="card-description">
					Connect with your Supabase account to access personalized features.
				</p>
				<a href="/connect-supabase/login" rel="external" class="btn btn-primary">
					Login with Supabase
				</a>
			</div>

			<div class="card">
				<h3 class="card-title">Project Management</h3>
				<p class="card-description">View and manage your projects once authenticated.</p>
				<a href="/connect-supabase/projects" rel="external" class="btn btn-outline">
					View Projects
				</a>
			</div>
		</div>
	</section>
</div>

<style>
	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem 1rem;
		font-family:
			system-ui,
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			sans-serif;
		line-height: 1.6;
		color: #2d3748;
	}

	/* Hero Section */
	.hero {
		text-align: center;
		margin-bottom: 4rem;
		padding: 3rem 0;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-radius: 1rem;
		color: white;
		position: relative;
		overflow: hidden;
	}

	.hero::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		z-index: 1;
	}

	.hero > * {
		position: relative;
		z-index: 2;
	}

	.hero-title {
		font-size: 3rem;
		font-weight: 800;
		margin: 0 0 1rem 0;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	}

	.hero-subtitle {
		font-size: 1.2rem;
		opacity: 0.9;
		margin: 0;
		font-weight: 300;
	}

	/* Section Styling */
	.demo-section,
	.test-section,
	.supabase-section {
		margin-bottom: 3rem;
		padding: 2rem 0;
	}

	.section-title {
		font-size: 2rem;
		font-weight: 700;
		margin: 0 0 1.5rem 0;
		color: #1a202c;
		position: relative;
		padding-bottom: 0.5rem;
	}

	.section-title::after {
		content: '';
		position: absolute;
		bottom: 0;
		left: 0;
		width: 3rem;
		height: 3px;
		background: linear-gradient(90deg, #3182ce, #63b3ed);
		border-radius: 2px;
	}

	/* Counter Demo */
	.counter-demo {
		display: flex;
		justify-content: center;
		margin: 2rem 0;
	}

	.counter-btn {
		font-size: 1.1rem;
		padding: 1rem 2rem;
		min-width: 200px;
	}

	.counter-value {
		font-weight: 700;
		color: #ffd700;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
	}

	/* Test Section */
	.test-content {
		padding: 2rem;
		background: #f8fafc;
		border-radius: 0.75rem;
		border: 1px solid #e2e8f0;
	}

	.test-description {
		margin: 0 0 1.5rem 0;
		color: #4a5568;
	}

	/* Feature Description */
	.feature-description {
		margin-bottom: 2rem;
	}

	.feature-description p {
		font-size: 1.1rem;
		color: #4a5568;
		margin: 0;
	}

	/* Action Cards */
	.action-cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 2rem;
		margin-top: 2rem;
	}

	.card {
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 0.75rem;
		padding: 2rem;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
		transition: all 0.3s ease;
		position: relative;
		overflow: hidden;
	}

	.card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 4px;
		background: linear-gradient(90deg, #3182ce, #63b3ed);
	}

	.card:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 600;
		margin: 0 0 1rem 0;
		color: #2d3748;
	}

	.card-description {
		color: #4a5568;
		margin: 0 0 1.5rem 0;
		line-height: 1.6;
	}

	/* Button Styles */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.75rem 1.5rem;
		border: 2px solid transparent;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		min-width: 140px;
		text-align: center;
	}

	.btn:focus {
		outline: none;
		box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.3);
	}

	.btn-primary {
		background-color: #3182ce;
		color: white;
		border-color: #3182ce;
	}

	.btn-primary:hover {
		background-color: #2c5282;
		border-color: #2c5282;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(49, 130, 206, 0.3);
	}

	.btn-secondary {
		background-color: #4a5568;
		color: white;
		border-color: #4a5568;
	}

	.btn-secondary:hover {
		background-color: #2d3748;
		border-color: #2d3748;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(74, 85, 104, 0.3);
	}

	.btn-outline {
		background-color: transparent;
		color: #3182ce;
		border-color: #3182ce;
	}

	.btn-outline:hover {
		background-color: #3182ce;
		color: white;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(49, 130, 206, 0.3);
	}

	.btn:active {
		transform: translateY(0);
	}

	/* Responsive Design */
	@media (max-width: 768px) {
		.container {
			padding: 1rem 0.75rem;
		}

		.hero {
			margin-bottom: 2rem;
			padding: 2rem 1rem;
		}

		.hero-title {
			font-size: 2rem;
		}

		.hero-subtitle {
			font-size: 1rem;
		}

		.section-title {
			font-size: 1.5rem;
		}

		.action-cards {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		.card {
			padding: 1.5rem;
		}

		.counter-btn {
			font-size: 1rem;
			padding: 0.75rem 1.5rem;
			min-width: 160px;
		}
	}

	@media (max-width: 480px) {
		.hero-title {
			font-size: 1.75rem;
		}

		.btn {
			width: 100%;
			min-width: auto;
		}

		.counter-demo {
			margin: 1.5rem 0;
		}
	}

	/* High Contrast Mode */
	@media (prefers-contrast: high) {
		.btn {
			border-width: 3px;
		}

		.card {
			border-width: 2px;
		}

		.section-title::after {
			height: 4px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.btn,
		.card {
			transition: none;
		}

		.btn:hover,
		.card:hover {
			transform: none;
		}

		.hero::before {
			backdrop-filter: none;
		}
	}

	/* Print Styles */
	@media print {
		.hero {
			background: white !important;
			color: #000 !important;
			border: 2px solid #000;
		}

		.btn {
			border: 2px solid #000 !important;
			background: white !important;
			color: #000 !important;
		}

		.card {
			border: 1px solid #000;
			box-shadow: none;
		}
	}
</style>
