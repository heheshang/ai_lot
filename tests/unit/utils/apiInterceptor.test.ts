/**
 * Unit tests for ApiInterceptor
 *
 * @vitest-environment jsdom
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'

// Create mock functions
const mockInvoke = vi.fn()
const mockGetAccessToken = vi.fn()
const mockUseUserStore = vi.fn()

// Mock external dependencies before importing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}))

vi.mock('@/utils/secureStorage', () => ({
  getAccessToken: mockGetAccessToken,
}))

vi.mock('@/store/modules/user', () => ({
  useUserStore: mockUseUserStore,
}))

// Import after mocking
import { ApiInterceptor, ApiInterceptorError } from '@/utils/apiInterceptor'

describe('ApiInterceptor', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    // Stub window for tests that need it
    if (typeof window === 'undefined') {
      ;(global as any).window = {
        dispatchEvent: vi.fn(),
      }
    }
  })

  afterEach(() => {
    vi.resetAllMocks()
  })

  describe('constructor', () => {
    it('should use default config when no options provided', () => {
      const interceptor = new ApiInterceptor()
      expect((interceptor as any).config.retryAttempts).toBe(3)
      expect((interceptor as any).config.retryDelay).toBe(1000)
      expect((interceptor as any).config.enableAuth).toBe(true)
      expect((interceptor as any).config.enableRetry).toBe(true)
    })

    it('should merge custom config with defaults', () => {
      const interceptor = new ApiInterceptor({ retryAttempts: 5, retryDelay: 2000 })
      expect((interceptor as any).config.retryAttempts).toBe(5)
      expect((interceptor as any).config.retryDelay).toBe(2000)
      expect((interceptor as any).config.enableAuth).toBe(true)
      expect((interceptor as any).config.enableRetry).toBe(true)
    })
  })

  describe('successful request', () => {
    it('should return data on successful request', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockResolvedValue({ success: true, data: { id: 1 } })

      const interceptor = new ApiInterceptor()
      const result = await interceptor.request<{ id: number }>('test_command', { foo: 'bar' })

      expect(result).toEqual({ id: 1 })
      expect(mockInvoke).toHaveBeenCalledTimes(1)
      expect(mockInvoke).toHaveBeenCalledWith(
        'test_command',
        expect.objectContaining({
          foo: 'bar',
          _auth: 'test-token',
        })
      )
    })

    it('should handle legacy response format (direct data)', async () => {
      mockGetAccessToken.mockResolvedValue(null)
      mockInvoke.mockResolvedValue('legacy-data')

      const interceptor = new ApiInterceptor()
      const result = await interceptor.request<string>('test_command')

      expect(result).toBe('legacy-data')
    })

    it('should inject auth token when enabled', async () => {
      mockGetAccessToken.mockResolvedValue('my-auth-token')
      mockInvoke.mockResolvedValue({ success: true, data: 'result' })

      const interceptor = new ApiInterceptor({ enableAuth: true })
      await interceptor.request('test_command')

      expect(mockInvoke).toHaveBeenCalledWith(
        'test_command',
        expect.objectContaining({
          _auth: 'my-auth-token',
        })
      )
    })

    it('should not inject auth token when disabled', async () => {
      mockGetAccessToken.mockResolvedValue('my-auth-token')
      mockInvoke.mockResolvedValue({ success: true, data: 'result' })

      const interceptor = new ApiInterceptor({ enableAuth: false })
      await interceptor.request('test_command', { foo: 'bar' })

      expect(mockInvoke).toHaveBeenCalledWith('test_command', { foo: 'bar' })
    })
  })

  describe('error handling', () => {
    it('should throw ApiInterceptorError on failed response', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockResolvedValue({
        success: false,
        error: { message: 'Not found', code: 'NOT_FOUND', details: 'Resource missing' },
      })

      const interceptor = new ApiInterceptor()

      await expect(interceptor.request('test_command')).rejects.toThrow(ApiInterceptorError)
      await expect(interceptor.request('test_command')).rejects.toMatchObject({
        code: 'NOT_FOUND',
        message: 'Not found',
        details: 'Resource missing',
      })
    })

    it('should throw NETWORK_ERROR for failed requests without ApiResponse', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockRejectedValue(new Error('Network error'))

      const interceptor = new ApiInterceptor({ retryAttempts: 1, retryDelay: 10 })

      await expect(interceptor.request('test_command')).rejects.toThrow(ApiInterceptorError)
      await expect(interceptor.request('test_command')).rejects.toMatchObject({
        code: 'NETWORK_ERROR',
      })
    })
  })

  describe('retry logic', () => {
    it('should retry on network error when retry is enabled', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke
        .mockRejectedValueOnce(new Error('timeout'))
        .mockRejectedValueOnce(new Error('timeout'))
        .mockResolvedValue({ success: true, data: 'success' })

      const interceptor = new ApiInterceptor({ retryAttempts: 3, retryDelay: 10 })
      const result = await interceptor.request('test_command')

      expect(result).toBe('success')
      expect(mockInvoke).toHaveBeenCalledTimes(3)
    })

    it('should not retry when retry is disabled', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockRejectedValue(new Error('timeout'))

      const interceptor = new ApiInterceptor({ enableRetry: false, retryDelay: 10 })

      await expect(interceptor.request('test_command')).rejects.toThrow()
      expect(mockInvoke).toHaveBeenCalledTimes(1)
    })

    it('should not retry non-retryable errors', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockRejectedValue(new Error('validation error'))

      const interceptor = new ApiInterceptor({ retryAttempts: 3, retryDelay: 10 })

      await expect(interceptor.request('test_command')).rejects.toThrow()
      expect(mockInvoke).toHaveBeenCalledTimes(1)
    })

    it('should respect retryAttempts configuration', async () => {
      mockGetAccessToken.mockResolvedValue('test-token')
      mockInvoke.mockRejectedValue(new Error('timeout'))

      const interceptor = new ApiInterceptor({ retryAttempts: 5, retryDelay: 10 })

      await expect(interceptor.request('test_command')).rejects.toThrow()
      expect(mockInvoke).toHaveBeenCalledTimes(5)
    })
  })

  describe('token refresh', () => {
    it('should attempt token refresh on auth error', async () => {
      mockGetAccessToken.mockResolvedValue('expired-token')
      mockInvoke
        .mockResolvedValueOnce({
          success: false,
          error: { message: 'Token expired', code: 'TOKEN_EXPIRED' },
        })
        .mockResolvedValueOnce({ success: true, data: 'success' })

      const mockRefreshToken = vi.fn().mockResolvedValue(true)
      mockUseUserStore.mockReturnValue({
        refreshAccessToken: mockRefreshToken,
        logout: vi.fn(),
      })

      const interceptor = new ApiInterceptor({ retryDelay: 10 })
      const result = await interceptor.request('test_command')

      expect(result).toBe('success')
      expect(mockRefreshToken).toHaveBeenCalledTimes(1)
    })

    it('should handle auth failure and redirect to login', async () => {
      mockGetAccessToken.mockResolvedValue('expired-token')
      mockInvoke.mockResolvedValue({
        success: false,
        error: { message: 'Unauthorized', code: 'UNAUTHORIZED' },
      })

      const mockLogout = vi.fn()
      const dispatchEventSpy = vi.fn()
      mockUseUserStore.mockReturnValue({
        refreshAccessToken: vi.fn().mockResolvedValue(false),
        logout: mockLogout,
      })

      // Stub window.dispatchEvent
      const originalWindow = global.window
      global.window = { ...originalWindow, dispatchEvent: dispatchEventSpy } as any

      const interceptor = new ApiInterceptor({ retryDelay: 10 })

      await expect(interceptor.request('test_command')).rejects.toThrow()
      expect(mockLogout).toHaveBeenCalled()
      expect(dispatchEventSpy).toHaveBeenCalledWith(expect.any(CustomEvent))

      global.window = originalWindow
    })

    it('should not retry auth errors through normal retry path', async () => {
      mockGetAccessToken.mockResolvedValue('expired-token')
      mockInvoke
        .mockResolvedValueOnce({
          success: false,
          error: { message: 'Unauthorized', code: 'UNAUTHORIZED' },
        })
        .mockResolvedValue({ success: true, data: 'success' })

      const mockRefreshToken = vi.fn().mockResolvedValue(true)
      mockUseUserStore.mockReturnValue({
        refreshAccessToken: mockRefreshToken,
        logout: vi.fn(),
      })

      const interceptor = new ApiInterceptor({ retryAttempts: 3, retryDelay: 10 })
      const result = await interceptor.request('test_command')

      expect(result).toBe('success')
      // First call gets UNAUTHORIZED, triggers refresh, second call succeeds
      expect(mockRefreshToken).toHaveBeenCalledTimes(1)
    })
  })

  describe('withoutAuth', () => {
    it('should return new interceptor with auth disabled', () => {
      const interceptor = new ApiInterceptor()
      const withoutAuthInterceptor = interceptor.withoutAuth()

      expect(withoutAuthInterceptor).not.toBe(interceptor)
      expect((withoutAuthInterceptor as any).config.enableAuth).toBe(false)
    })

    it('should preserve other config options', () => {
      const interceptor = new ApiInterceptor({ retryAttempts: 5 })
      const withoutAuthInterceptor = interceptor.withoutAuth()

      expect((withoutAuthInterceptor as any).config.retryAttempts).toBe(5)
      expect((withoutAuthInterceptor as any).config.enableRetry).toBe(true)
    })
  })

  describe('withoutRetry', () => {
    it('should return new interceptor with retry disabled', () => {
      const interceptor = new ApiInterceptor()
      const withoutRetryInterceptor = interceptor.withoutRetry()

      expect(withoutRetryInterceptor).not.toBe(interceptor)
      expect((withoutRetryInterceptor as any).config.enableRetry).toBe(false)
    })

    it('should preserve other config options', () => {
      const interceptor = new ApiInterceptor({ enableAuth: false })
      const withoutRetryInterceptor = interceptor.withoutRetry()

      expect((withoutRetryInterceptor as any).config.enableAuth).toBe(false)
      expect((withoutRetryInterceptor as any).config.retryAttempts).toBe(3)
    })
  })

  describe('ApiInterceptorError', () => {
    it('should correctly identify auth errors', () => {
      const authError1 = new ApiInterceptorError('Token expired', 'TOKEN_EXPIRED')
      const authError2 = new ApiInterceptorError('Unauthorized', 'UNAUTHORIZED')
      const otherError = new ApiInterceptorError('Not found', 'NOT_FOUND')

      expect(authError1.isAuthError).toBe(true)
      expect(authError2.isAuthError).toBe(true)
      expect(otherError.isAuthError).toBe(false)
    })

    it('should have correct properties', () => {
      const error = new ApiInterceptorError('Test message', 'TEST_CODE', 'Test details')

      expect(error.message).toBe('Test message')
      expect(error.code).toBe('TEST_CODE')
      expect(error.details).toBe('Test details')
      expect(error.name).toBe('ApiInterceptorError')
    })
  })

  describe('concurrent refresh requests', () => {
    it('should share refresh promise when multiple requests fail simultaneously', async () => {
      mockGetAccessToken.mockResolvedValue('expired-token')

      mockInvoke
        .mockResolvedValueOnce({
          success: false,
          error: { message: 'Token expired', code: 'TOKEN_EXPIRED' },
        })
        .mockResolvedValueOnce({
          success: false,
          error: { message: 'Token expired', code: 'TOKEN_EXPIRED' },
        })
        .mockResolvedValue({ success: true, data: 'success' })

      let refreshResolve: (value: boolean) => void
      const refreshPromise = new Promise<boolean>((resolve) => {
        refreshResolve = resolve
      })

      const mockRefreshToken = vi.fn().mockReturnValue(refreshPromise)
      mockUseUserStore.mockReturnValue({
        refreshAccessToken: mockRefreshToken,
        logout: vi.fn(),
      })

      const interceptor = new ApiInterceptor({ retryDelay: 10 })

      const request1 = interceptor.request('test_command')
      const request2 = interceptor.request('test_command')

      refreshResolve!(true)

      const [result1, result2] = await Promise.all([request1, request2])

      expect(result1).toBe('success')
      expect(result2).toBe('success')
      expect(mockRefreshToken).toHaveBeenCalledTimes(1)
    })
  })
})
