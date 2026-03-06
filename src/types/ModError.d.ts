type ErrorType = 'duplicate_order'

interface ModError {
  type: ErrorType
  message: string
}
