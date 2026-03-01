//TODO: Implement countdown store

import type {CountdownSnapshot} from "../model/countdown.types";
import {writable} from "svelte/store";
import {createCountdown, listCountdowns} from "../api/countdown.client";
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

const {subscribe} = writable(initialStateStore);

function resolveSelected(items: CountdownSnapshot[], selectedId: number | null): CountdownSnapshot | null {
    return selectedId === null
        ? null
        : items.find((x) => x.id === selectedId) ?? null;
}

export function loadList() {
    initialStateStore.loading = true;
    listCountdowns().then((items) => {
        initialStateStore.items = items;
        initialStateStore.selectedId = initialStateStore.selectedId === null
            ? (items.length > 0 ? items[0].id : null)
            : initialStateStore.selectedId;
        initialStateStore.selected = resolveSelected(items, initialStateStore.selectedId);
        initialStateStore.loading = false;
    });
}

export function select(id: number) {
    initialStateStore.selectedId = id;
    initialStateStore.selected = resolveSelected(initialStateStore.items, id);
}

export function create(label: string, duration: Duration) {
    createCountdown(label, duration).then((id) => {
        loadList();
        initialStateStore.selectedId = id;
        initialStateStore.selected = resolveSelected(initialStateStore.items, id);
    })
}

export function startSelected() {

}

export function resumeSelected() {

}

export function pauseSelected() {

}

export function resetSelected() {

}

export function deleteSelected() {

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