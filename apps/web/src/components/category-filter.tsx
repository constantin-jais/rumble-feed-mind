"use client";

import { X, Tag } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import { useCategories, useFeedCategories } from "@/lib/queries";
import { useUIStore } from "@/lib/store";

interface CategoryFilterProps {
  className?: string;
}

export function CategoryFilter({ className }: CategoryFilterProps) {
  const {
    selectedFeedId,
    selectedCategories,
    toggleCategory,
    clearCategories,
  } = useUIStore();

  // Fetch categories based on context
  const { data: allCategories, isLoading: loadingAll } = useCategories();
  const { data: feedCategories, isLoading: loadingFeed } = useFeedCategories(
    selectedFeedId ?? "",
  );

  const isLoading = selectedFeedId ? loadingFeed : loadingAll;
  const categories = selectedFeedId
    ? feedCategories?.map((fc) => ({
        category: fc.category,
        count: fc.article_count,
      }))
    : allCategories?.map((c) => ({
        category: c.category,
        count: c.article_count,
      }));

  if (isLoading || !categories?.length) {
    return null;
  }

  return (
    <div className={cn("flex flex-wrap items-center gap-2 px-4 py-2 border-b", className)}>
      <Tag className="w-4 h-4 text-muted-foreground" />

      {categories.map(({ category, count }) => {
        const isSelected = selectedCategories.includes(category);
        return (
          <button
            key={category}
            onClick={() => toggleCategory(category)}
            className="focus:outline-none"
          >
            <Badge
              variant={isSelected ? "default" : "outline"}
              className={cn(
                "cursor-pointer transition-all",
                isSelected && "ring-2 ring-primary/20",
              )}
            >
              {category}
              <span className="ml-1 text-[10px] opacity-70">({count})</span>
            </Badge>
          </button>
        );
      })}

      {selectedCategories.length > 0 && (
        <button
          onClick={clearCategories}
          className="ml-2 p-1 rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors"
          title="Clear category filter"
        >
          <X className="w-4 h-4" />
        </button>
      )}
    </div>
  );
}
