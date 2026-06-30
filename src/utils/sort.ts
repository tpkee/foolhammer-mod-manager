export function compareModNames(a: string, b: string): number {
  const aLetters = a.toLowerCase()
  const bLetters = b.toLowerCase()

  const aLen = aLetters.length
  const bLen = bLetters.length
  const totalLen = Math.max(aLen, bLen)

  for (let i = 0; i < totalLen; i++) {
    if (i === aLen)
      return 1
    if (i === bLen)
      return -1

    const aCode = aLetters.charCodeAt(i)
    const bCode = bLetters.charCodeAt(i)

    if (aCode < bCode)
      return -1
    if (aCode > bCode)
      return 1
  }

  return 0
}
