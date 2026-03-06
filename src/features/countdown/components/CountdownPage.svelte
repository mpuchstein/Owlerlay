<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {countdownStore} from "../state/countdown.store";
    import {type Duration, formatDuration} from "../../../shared/time/duration";

    let label = "";
    let hours = 0;
    let minutes = 0;
    let seconds = 0;

    function handleCreate() {
        const duration: Duration = {hours, minutes, seconds, millis: 0};
        countdownStore.create(label, duration);
        label = "";
        hours = 0;
        minutes = 0;
        seconds = 0;
    }

    let cleanup: (() => void) | null = null;

    onMount(async () => {
        void countdownStore.loadList();
        cleanup = await countdownStore.initStoreListeners();
    });

    onDestroy(() => cleanup?.());
</script>

{#if $countdownStore.loading}<p aria-busy="true">Loading...</p>{/if}
{#if $countdownStore.error}<p class="countdown-error">{$countdownStore.error}</p>{/if}

<article>
    <header>New Countdown</header>
    <input bind:value={label} placeholder="Label" required/>
    <fieldset role="group">
        <input aria-label="Hours" bind:value={hours} max="99" min="0" placeholder="hh" type="number"/>
        <input aria-label="Minutes" bind:value={minutes} max="59" min="0" placeholder="mm" type="number"/>
        <input aria-label="Seconds" bind:value={seconds} max="59" min="0" placeholder="ss" type="number"/>
        <button on:click={handleCreate}>Create</button>
    </fieldset>
</article>

{#if $countdownStore.items.length > 0}
    <article>
        <header>
            <select on:change={(e) => countdownStore.select(Number(e.currentTarget.value))}>
                {#each $countdownStore.items as item (item.id)}
                    <option value={item.id}>{item.label}</option>
                {/each}
            </select>
            {#if $countdownStore.selected}
                <mark data-state={$countdownStore.selected.state}>
                    {$countdownStore.selected.state}
                </mark>
            {/if}
        </header>

        {#if $countdownStore.selected}
            <p class="timer-display">
                {formatDuration($countdownStore.liveRemaining ?? $countdownStore.selected.duration)}
            </p>
            <footer class="countdown-actions">
                {#if $countdownStore.selected.state === "Idle"}
                    <button on:click={() => countdownStore.startSelected()}>Start</button>
                {:else if $countdownStore.selected.state === "Running"}
                    <button on:click={() => countdownStore.pauseSelected()}>Pause</button>
                {:else if $countdownStore.selected.state === "Paused"}
                    <button on:click={() => countdownStore.resumeSelected()}>Resume</button>
                {/if}
                <button class="secondary" on:click={() => countdownStore.resetSelected()}>Reset</button>
                <button class="secondary contrast" on:click={() => countdownStore.deleteSelected()}>Delete</button>
            </footer>
        {/if}
    </article>
{/if}
