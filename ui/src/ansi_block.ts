import { mirage } from "./ayu";

// returns a codeblock as an html element with ANSI escape codes converted to HTML styles
export function ansiBlock(text: string): string {
    const ansiRegex = /\x1b\[(\d+)(;\d+)*m(.*?)\x1b\[0m/g;

    return `<pre style='background-color: ${mirage.ui.panel.bg.hex()}; color: ${mirage.terminal.white.hex()}; padding: 5px; padding-left: 10px; border-radius: 5px; width: min-content;'>` + text.replace(ansiRegex, (match, p1, p2, p3) => {
        const codes = [p1, ...(p2 ? p2.split(';').slice(1) : [])].map(Number);
        let style = '';

        codes.forEach(code => {
            if (code === 1) {
                style += 'font-weight: bold;';
            }
            else if (code === 2) {
                style += 'opacity: 0.5;';
            }
            else if (code === 30) {
                style += `color: ${mirage.terminal.white.hex()};`;
            }
            else if (code === 31) {
                style += `color: ${mirage.terminal.red.hex()};`;
            }
            else if (code === 32) {
                style += `color: ${mirage.terminal.green.hex()};`;
            }
            else if (code === 33) {
                style += `color: ${mirage.terminal.yellow.hex()};`;
            }
            else if (code === 34) {
                style += `color: ${mirage.terminal.blue.hex()};`;
            }
            else if (code === 35) {
                style += `color: ${mirage.terminal.magenta.hex()};`;
            }
            else if (code === 36) {
                style += `color: ${mirage.terminal.cyan.hex()};`;
            }
            else if (code === 37) {
                style += `color: ${mirage.terminal.white.hex()};`;
            }
        });

        return `<span style="${style}">${p3}</span>`;
    }) + `</pre>`;

}