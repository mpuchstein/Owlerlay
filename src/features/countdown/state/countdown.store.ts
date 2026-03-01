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

const initalStateStore: CountdownStateStore = {
    items: [],
    selected: null,
    selectedId: null,
    loading: false,
    error: null,
};

const {subscribe} = writable(initalStateStore);

function resolveSelected(items: CountdownSnapshot[], selectedId: number | null): CountdownSnapshot | null {
    return selectedId === null
        ? null
        : items.find((x) => x.id === selectedId) ?? null;
}

export function loadList() {
    initalStateStore.loading = true;
    listCountdowns().then((items) => {
        initalStateStore.items = items;
        initalStateStore.selectedId = initalStateStore.selectedId === null
            ? (items.length > 0 ? items[0].id : null)
            : initalStateStore.selectedId;
        initalStateStore.selected = resolveSelected(items, initalStateStore.selectedId);
        initalStateStore.loading = false;
    });
}

export function select(id: number) {
    initalStateStore.selectedId = id;
    initalStateStore.selected = resolveSelected(initalStateStore.items, id);
}

export function create(label: string, duration: Duration) {
    createCountdown(label, duration).then((id) => {
        loadList();
        initalStateStore.selectedId = id;
        initalStateStore.selected = resolveSelected(initalStateStore.items, id);
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