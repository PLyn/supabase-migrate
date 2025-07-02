<!-- src/routes/auth/+page.svelte -->
<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    $: status = $page.url.searchParams.get("status"); // 'success' or 'error'
    $: reason = $page.url.searchParams.get("reason");

    onMount(() => {
        if (status === "success") {
            // Handle success
            setTimeout(() => goto("/"), 2000);
        }
    });
</script>
