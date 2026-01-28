"use client";

import { X } from "lucide-react";
import { cn } from "@/lib/utils";

interface TagBadgeProps {
  name: string;
  color?: string | null;
  count?: number;
  selected?: boolean;
  removable?: boolean;
  onClick?: () => void;
  onRemove?: () => void;
  size?: "sm" | "md";
}

/**
 * Get contrasting text color for a background color
 */
function getContrastColor(hexColor: string): string {
  // Remove # if present
  const hex = hexColor.replace("#", "");

  // Convert to RGB
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);

  // Calculate luminance
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

  // Return black or white based on luminance
  return luminance > 0.5 ? "#000000" : "#FFFFFF";
}

export function TagBadge({
  name,
  color,
  count,
  selected = false,
  removable = false,
  onClick,
  onRemove,
  size = "md",
}: TagBadgeProps) {
  const hasCustomColor = !!color;
  const bgColor = color || undefined;
  const textColor = color ? getContrastColor(color) : undefined;

  return (
    <span
      className={cn(
        "inline-flex items-center gap-1 rounded-full font-medium transition-colors",
        size === "sm" ? "px-2 py-0.5 text-xs" : "px-2.5 py-1 text-sm",
        hasCustomColor
          ? ""
          : selected
            ? "bg-primary text-primary-foreground"
            : "bg-muted text-muted-foreground hover:bg-muted/80",
        onClick && "cursor-pointer"
      )}
      style={
        hasCustomColor
          ? {
              backgroundColor: bgColor,
              color: textColor,
            }
          : undefined
      }
      onClick={onClick}
    >
      {name}
      {count !== undefined && (
        <span
          className={cn(
            "rounded-full px-1.5 text-xs",
            hasCustomColor
              ? "bg-white/20"
              : selected
                ? "bg-primary-foreground/20 text-primary-foreground"
                : "bg-background/50"
          )}
        >
          {count}
        </span>
      )}
      {removable && onRemove && (
        <button
          type="button"
          className={cn(
            "ml-0.5 rounded-full p-0.5 hover:bg-black/10",
            size === "sm" ? "-mr-1" : "-mr-0.5"
          )}
          onClick={(e) => {
            e.stopPropagation();
            onRemove();
          }}
        >
          <X className={size === "sm" ? "h-3 w-3" : "h-3.5 w-3.5"} />
        </button>
      )}
    </span>
  );
}
