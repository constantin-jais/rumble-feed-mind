"use client";

import { List, LayoutGrid, Newspaper } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useUIStore } from "@/lib/store";

export function ViewModeToggle() {
  const { viewMode, setViewMode } = useUIStore();

  return (
    <div className="flex items-center rounded-md border bg-muted p-0.5">
      <Button
        variant="ghost"
        size="icon"
        className={cn(
          "h-7 w-7 rounded-sm",
          viewMode === "list" && "bg-background shadow-sm"
        )}
        onClick={() => setViewMode("list")}
        title="List view"
      >
        <List className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className={cn(
          "h-7 w-7 rounded-sm",
          viewMode === "card" && "bg-background shadow-sm"
        )}
        onClick={() => setViewMode("card")}
        title="Card view"
      >
        <LayoutGrid className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className={cn(
          "h-7 w-7 rounded-sm",
          viewMode === "magazine" && "bg-background shadow-sm"
        )}
        onClick={() => setViewMode("magazine")}
        title="Magazine view"
      >
        <Newspaper className="h-4 w-4" />
      </Button>
    </div>
  );
}
