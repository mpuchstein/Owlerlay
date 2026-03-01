<script lang="ts">
    import {onMount} from "svelte";
    import {countdownStore} from "../state/countdown.store";
    import type {Duration} from "../../../shared/time/duration";

    let label = "";
    let hours = 0;
    let minutes = 0;
    let seconds = 0;
    let millis = 0;
    let duration: Duration = {hours, minutes, seconds, millis};

    let createVisible = false;
    let handleCreate = () => {
        if (createVisible) {
            duration = {hours, minutes, seconds, millis};
            countdownStore.create(label, duration);
            createVisible = false;
        } else {
            createVisible = true;
        }
    }

    onMount(() => {
        void countdownStore.loadList();
    });


</script>

{#if $countdownStore.loading}<p>Loading...</p>{/if}
{#if $countdownStore.error}<p>{$countdownStore.error}</p>{/if}
{#if createVisible}
    <div>
        <label for="cd-create-label">Label</label>
        <input id="cd-create-label" bind:value={label} placeholder="Label"/>
        <label for="cd-create-hours">Hours</label>
        <input id="cd-create-hours" bind:value={hours} placeholder="Hours" type="number"/>
        <label for="cd-create-minutes">Minutes</label>
        <input id="cd-create-minutes" bind:value={minutes} placeholder="Minutes" type="number"/>
        <label for="cd-create-seconds">Seconds</label>
        <input id="cd-create-seconds" bind:value={seconds} placeholder="Seconds" type="number"/>
    </div>
{/if}
<button on:click={handleCreate}>Create</button>
<br/>
<label for="countdown-select">Select Countdown</label>
<select id="countdown-select"
        on:change={(e) => {countdownStore.select(Number(e.currentTarget.value))}}>
    {#each $countdownStore.items as item (item.id)}
        <option value={item.id}>{item.label} {item.state}</option>
    {/each}
</select>
<br/>
{#if $countdownStore.selected}
    <p>{$countdownStore.selected.duration}</p>
    <p>{$countdownStore.selected.state}</p>
    <button on:click={() => {countdownStore.deleteSelected()}}>Delete</button>
    {#if $countdownStore.selected.state === "Idle"}
        <button on:click={() => {countdownStore.startSelected()}}>Start</button>
    {:else if $countdownStore.selected.state === "Running"}
        <button on:click={() => {countdownStore.pauseSelected()}}>Pause</button>
    {:else if $countdownStore.selected.state === "Paused"}
        <button on:click={() => {countdownStore.resumeSelected()}}>Resume</button>
    {/if}
    <button on:click={() => {countdownStore.resetSelected()}}>Reset</button>
{/if}