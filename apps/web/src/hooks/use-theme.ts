"use client";

import { useEffect } from "react";
import { useUIStore } from "@/lib/store";

/**
 * Hook to sync theme preference with document class
 * Handles 'system' theme by listening to OS preference changes
 */
export function useTheme() {
  const { theme } = useUIStore();

  useEffect(() => {
    const root = document.documentElement;
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    const applyTheme = () => {
      const isDark =
        theme === "dark" || (theme === "system" && mediaQuery.matches);
      root.classList.toggle("dark", isDark);
    };

    applyTheme();
    mediaQuery.addEventListener("change", applyTheme);
    return () => mediaQuery.removeEventListener("change", applyTheme);
  }, [theme]);
}
