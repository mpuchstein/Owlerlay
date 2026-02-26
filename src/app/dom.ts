export type GreetDom = {
  form: HTMLFormElement;
  input: HTMLInputElement;
  message: HTMLElement;
};

export function getMainContainer(): HTMLElement | null {
  return document.querySelector(".container");
}

export function getGreetDom(): GreetDom | null {
  const form = document.querySelector<HTMLFormElement>("#greet-form");
  const input = document.querySelector<HTMLInputElement>("#greet-input");
  const message = document.querySelector<HTMLElement>("#greet-msg");

  if (!form || !input || !message) {
    return null;
  }

  return { form, input, message };
}
