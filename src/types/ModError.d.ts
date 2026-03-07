type ErrorType = 'duplicate_order' | 'missing_dependency' | 'dependency_order'

interface ModError {
  type: ErrorType
  message: string
}
