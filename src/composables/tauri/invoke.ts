export function useInvoke() {
  return (window as any).__TAURI__.core.invoke as <T>(...args: T[]) => Promise<unknown>
}
