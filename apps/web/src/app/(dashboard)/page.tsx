"use client";

import { ArticleList } from "@/components/article-list";
import { ArticleView } from "@/components/article-view";
import { CategoryFilter } from "@/components/category-filter";
import { useUIStore } from "@/lib/store";
import { Button } from "@/components/ui/button";
import { CheckCheck } from "lucide-react";
import { useMarkAllRead } from "@/lib/queries";
import { cn } from "@/lib/utils";

type ArticleFilter = "all" | "unread" | "starred";

const FILTER_OPTIONS: { value: ArticleFilter; label: string }[] = [
  { value: "all", label: "All" },
  { value: "unread", label: "Unread" },
  { value: "starred", label: "Starred" },
];

export default function DashboardPage() {
  const { selectedFeedId, articleFilter, setArticleFilter, selectedArticleId } =
    useUIStore();
  const markAllRead = useMarkAllRead();

  const handleMarkAllRead = () => {
    markAllRead.mutate({ feed_id: selectedFeedId ?? undefined });
  };

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* Article list panel */}
      <div
        className={cn(
          "w-full md:w-80 lg:w-96 border-r border-border flex flex-col shrink-0",
          selectedArticleId && "hidden md:flex",
        )}
      >
        {/* Header with filter buttons */}
        <header className="flex items-center justify-between p-3 border-b border-border gap-2">
          <div className="flex items-center gap-1 bg-muted rounded-lg p-1">
            {FILTER_OPTIONS.map((option) => (
              <button
                key={option.value}
                onClick={() => setArticleFilter(option.value)}
                className={cn(
                  "px-3 py-1 text-sm rounded-md transition-colors",
                  articleFilter === option.value
                    ? "bg-background text-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                {option.label}
              </button>
            ))}
          </div>
          <Button
            variant="ghost"
            size="icon"
            onClick={handleMarkAllRead}
            disabled={markAllRead.isPending}
            title="Mark all as read"
            className="shrink-0"
          >
            <CheckCheck className="w-4 h-4" />
          </Button>
        </header>

        <CategoryFilter />
        <ArticleList />
      </div>

      {/* Article view panel */}
      <div
        className={cn(
          "flex-1 flex flex-col min-w-0",
          !selectedArticleId && "hidden md:flex",
        )}
      >
        <ArticleView />
      </div>
    </div>
  );
}
