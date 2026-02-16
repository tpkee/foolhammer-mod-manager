interface EditHistoryControls<T> {
  snapshots: Ref<T[]>
  undo: () => void
  redo: () => void
  commit: () => void
  cancel: () => void
  canUndo: ComputedRef<boolean>
  canRedo: ComputedRef<boolean>
}

export function useHistory<T extends object>(source: Ref<T>): EditHistoryControls<T> {
  const snapshots = ref<T[]>([]) as Ref<T[]>
  const index = ref(0)
  const isApplying = ref(false)
  let initialSnapshot = cloneValue(source.value)

  const canUndo = computed(() => index.value > 0)
  const canRedo = computed(() => index.value < snapshots.value.length - 1)

  function applySnapshot(target: T) {
    isApplying.value = true
    source.value = cloneValue(target)
    isApplying.value = false
  }

  function commitSnapshot(value: T) {
    snapshots.value = snapshots.value.slice(0, index.value + 1)
    snapshots.value.push(cloneValue(value))
    index.value = snapshots.value.length - 1
  }

  function undo() {
    if (!canUndo.value)
      return
    index.value -= 1

    const snapshot = snapshots.value[index.value]

    if (!snapshot) {
      console.warn('No snapshot found at index', index.value)
      return
    }

    applySnapshot(snapshot)
  }

  function redo() {
    if (!canRedo.value)
      return
    index.value += 1

    const snapshot = snapshots.value[index.value]

    if (!snapshot) {
      console.warn('No snapshot found at index', index.value)
      return
    }

    applySnapshot(snapshot)
  }

  function commit() {
    initialSnapshot = cloneValue(source.value)
    snapshots.value = [cloneValue(source.value)]
    index.value = 0
  }

  function cancel() {
    applySnapshot(initialSnapshot)
    snapshots.value = [cloneValue(initialSnapshot)]
    index.value = 0
  }

  watch(
    source,
    (value) => {
      if (isApplying.value)
        return

      if (snapshots.value.length === 0) {
        snapshots.value = [cloneValue(value)]
        index.value = 0
        return
      }

      if (!recursiveDeepEqual(value, snapshots.value[index.value]))
        commitSnapshot(value)
    },
    { deep: true, immediate: true },
  )

  return {
    snapshots,
    undo,
    redo,
    commit,
    cancel,
    canUndo,
    canRedo,
  }
}

function cloneValue<T extends object>(value: T): T {
  if (typeof structuredClone === 'function') {
    try {
      return structuredClone(toRaw(value))
    }
    catch (error) {
      console.warn('structuredClone failed, falling back to JSON clone', error)
    }
  }

  return JSON.parse(JSON.stringify(toRaw(value))) as T
}

function recursiveDeepEqual(a: unknown, b: unknown): boolean {
  if (Object.is(a, b))
    return true

  if (!a || !b || typeof a !== 'object' || typeof b !== 'object')
    return false

  if (Array.isArray(a) || Array.isArray(b)) {
    if (!Array.isArray(a) || !Array.isArray(b) || a.length !== b.length)
      return false

    for (let i = 0; i < a.length; i++) {
      if (!recursiveDeepEqual(a[i], b[i]))
        return false
    }

    return true
  }

  const aKeys = Object.keys(a as Record<string, unknown>)
  const bKeys = Object.keys(b as Record<string, unknown>)

  if (aKeys.length !== bKeys.length)
    return false

  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(b, key))
      return false

    const aValue = (a as Record<string, unknown>)[key]
    const bValue = (b as Record<string, unknown>)[key]

    if (!recursiveDeepEqual(aValue, bValue))
      return false
  }

  return true
}
