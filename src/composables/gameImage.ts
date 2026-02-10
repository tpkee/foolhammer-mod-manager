export function useGameImage(gameId: MaybeRefOrGetter<string>) {
  switch (toValue(gameId)) {
    case '1142710': // wh3
      return '/images/games/wh3.webp'

    default:
      return null
  }
}
