export function gradient(v: number, stops: string[]): string {
    const n = stops.length;
    if (n === 0) return '#000000';
    if (n === 1) return stops[0]!;
    const scaledV = v * (n - 1);
    const index = Math.floor(scaledV);
    const t = scaledV - index;
    const c1 = parseInt(stops[index]!.substring(1), 16);
    const c2 = parseInt(stops[Math.min(index + 1, n - 1)]!.substring(1), 16);
    const r = Math.round(((c1 >> 16) * (1 - t)) + ((c2 >> 16) * t));
    const g = Math.round((((c1 >> 8) & 0xFF) * (1 - t)) + (((c2 >> 8) & 0xFF) * t));
    const b = Math.round(((c1 & 0xFF) * (1 - t)) + ((c2 & 0xFF) * t));
    return `#${((r << 16) | (g << 8) | b).toString(16).padStart(6, '0')}`;
}