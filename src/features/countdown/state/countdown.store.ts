import {
  fetchCountdownSnapshot,
  pauseCountdown,
  resetCountdown,
  resumeCountdown,
  startCountdown,
} from "../api/countdown.client";
import {createCountdownView} from "../view";

export async function initCountdownController(container: HTMLElement): Promise<void> {
    const view = createCountdownView(container);

    const refreshSnapshot = async (): Promise<void> => {
        try {
            const snapshot = await fetchCountdownSnapshot();
            view.setSnapshot(snapshot);
        } catch (error) {
            view.setError(`snapshot error: ${String(error)}`);
        }
    };

    const runAction = async (action: () => Promise<void>): Promise<void> => {
        try {
            await action();
            await refreshSnapshot();
        } catch (error) {
            view.setError(`command error: ${String(error)}`);
        }
    };

    view.onStart(() => {
        void runAction(startCountdown);
    });
    view.onPause(() => {
        void runAction(pauseCountdown);
    });
    view.onResume(() => {
        void runAction(resumeCountdown);
    });
    view.onReset(() => {
        void runAction(resetCountdown);
    });
    view.onRefresh(() => {
        void refreshSnapshot();
    });

    await refreshSnapshot();
}
