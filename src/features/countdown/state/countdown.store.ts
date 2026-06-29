import type {
  CountdownSnapshot,
  CountdownSnapshotDto,
  CountdownTickPayload,
} from "../model/countdown.types";
import { get, writable } from "svelte/store";
import {
  createCountdown,
  deleteCountdown,
  listCountdowns,
  pauseCountdown,
  resetCountdown,
  resumeCountdown,
  startCountdown,
} from "../api/countdown.client";
import type { Duration } from "../../../shared/time/duration";
import { millisToDuration } from "../../../shared/time/duration";
import { mapSnapshotDtoToSnapshot } from "../model/countdown.mapper";
import { listen } from "@tauri-apps/api/event";

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

const { subscribe, update } = writable(initialStateStore);

function getSelectedIdOrThrow(): number {
  const selectedId = get(countdownStore).selectedId;
  if (selectedId === null) throw new Error("No selected countdown");
  return selectedId;
}

async function actionOnSelected(
  action: (id: number) => Promise<void>,
): Promise<void> {
  update((state) => ({ ...state, loading: true, error: null }));
  try {
    const selectedId = getSelectedIdOrThrow();
    await action(selectedId);
  } catch (error) {
    update((state) => {
      const e_msg = error instanceof Error ? error.message : String(error);
      return { ...state, error: e_msg };
    });
  } finally {
    update((state) => ({ ...state, loading: false }));
  }
}

export async function loadList() {
  update((state) => ({ ...state, loading: true, error: null }));
  try {
    const items = await listCountdowns();
    update((state) => ({ ...state, items }));
  } catch (error) {
    update((state) => {
      const e_msg = error instanceof Error ? error.message : String(error);
      return { ...state, error: e_msg };
    });
  } finally {
    update((state) => ({ ...state, loading: false }));
  }
}

export function select(id: number) {
  update((s) => {
    const selected = s.items.find((x) => x.id === id) ?? null;
    return { ...s, selectedId: id, selected };
  });
}

export async function create(label: string, duration: Duration) {
  update((state) => ({ ...state, loading: true, error: null }));
  try {
    const nId = await createCountdown(label, duration);
    const freshItems = await listCountdowns();
    update((s) => ({
      ...s,
      items: freshItems,
      selectedId: nId,
      selected: freshItems.find((x) => x.id === nId) ?? null,
    }));
  } catch (error) {
    update((state) => {
      const e_msg = error instanceof Error ? error.message : String(error);
      return { ...state, error: e_msg };
    });
  } finally {
    update((state) => ({ ...state, loading: false }));
  }
}

export async function deleteSelected() {
  update((state) => ({ ...state, loading: true, error: null }));
  try {
    const selectedId = getSelectedIdOrThrow();
    const remainingItems = get(countdownStore).items.filter(
      (x) => x.id !== selectedId,
    );
    const next = remainingItems[0] ?? null;
    await deleteCountdown(selectedId);
    update((s) => ({
      ...s,
      selected: next,
      selectedId: next?.id ?? null,
    }));
  } catch (error) {
    update((state) => {
      const e_msg = error instanceof Error ? error.message : String(error);
      return { ...state, error: e_msg };
    });
  } finally {
    update((state) => ({ ...state, loading: false }));
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

export async function initStoreListeners(): Promise<() => void> {
  const unlistenTick = await listen<CountdownTickPayload>(
    "countdown_tick",
    (e) => {
      update((s) => {
        const duration = millisToDuration(e.payload.remaining_ms);
        // Patch the ticked countdown in place so its rail row counts down live
        // (and the detail panel, which reads the selected item, with it). Guard
        // on Running: the backend only ticks running timers, so a tick that
        // races in after a reset/pause is stale and must not overwrite the
        // authoritative value countdown_changed just set.
        const isLiveTick = (it: CountdownSnapshot) =>
          it.id === e.payload.id && it.state === "Running";
        const items = s.items.map((it) =>
          isLiveTick(it) ? { ...it, duration } : it,
        );
        const selected =
          s.selected && isLiveTick(s.selected)
            ? { ...s.selected, duration }
            : s.selected;
        return { ...s, items, selected };
      });
    },
  );

  const unlistenChanged = await listen<CountdownSnapshotDto[]>(
    "countdown_changed",
    (e) => {
      const items = e.payload.map(mapSnapshotDtoToSnapshot);
      update((s) => {
        const selected =
          s.selectedId !== null
            ? (items.find((x) => x.id === s.selectedId) ?? null)
            : null;
        return { ...s, items, selected };
      });
    },
  );

  return () => {
    unlistenTick();
    unlistenChanged();
  };
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
  deleteSelected,
  initStoreListeners,
};
