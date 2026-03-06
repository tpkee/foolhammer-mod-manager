import Sortable, { MultiDrag } from 'sortablejs'

Sortable.mount(new MultiDrag())

export function useMultiDrag(
  el: MaybeRefOrGetter<Nullable<HTMLElement>>,
  enabled: MaybeRefOrGetter<boolean>,
  onDragEnd: (event: Sortable.SortableEvent) => void,
) {
  const instance = shallowRef<Nullable<Sortable>>(null)

  watch(() => toValue(el), (elValue) => {
    if (elValue) {
      instance.value = initSortable(elValue, onDragEnd)
      instance.value.option('disabled', !toValue(enabled))
    }
    else {
      instance.value?.destroy()
      instance.value = null
    }
  }, { immediate: true })

  watch(() => toValue(enabled), (value) => {
    instance.value?.option('disabled', !value)
  })

  return { instance }
}

function initSortable(el: HTMLElement, onDragEnd: (event: Sortable.SortableEvent) => void) {
  return Sortable.create(el, {
    multiDrag: true,
    selectedClass: 'drag-highlight',
    chosenClass: 'drag-highlight',
    handle: '.drag-handle',

    removeCloneOnHide: true,
    emptyInsertThreshold: 5,
    forceFallback: true,

    onStart: () => document.body.classList.add('is-dragging'),
    onEnd(event) {
      document.body.classList.remove('is-dragging')
      onDragEnd(event)
    },
  })
}
