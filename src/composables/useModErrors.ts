import type { ModResponseDto } from '~/types/dto'

type PushError = (name: string, error: ModError) => void

interface ModIndices {
  orderIndex: Map<string, number | null>
  enabledIndex: Map<string, boolean>
}

// AI generated code, if it breaks it's cuz it's crap

/**
 * Computes a map of mod errors for a given list of mods.
 *
 * Detected error types:
 * - `duplicate_order`   – two or more mods share the same order number
 * - `missing_dependency` – a declared dependency is absent from the list or disabled
 * - `dependency_order`  – a dependency is present & enabled but violates load-order constraints
 *
 * Dependency semantics: `[loadBefore, depName]`
 * - All entries are required dependencies (must be present and enabled).
 * - `loadBefore = true`  → `depName` must also have a *lower* order number than this mod (loads first).
 * - `loadBefore = false` → no load-order constraint, only presence/enabled is checked.
 * Order constraints are only evaluated when both mods have a non-null order value.
 */
export function useModErrors(
  mods: MaybeRefOrGetter<ModResponseDto[]>,
): ComputedRef<Map<string, ModError[]>> {
  return computed(() => {
    const list = toValue(mods)
    const errors = new Map<string, ModError[]>()

    const push: PushError = (name, error) => {
      const bucket = errors.get(name) ?? []
      bucket.push(error)
      errors.set(name, bucket)
    }

    const { orderIndex, enabledIndex } = buildIndices(list)
    collectDuplicateOrderErrors(list, push)
    collectDependencyErrors(list, orderIndex, enabledIndex, push)

    return errors
  })
}

// ---------- module-private helpers ----------------------------------------

function buildIndices(list: ModResponseDto[]): ModIndices {
  const orderIndex = new Map<string, number | null>()
  const enabledIndex = new Map<string, boolean>()

  for (const mod of list) {
    if (!mod.name)
      continue
    orderIndex.set(mod.name, mod.order ?? null)
    enabledIndex.set(mod.name, mod.enabled ?? false)
  }

  return { orderIndex, enabledIndex }
}

function collectDuplicateOrderErrors(list: ModResponseDto[], push: PushError) {
  const orderGroups = new Map<number, string[]>()

  for (const mod of list) {
    if (mod.order == null || !mod.name)
      continue
    const group = orderGroups.get(mod.order) ?? []
    group.push(mod.name)
    orderGroups.set(mod.order, group)
  }

  for (const [order, names] of orderGroups) {
    if (names.length < 2)
      continue
    for (const name of names)
      push(name, { type: 'duplicate_order', message: `Duplicate order number: ${order}` })
  }
}

/**
 * Resolves a dependency name against the enabled-index, accounting for an
 * optional `.pack` extension — returns the matched key or `undefined`.
 */
function resolveDependencyName(depName: string, enabledIndex: Map<string, boolean>): string | undefined {
  if (enabledIndex.has(depName))
    return depName
  const stripped = depName.endsWith('.pack') ? depName.slice(0, -5) : undefined
  return stripped && enabledIndex.has(stripped) ? stripped : undefined
}

function collectDependencyErrors(
  list: ModResponseDto[],
  orderIndex: Map<string, number | null>,
  enabledIndex: Map<string, boolean>,
  push: PushError,
) {
  for (const mod of list) {
    if (!mod.name || !mod.dependencies?.length || !mod.enabled)
      continue

    const thisOrder = orderIndex.get(mod.name) ?? null

    for (const [loadBefore, depName] of mod.dependencies as [boolean, string][]) {
      const match = resolveDependencyName(depName, enabledIndex)
      const depEnabled = match ? (enabledIndex.get(match) ?? false) : false

      if (!match || !depEnabled) {
        push(mod.name, { type: 'missing_dependency', message: `Missing or disabled dependency: '${depName}'` })
        continue
      }

      // load-order constraint only applies when loadBefore=true (both orders must be known)
      if (!loadBefore || thisOrder == null)
        continue

      const depOrder = orderIndex.get(match) ?? null
      if (depOrder == null)
        continue

      // dep must load first → depOrder < thisOrder
      if (depOrder >= thisOrder)
        push(mod.name, { type: 'dependency_order', message: `'${depName}' must load before this mod` })
    }
  }
}
