<!-- src/routes/auth/callback/+page.svelte -->
<script lang="ts">
	import { createClient } from '@supabase/supabase-js';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	// Initialize Supabase client - use same credentials as login page
	const supabaseUrl = 'YOUR_SUPABASE_URL';
	const supabaseAnonKey = 'YOUR_SUPABASE_ANON_KEY';
	const supabase = createClient(supabaseUrl, supabaseAnonKey);

	let isProcessing = true;
	let error = '';
	let status = 'Processing authentication...';

	onMount(async () => {
		try {
			// Extract the code and state from URL parameters
			const urlParams = new URLSearchParams(window.location.search);
			const code = urlParams.get('code');
			const error_param = urlParams.get('error');
			const error_description = urlParams.get('error_description');

			// Handle OAuth errors (user denied access, etc.)
			if (error_param) {
				throw new Error(error_description || 'Authentication was cancelled or failed');
			}

			// If no code, something went wrong
			if (!code) {
				throw new Error('No authorization code received');
			}

			status = 'Exchanging authorization code...';

			// Exchange the code for a session
			const { data, error: exchangeError } = await supabase.auth.exchangeCodeForSession(code);

			if (exchangeError) {
				throw exchangeError;
			}

			if (!data.session) {
				throw new Error('No session created');
			}

			status = 'Authentication successful! Redirecting...';

			// Get the user's profile to ensure everything is set up
			const { data: profile, error: profileError } = await supabase.auth.getUser();

			if (profileError) {
				console.warn('Could not fetch user profile:', profileError);
				// Don't throw here, session is still valid
			}

			// Optional: Store user info or perform additional setup here
			// For example, create a user profile in your database if it doesn't exist

			// Redirect to intended destination or dashboard
			const redirectTo = sessionStorage.getItem('redirectAfterAuth') || '/dashboard';
			sessionStorage.removeItem('redirectAfterAuth'); // Clean up

			// Small delay to show success message
			setTimeout(() => {
				goto(redirectTo, { replaceState: true });
			}, 1000);
		} catch (err) {
			isProcessing = false;
			error =
				err instanceof Error ? err.message : 'An unexpected error occurred during authentication';
			console.error('Auth callback error:', err);

			// Redirect to login page after showing error
			setTimeout(() => {
				goto('/login?error=auth_failed', { replaceState: true });
			}, 3000);
		}
	});

	// Handle the case where user manually navigates away
	function handleReturnToLogin() {
		goto('/login', { replaceState: true });
	}
</script>

<svelte:head>
	<title>Authenticating... - Your App</title>
	<meta name="description" content="Processing your authentication" />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="container">
	<main class="callback-card">
		{#if isProcessing}
			<div class="processing-state">
				<div class="spinner-large" aria-hidden="true"></div>
				<h1>Authenticating</h1>
				<p aria-live="polite">{status}</p>
			</div>
		{:else if error}
			<div class="error-state">
				<div class="error-icon" aria-hidden="true">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor">
						<path
							d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
						/>
					</svg>
				</div>
				<h1>Authentication Failed</h1>
				<p class="error-message" role="alert">{error}</p>
				<button type="button" class="return-button" onclick={handleReturnToLogin}>
					Return to Sign In
				</button>
			</div>
		{/if}
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
	}

	.container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		padding: 1rem;
	}

	.callback-card {
		background: white;
		border-radius: 16px;
		padding: 3rem 2.5rem;
		width: 100%;
		max-width: 400px;
		box-shadow:
			0 20px 25px -5px rgba(0, 0, 0, 0.1),
			0 10px 10px -5px rgba(0, 0, 0, 0.04);
		text-align: center;
	}

	.processing-state h1,
	.error-state h1 {
		margin: 1rem 0 0.5rem 0;
		font-size: 1.5rem;
		font-weight: 600;
		color: #1f2937;
	}

	.processing-state p {
		margin: 0;
		color: #6b7280;
		font-size: 1rem;
	}

	.spinner-large {
		width: 48px;
		height: 48px;
		border: 4px solid #e5e7eb;
		border-top: 4px solid #667eea;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem auto;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state {
		color: #dc2626;
	}

	.error-icon {
		width: 48px;
		height: 48px;
		margin: 0 auto 1rem auto;
		background-color: #fef2f2;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #dc2626;
	}

	.error-message {
		margin: 1rem 0 2rem 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.return-button {
		background-color: #667eea;
		color: white;
		border: none;
		border-radius: 8px;
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease-in-out;
		min-height: 48px;
	}

	.return-button:hover {
		background-color: #5a67d8;
	}

	.return-button:focus-visible {
		outline: 2px solid #667eea;
		outline-offset: 2px;
	}

	@media (max-width: 640px) {
		.callback-card {
			padding: 2rem 1.5rem;
		}

		.processing-state h1,
		.error-state h1 {
			font-size: 1.25rem;
		}
	}

	/* Reduced motion support */
	@media (prefers-reduced-motion: reduce) {
		.spinner-large {
			animation: none;
		}

		.return-button {
			transition: none;
		}
	}
</style>
