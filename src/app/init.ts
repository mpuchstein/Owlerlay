import { initCountdownController } from "../features/countdown/controller";
import { invokeCommand } from "../shared/tauri/invoke";
import { getGreetDom, getMainContainer } from "./dom";

async function greet(name: string, messageEl: HTMLElement): Promise<void> {
  try {
    const message = await invokeCommand<string>("greet", { name });
    messageEl.textContent = message;
  } catch (error) {
    messageEl.textContent = `greet error: ${String(error)}`;
  }
}

function bindGreetForm(): void {
  const greetDom = getGreetDom();
  if (!greetDom) {
    return;
  }

  greetDom.form.addEventListener("submit", (event) => {
    event.preventDefault();
    void greet(greetDom.input.value, greetDom.message);
  });
}

export async function initApp(): Promise<void> {
  bindGreetForm();

  const container = getMainContainer();
  if (container) {
    await initCountdownController(container);
  }
}
