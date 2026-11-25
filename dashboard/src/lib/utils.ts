import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// The backend writes some times as ISO strings with no zone (Python's
// datetime.utcnow().isoformat()). Those are UTC, but the browser would read them
// as local time, so a zone-less string gets a Z before it is parsed.
export function formatIsoTime(value: string): string {
  const zoned = /([zZ]|[+-]\d\d:?\d\d)$/.test(value) ? value : `${value}Z`
  const d = new Date(zoned)
  return Number.isNaN(d.getTime()) ? value : d.toLocaleString()
}
