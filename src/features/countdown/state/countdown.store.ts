import type {CountdownSnapshot} from "../model/countdown.types";
import {get, writable} from "svelte/store";
import {
    createCountdown,
    deleteCountdown,
    listCountdowns,
    pauseCountdown,
    resetCountdown,
    resumeCountdown,
    startCountdown
} from "../api/countdown.client";
import type {Duration} from "../../../shared/time/duration";


type CountdownStateStore = {
    items: CountdownSnapshot[];
    selected: CountdownSnapshot | null;
    selectedId: number | null;
    loading: boolean;
    error: string | null;
};

const initialStateStore: CountdownStateStore = {
    items: [],
    selected: null,
    selectedId: null,
    loading: false,
    error: null,
};

const {subscribe, update} = writable(initialStateStore);

function resolveSelected(items: CountdownSnapshot[], selectedId: number | null): CountdownSnapshot | null {
    return selectedId === null
        ? null
        : items.find((x) => x.id === selectedId) ?? null;
}

function getSelectedIdOrThrow(): number {
    const selectedId = get(countdownStore).selectedId;
    if (selectedId === null)
        throw new Error("No selected countdown");
    return selectedId;
}

async function actionOnSelected(action: ((id: number) => Promise<void>)): Promise<void> {
    update((state) => ({...state, loading: true, error: null}));
    try {
        const selectedId = getSelectedIdOrThrow();
        await action(selectedId);
        await loadList();
    } catch (error) {
        update((state) => {
            const e_msg = error instanceof Error ? error.message : String(error);
            return {...state, error: e_msg}
        });
    } finally {
        update((state) => ({...state, loading: false}));
    }
}

export async function loadList() {
    update((state) => ({...state, loading: true, error: null}));
    try {
        const items = await listCountdowns();
        update((state) => ({...state, items}));
    } catch (error) {
        update((state) => {
            const e_msg = error instanceof Error ? error.message : String(error);
            return {...state, error: e_msg}
        });
    } finally {
        update((state) => ({...state, loading: false}));
    }
}

export async function select(id: number) {
    update((state) => ({...state, loading: true, error: null}));
    try {
        await loadList();
        update((state) => {
            const selected = resolveSelected(state.items, id);
            return {
                ...state,
                selectedId: id,
                selected: selected,
                error: selected === null ? "Selected countdown not found" : null
            };
        });
    } catch (error) {
        update((state) => {
            const e_msg = error instanceof Error ? error.message : String(error);
            return {...state, error: e_msg}
        });
    } finally {
        update((state) => ({...state, loading: false}));
    }
}

export async function create(label: string, duration: Duration) {
    update((state) => ({...state, loading: true, error: null}));
    try {
        const nId = await createCountdown(label, duration);
        await select(nId);
    } catch (error) {
        update((state) => {
            const e_msg = error instanceof Error ? error.message : String(error);
            return {...state, error: e_msg}
        });
    } finally {
        update((state) => ({...state, loading: false}));
    }
}

export async function deleteSelected() {
    update((state) => ({...state, loading: true, error: null}));
    try {
        const selectedId = getSelectedIdOrThrow();
        const items = get(countdownStore).items;
        let nId = items.findIndex((x) => x.id === selectedId) - 1;
        nId = nId < 0 ? 0 : nId == 0 ? 1 : 0;
        nId = items[nId].id;
        await deleteCountdown(selectedId);
        await select(nId)
    } catch (error) {
        update((state) => {
            const e_msg = error instanceof Error ? error.message : String(error);
            return {...state, error: e_msg}
        });
    } finally {
        update((state) => ({...state, loading: false}));
    }
}

export async function startSelected() {
    await actionOnSelected(startCountdown);
}

export async function resumeSelected() {
    await actionOnSelected(resumeCountdown);
}

export async function pauseSelected() {
    await actionOnSelected(pauseCountdown);
}

export async function resetSelected() {
    await actionOnSelected(resetCountdown);
}

export const countdownStore = {
    subscribe,
    loadList,
    select,
    create,
    startSelected,
    resumeSelected,
    pauseSelected,
    resetSelected,
    deleteSelected
};