// Global state management with Zustand

import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { User, Feed, Article } from "./api";

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  setAuth: (user: User, token: string) => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      isAuthenticated: false,
      setAuth: (user, token) => set({ user, token, isAuthenticated: true }),
      clearAuth: () => set({ user: null, token: null, isAuthenticated: false }),
    }),
    {
      name: "feedmind-auth",
      partialize: (state) => ({ token: state.token }),
    }
  )
);

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

  // Theme
  theme: "light" | "dark" | "system";
  setTheme: (theme: "light" | "dark" | "system") => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

      selectedFeedId: null,
      selectedFolderId: null,
      selectedArticleId: null,
      setSelectedFeed: (id) => set({ selectedFeedId: id, selectedFolderId: null }),
      setSelectedFolder: (id) => set({ selectedFolderId: id, selectedFeedId: null }),
      setSelectedArticle: (id) => set({ selectedArticleId: id }),

      viewMode: "list",
      setViewMode: (mode) => set({ viewMode: mode }),

      articleFilter: "unread",
      setArticleFilter: (filter) => set({ articleFilter: filter }),

      theme: "system",
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: "feedmind-ui",
    }
  )
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
