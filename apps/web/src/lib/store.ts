// Global state management with Zustand

import { create } from "zustand";
import { persist } from "zustand/middleware";
import { useEffect, useCallback } from "react";
import type { User } from "./api";
import { authApi, setAuthToken, getAuthToken } from "./api";

// Cookie helper for auth token (synced with middleware)
const AUTH_COOKIE_NAME = "feedmind_token";

function setAuthCookie(token: string, expiresAt?: string) {
  const expires = expiresAt
    ? new Date(expiresAt)
    : new Date(Date.now() + 7 * 24 * 60 * 60 * 1000);
  document.cookie = `${AUTH_COOKIE_NAME}=${token}; path=/; expires=${expires.toUTCString()}; SameSite=Lax`;
}

function clearAuthCookie() {
  document.cookie = `${AUTH_COOKIE_NAME}=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT`;
}

interface AuthState {
  user: User | null;
  token: string | null;
  expiresAt: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  isInitialized: boolean;
  setAuth: (user: User, token: string, expiresAt?: string) => void;
  clearAuth: () => void;
  setLoading: (loading: boolean) => void;
  setInitialized: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      expiresAt: null,
      isAuthenticated: false,
      isLoading: true,
      isInitialized: false,
      setAuth: (user, token, expiresAt) => {
        setAuthToken(token);
        setAuthCookie(token, expiresAt);
        set({
          user,
          token,
          expiresAt: expiresAt ?? null,
          isAuthenticated: true,
          isLoading: false,
        });
      },
      clearAuth: () => {
        setAuthToken(null);
        clearAuthCookie();
        set({
          user: null,
          token: null,
          expiresAt: null,
          isAuthenticated: false,
          isLoading: false,
        });
      },
      setLoading: (loading) => set({ isLoading: loading }),
      setInitialized: () => set({ isInitialized: true, isLoading: false }),
    }),
    {
      name: "feedmind-auth",
      partialize: (state) => ({
        token: state.token,
        expiresAt: state.expiresAt,
      }),
    },
  ),
);

/**
 * Hook to manage authentication state on app load.
 * Handles:
 * - Initial auth check
 * - Token refresh when needed
 * - Syncing token with API client and cookies
 */
export function useAuth() {
  const {
    user,
    token,
    expiresAt,
    isAuthenticated,
    isLoading,
    isInitialized,
    setAuth,
    clearAuth,
    setLoading,
    setInitialized,
  } = useAuthStore();

  const checkAuth = useCallback(async () => {
    // Already initialized or no token stored
    if (isInitialized) return;

    const storedToken = token || getAuthToken();
    if (!storedToken) {
      setInitialized();
      return;
    }

    // Sync token with API client and cookie
    setAuthToken(storedToken);
    if (typeof document !== "undefined") {
      setAuthCookie(storedToken, expiresAt ?? undefined);
    }

    // Check if token is about to expire (within 5 minutes)
    const shouldRefresh =
      expiresAt && new Date(expiresAt).getTime() - Date.now() < 5 * 60 * 1000;

    try {
      if (shouldRefresh) {
        // Token is expiring soon, refresh it
        const response = await authApi.refresh();
        setAuth(
          response.data.user,
          response.data.token,
          response.data.expires_at,
        );
      } else {
        // Validate current token by fetching user
        const response = await authApi.me();
        setAuth(response.data, storedToken, expiresAt ?? undefined);
      }
    } catch {
      // Token is invalid, clear auth
      clearAuth();
    }

    setInitialized();
  }, [token, expiresAt, isInitialized, setAuth, clearAuth, setInitialized]);

  // Check auth on mount
  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  const logout = useCallback(async () => {
    setLoading(true);
    try {
      await authApi.logout();
    } catch {
      // Ignore errors, still clear local state
    }
    clearAuth();
  }, [clearAuth, setLoading]);

  return {
    user,
    token,
    isAuthenticated,
    isLoading,
    isInitialized,
    logout,
    checkAuth,
  };
}

interface UIState {
  // Sidebar
  sidebarCollapsed: boolean;
  toggleSidebar: () => void;

  // Selected items
  selectedFeedId: string | null;
  selectedFolderId: string | null;
  selectedArticleId: string | null;
  setSelectedFeed: (id: string | null) => void;
  setSelectedFolder: (id: string | null) => void;
  setSelectedArticle: (id: string | null) => void;

  // View mode
  viewMode: "list" | "card" | "magazine";
  setViewMode: (mode: "list" | "card" | "magazine") => void;

  // Article filter
  articleFilter: "all" | "unread" | "starred";
  setArticleFilter: (filter: "all" | "unread" | "starred") => void;

  // Category filter
  selectedCategories: string[];
  setSelectedCategories: (categories: string[]) => void;
  toggleCategory: (category: string) => void;
  clearCategories: () => void;

  // Theme
  theme: "light" | "dark" | "system";
  setTheme: (theme: "light" | "dark" | "system") => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      toggleSidebar: () =>
        set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

      selectedFeedId: null,
      selectedFolderId: null,
      selectedArticleId: null,
      setSelectedFeed: (id) =>
        set({ selectedFeedId: id, selectedFolderId: null, selectedCategories: [] }),
      setSelectedFolder: (id) =>
        set({ selectedFolderId: id, selectedFeedId: null, selectedCategories: [] }),
      setSelectedArticle: (id) => set({ selectedArticleId: id }),

      viewMode: "list",
      setViewMode: (mode) => set({ viewMode: mode }),

      articleFilter: "unread",
      setArticleFilter: (filter) => set({ articleFilter: filter }),

      selectedCategories: [],
      setSelectedCategories: (categories) => set({ selectedCategories: categories }),
      toggleCategory: (category) =>
        set((state) => ({
          selectedCategories: state.selectedCategories.includes(category)
            ? state.selectedCategories.filter((c) => c !== category)
            : [...state.selectedCategories, category],
        })),
      clearCategories: () => set({ selectedCategories: [] }),

      theme: "system",
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: "feedmind-ui",
    },
  ),
);

// Keyboard shortcuts
interface ShortcutsState {
  enabled: boolean;
  setEnabled: (enabled: boolean) => void;
}

export const useShortcutsStore = create<ShortcutsState>((set) => ({
  enabled: true,
  setEnabled: (enabled) => set({ enabled }),
}));
