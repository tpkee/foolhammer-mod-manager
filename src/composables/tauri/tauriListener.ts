type TUnlisten = () => void

export async function useTauriListener(event: string, callback: (event: any) => void) {
  const listener = (window as any).__TAURI__.core.listen as (event: string, callback: (event: unknown) => void) => Promise<TUnlisten>

  const unlisten = await listener(event, callback)

  onUnmounted(unlisten)
}
