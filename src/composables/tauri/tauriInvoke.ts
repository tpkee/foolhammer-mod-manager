export function useTauriInvoke() {
  return (window as any).__TAURI__.core.invoke as <T>(...args: T[]) => Promise<unknown>
}
