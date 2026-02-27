import {initCountdownController} from "../features/countdown/controller";
import {getDiv} from "./dom";

export async function initApp(): Promise<void> {
    const countdownContainer = getDiv("countdown-container");
    if (countdownContainer) {
        await initCountdownController(countdownContainer);
    }
}
