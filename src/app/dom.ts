export function getMainContainer(): HTMLElement | null {
    return document.querySelector(".container");
}

export function getDiv(id: string): HTMLElement | null {
    return document.getElementById(id);
}
