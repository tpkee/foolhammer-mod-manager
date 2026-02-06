export function useInvoke() {
  return (window as any).__TAURI__.core.invoke as (...args: unknown[]) => Promise<unknown>
}
