<script lang="ts">

    import CountdownPage from "../../features/countdown/components/CountdownPage.svelte";

    type pages = ("countdowns" | "about");
    let activePage: pages = "countdowns";

</script>

<header class="container-fluid">
    <nav>
        <ul>
            <li><strong>Owlerlay</strong></li>
        </ul>
        <ul>
            <li><a href="#" on:click|preventDefault={() => activePage = "countdowns"}>Countdowns</a></li>
            <li><a href="#" on:click|preventDefault={() => activePage = "about"}>About</a></li>
        </ul>
    </nav>
</header>

<main class="container">
    {#if activePage === "countdowns"}
        <CountdownPage/>
    {:else if activePage === "about"}
        <p>Owlerlay — OBS overlay control center.</p>
    {/if}
</main>
