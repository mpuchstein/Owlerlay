<script lang="ts">

    import CountdownPage from "../../features/countdown/components/CountdownPage.svelte";

    type pages = ("countdowns" | "about");
    let activePage: pages = "countdowns";

</script>
<header>
    <h1>Owlerlay</h1>
</header>
<nav>
    <button on:click={() => activePage = "countdowns"}>Countdowns</button>
    <button on:click={() => activePage = "about"}>About</button>
</nav>
<br/>
<main>
    {#if activePage === "countdowns"}
        <CountdownPage/>
    {:else if activePage === "about"}
        <slot name="about"/>
    {:else}
        <slot name="countdowns"/>
    {/if}
</main>
