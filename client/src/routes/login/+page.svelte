<!-- src/routes/login/+page.svelte -->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { supabase } from '$lib/supabaseClient';

	let isLoading = false;
	let error = '';

	// Handle OAuth sign in
	async function signInWithProvider(provider: 'google' | 'github') {
		try {
			isLoading = true;
			error = '';

			const { data, error: signInError } = await supabase.auth.signInWithOAuth({
				provider,
				options: {
					redirectTo: `${window.location.origin}/auth/callback`
				}
			});

			if (signInError) {
				throw signInError;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An unexpected error occurred';
			console.error('Sign in error:', err);
		} finally {
			isLoading = false;
		}
	}

	// Check if user is already authenticated
	onMount(async () => {
		const {
			data: { session }
		} = await supabase.auth.getSession();
		if (session) {
			goto('/dashboard'); // Redirect to dashboard or home page
		}
	});
</script>

<div class="container">
	<main class="login-card">
		<div class="header">
			<h1>Welcome Back</h1>
			<p>Sign in to your account to continue</p>
		</div>

		{#if error}
			<div class="error-message" role="alert" aria-live="polite">
				<svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z"
						clip-rule="evenodd"
					/>
				</svg>
				{error}
			</div>
		{/if}

		<div class="auth-buttons">
			<button
				type="button"
				class="auth-button google"
				onclick={() => signInWithProvider('google')}
				disabled={isLoading}
				aria-describedby="google-signin-desc"
			>
				<svg class="provider-icon" width="20" height="20" viewBox="0 0 24 24" aria-hidden="true">
					<path
						fill="#4285F4"
						d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
					/>
					<path
						fill="#34A853"
						d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
					/>
					<path
						fill="#FBBC05"
						d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
					/>
					<path
						fill="#EA4335"
						d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
					/>
				</svg>
				<span>Continue with Google</span>
				{#if isLoading}
					<div class="spinner" aria-hidden="true"></div>
				{/if}
			</button>
			<span id="google-signin-desc" class="sr-only">Sign in using your Google account</span>

			<button
				type="button"
				class="auth-button github"
				onclick={() => signInWithProvider('github')}
				disabled={isLoading}
				aria-describedby="github-signin-desc"
			>
				<svg
					class="provider-icon"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="currentColor"
					aria-hidden="true"
				>
					<path
						d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
					/>
				</svg>
				<span>Continue with GitHub</span>
				{#if isLoading}
					<div class="spinner" aria-hidden="true"></div>
				{/if}
			</button>
			<span id="github-signin-desc" class="sr-only">Sign in using your GitHub account</span>
		</div>

		<div class="footer">
			<p>
				By signing in, you agree to our
				<a href="/terms" class="link">Terms of Service</a> and
				<a href="/privacy" class="link">Privacy Policy</a>
			</p>
		</div>
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

	.login-card {
		background: white;
		border-radius: 16px;
		padding: 2.5rem;
		width: 100%;
		max-width: 400px;
		box-shadow:
			0 20px 25px -5px rgba(0, 0, 0, 0.1),
			0 10px 10px -5px rgba(0, 0, 0, 0.04);
	}

	.header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.header h1 {
		margin: 0 0 0.5rem 0;
		font-size: 1.875rem;
		font-weight: 700;
		color: #1f2937;
	}

	.header p {
		margin: 0;
		color: #6b7280;
		font-size: 1rem;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background-color: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.875rem;
		margin-bottom: 1.5rem;
	}

	.auth-buttons {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	.auth-button {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.875rem 1rem;
		border: 2px solid transparent;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		text-decoration: none;
		transition: all 0.2s ease-in-out;
		cursor: pointer;
		min-height: 48px; /* Accessibility: minimum touch target */
	}

	.auth-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.auth-button.google {
		background-color: white;
		border-color: #e5e7eb;
		color: #374151;
	}

	.auth-button.google:hover:not(:disabled) {
		background-color: #f9fafb;
		border-color: #d1d5db;
	}

	.auth-button.google:focus-visible {
		outline: 2px solid #4285f4;
		outline-offset: 2px;
	}

	.auth-button.github {
		background-color: #24292e;
		color: white;
	}

	.auth-button.github:hover:not(:disabled) {
		background-color: #1a1e22;
	}

	.auth-button.github:focus-visible {
		outline: 2px solid #f97316;
		outline-offset: 2px;
	}

	.provider-icon {
		flex-shrink: 0;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top: 2px solid currentColor;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		position: absolute;
		right: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.footer {
		text-align: center;
		font-size: 0.875rem;
		color: #6b7280;
		line-height: 1.5;
	}

	.footer p {
		margin: 0;
	}

	.link {
		color: #667eea;
		text-decoration: none;
	}

	.link:hover {
		text-decoration: underline;
	}

	.link:focus-visible {
		outline: 2px solid #667eea;
		outline-offset: 2px;
		border-radius: 2px;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	@media (max-width: 640px) {
		.login-card {
			padding: 1.5rem;
		}

		.header h1 {
			font-size: 1.5rem;
		}
	}

	/* High contrast mode support */
	@media (prefers-contrast: high) {
		.auth-button.google {
			border-color: #000;
		}
	}

	/* Reduced motion support */
	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}

		.auth-button {
			transition: none;
		}
	}
</style>
