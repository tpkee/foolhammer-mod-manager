import type { Ref } from 'vue'

type PreferredDirection = 'vertical' | 'horizontal'

interface UseFloatingPositionOptions {
  offset?: number
  isVisible?: Ref<boolean>
  preferredDirection?: PreferredDirection
}

export function useFloatingPosition(
  triggerRef: Ref<HTMLElement | null>,
  floatingRef: Ref<HTMLElement | null>,
  options: UseFloatingPositionOptions = {},
) {
  const { offset = 8, isVisible, preferredDirection = 'vertical' } = options

  const floatingStyles = ref<Record<string, string>>({
    top: '0px',
    left: '0px',
  })

  function updatePosition() {
    const trigger = triggerRef.value
    const floating = floatingRef.value
    if (!trigger || !floating)
      return

    const triggerRect = trigger.getBoundingClientRect()
    const floatingRect = floating.getBoundingClientRect()
    const viewportWidth = window.innerWidth
    const viewportHeight = window.innerHeight

    // Measure available space in each direction
    const spaceBelow = viewportHeight - triggerRect.bottom - offset
    const spaceAbove = triggerRect.top - offset
    const spaceRight = viewportWidth - triggerRect.right - offset
    const spaceLeft = triggerRect.left - offset

    let top: number
    let left: number

    if (preferredDirection === 'horizontal') {
      // Prefer left/right positioning, fallback to top/bottom
      const spaceRightTotal = viewportWidth - triggerRect.right - offset
      const spaceLeftTotal = triggerRect.left - offset

      // Horizontal: prefer aligning to the right, flip to left if not enough room
      if (spaceRightTotal >= floatingRect.width || spaceRightTotal >= spaceLeftTotal) {
        left = triggerRect.right + offset
      }
      else {
        left = triggerRect.left - floatingRect.width - offset
      }

      // Clamp horizontal to viewport
      left = Math.max(0, Math.min(left, viewportWidth - floatingRect.width))

      // Vertical fallback: center vertically on the trigger
      const triggerCenterY = triggerRect.top + triggerRect.height / 2
      top = triggerCenterY - floatingRect.height / 2

      // If centered position goes out of bounds, prefer above, then below
      if (top < 0) {
        top = Math.max(0, triggerRect.top - floatingRect.height - offset)
      }
      else if (top + floatingRect.height > viewportHeight) {
        top = Math.min(viewportHeight - floatingRect.height, triggerRect.bottom + offset)
      }

      // Final vertical clamp
      top = Math.max(0, Math.min(top, viewportHeight - floatingRect.height))
    }
    else {
      // Vertical: prefer below, flip to above if not enough room
      if (spaceBelow >= floatingRect.height || spaceBelow >= spaceAbove) {
        top = triggerRect.bottom + offset
      }
      else {
        top = triggerRect.top - floatingRect.height - offset
      }

      // Horizontal: prefer aligning start (left edge), flip to end (right edge) if not enough room
      if (spaceRight >= floatingRect.width || spaceRight >= spaceLeft) {
        left = triggerRect.left
      }
      else {
        left = triggerRect.right - floatingRect.width
      }

      // Clamp to viewport bounds
      top = Math.max(0, Math.min(top, viewportHeight - floatingRect.height))
      left = Math.max(0, Math.min(left, viewportWidth - floatingRect.width))
    }

    floatingStyles.value = {
      top: `${top}px`,
      left: `${left}px`,
    }
  }

  // Reposition on scroll (any ancestor) and resize
  useEventListener('scroll', () => {
    if (!isVisible || isVisible.value)
      updatePosition()
  }, { capture: true, passive: true })

  useEventListener('resize', () => {
    if (!isVisible || isVisible.value)
      updatePosition()
  }, { passive: true })

  return {
    floatingStyles,
    updatePosition,
  }
}

