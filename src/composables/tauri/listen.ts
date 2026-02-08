type TUnlisten = () => void

export function useListen() {
  return (window as any).__TAURI__.core.listen as <T>(...args: T[]) => Promise<TUnlisten>
}
