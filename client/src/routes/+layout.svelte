<script>
    import "../app.css";
    import AuthComponent from "$lib/components/auth_component.svelte";
</script>

<div class="app">
    <header class="header">
        <div class="header-container">
            <div class="header-content">
                <h1 class="site-title">Supabase M&M</h1>

                <nav class="nav" aria-label="Main navigation">
                    <a href="/" class="nav-link"> Home </a>
                    <a href="/migrate" class="nav-link"> Migrate </a>
                </nav>

                <AuthComponent />
            </div>
        </div>
    </header>

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
            "Segoe UI",
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
        gap: 1.5rem; /* Increased gap for better spacing */
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

    /* Main Content */
    .main {
        flex: 1;
        display: flex;
        flex-direction: column;
        padding: 2rem 1rem; /* Added padding to main content area */
        max-width: 95%;
        margin: 0 auto;
        width: 100%;
        box-sizing: border-box;
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

        .main {
            padding: 1.5rem 1rem; /* Adjust padding for mobile */
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

        .main {
            padding: 1rem 0.75rem; /* Further adjust padding for smaller screens */
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
        .main {
            padding: 2rem 1.5rem;
        }
    }

    /* Large Desktop */
    @media (min-width: 1200px) {
        .header-container {
            padding: 0 2rem;
        }
        .main {
            padding: 2rem 2rem;
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
    }

    /* Print Styles */
    @media print {
        .header {
            box-shadow: none;
            border-bottom: 2px solid #000;
        }

        .nav-link {
            color: #000 !important;
        }

        .site-title {
            color: #000 !important;
        }
    }
</style>
