<script>
    import { page } from '$app/stores';
</script>

<div class="error-container">
    <h1>Oops! Something went wrong</h1>
    <p>{$page.error?.message || 'An unexpected error occurred'}</p>
    <a href="/">Go back home</a>
</div>

<style>
    .error-container {
        text-align: center;
        padding: 2rem;
        max-width: 600px;
        margin: 4rem auto;
    }
    
    h1 {
        color: #e53e3e;
        margin-bottom: 1rem;
    }
    
    a {
        display: inline-block;
        margin-top: 1rem;
        padding: 0.5rem 1rem;
        background: #4299e1;
        color: white;
        text-decoration: none;
        border-radius: 0.375rem;
    }
    
    a:hover {
        background: #3182ce;
    }
</style>