<!-- src/routes/+page.svelte -->
<script>
    import { onMount } from "svelte";

    let is_authenticated = $state(false);
    let is_loading = $state(true); // Track loading state
    let count = $state(0);

    async function onClick() {
        count += 1;
    }

    onMount(async () => {
        try {
            const response = await fetch("http://localhost:10000/auth", {
                method: "GET",
                credentials: "include",
                headers: {
                    "Content-Type": "application/json",
                },
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            let response_text = await response.json();
            console.log(response_text);
            is_authenticated = response_text;
            console.log("is_auth: " + is_authenticated);
        } catch (err) {
            console.error("Error fetching preview:", err);
        } finally {
            is_loading = false; // Set loading to false regardless of success/failure
            console.log("finally");
        }
    });
</script>

<div class="container">
    <header class="hero">
        <h1 class="hero-title">Welcome to Svelte!</h1>
        <p class="hero-subtitle">
            A modern web application with Supabase integration
        </p>
    </header>

    {#if !is_authenticated && !is_loading}
        <div class="card">
            <h3 class="card-title">Authentication</h3>
            <p class="card-description">
                Connect with your Supabase account to access personalized
                features.
            </p>
            <a
                href="http://localhost:10000/connect-supabase/login"
                rel="external"
                class="btn btn-primary"
            >
                Authorize Supabase
            </a>
        </div>
    {/if}

    <!-- Loading skeleton or authentication status -->
    {#if is_loading}
        <div class="auth-skeleton">
            <div class="skeleton-header"></div>
            <div class="skeleton-text"></div>
        </div>
    {:else if !is_authenticated}
        <h1>I am not logged in</h1>
    {:else}
        <h1>Welcome user</h1>
        <p>I am logged in!</p>
    {/if}

    <section class="demo-section">
        <h2 class="section-title">Interactive Demo</h2>
        <div class="counter-demo">
            <button onclick={onClick} class="btn btn-primary counter-btn">
                Click Me: <span class="counter-value">{count}</span>
            </button>
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
            "Segoe UI",
            Roboto,
            sans-serif;
        line-height: 1.6;
        color: #2d3748;
    }

    /* Loading Skeleton Styles */
    .auth-skeleton {
        margin: 2rem 0;
        padding: 1.5rem;
        border-radius: 0.5rem;
        background: #f7fafc;
        border: 1px solid #e2e8f0;
    }

    .skeleton-header {
        height: 2rem;
        background: linear-gradient(
            90deg,
            #e2e8f0 25%,
            #f7fafc 50%,
            #e2e8f0 75%
        );
        background-size: 200% 100%;
        animation: skeleton-loading 1.5s infinite;
        border-radius: 0.25rem;
        margin-bottom: 0.75rem;
        width: 60%;
    }

    .skeleton-text {
        height: 1rem;
        background: linear-gradient(
            90deg,
            #e2e8f0 25%,
            #f7fafc 50%,
            #e2e8f0 75%
        );
        background-size: 200% 100%;
        animation: skeleton-loading 1.5s infinite;
        border-radius: 0.25rem;
        width: 40%;
    }

    @keyframes skeleton-loading {
        0% {
            background-position: 200% 0;
        }
        100% {
            background-position: -200% 0;
        }
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
        content: "";
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
    .demo-section {
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
        content: "";
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
        content: "";
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

        .card {
            padding: 1.5rem;
        }

        .counter-btn {
            font-size: 1rem;
            padding: 0.75rem 1.5rem;
            min-width: 160px;
        }

        .auth-skeleton {
            padding: 1rem;
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

    /* Reduced Motion - Disable skeleton animation */
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

        .skeleton-header,
        .skeleton-text {
            animation: none;
            background: #e2e8f0;
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

        .auth-skeleton {
            display: none;
        }
    }
</style>
