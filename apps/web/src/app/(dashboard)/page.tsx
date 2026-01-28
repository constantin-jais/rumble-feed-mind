"use client";

import { ArticleList } from "@/components/article-list";
import { ArticleView } from "@/components/article-view";
import { useUIStore } from "@/lib/store";
import { Button } from "@/components/ui/button";
import { RefreshCw, CheckCheck } from "lucide-react";
import { useMarkAllRead } from "@/lib/queries";

export default function DashboardPage() {
  const { selectedArticleId, selectedFeedId, articleFilter } = useUIStore();
  const markAllRead = useMarkAllRead();

  const handleMarkAllRead = () => {
    markAllRead.mutate({ feed_id: selectedFeedId ?? undefined });
  };

  return (
    <>
      {/* Article list panel */}
      <div className="w-96 border-r border-border flex flex-col">
        {/* Header */}
        <header className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="font-semibold">
            {articleFilter === "unread"
              ? "Unread"
              : articleFilter === "starred"
              ? "Starred"
              : "All Articles"}
          </h2>
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              onClick={handleMarkAllRead}
              disabled={markAllRead.isPending}
              title="Mark all as read"
            >
              <CheckCheck className="w-4 h-4" />
            </Button>
          </div>
        </header>

        <ArticleList />
      </div>

      {/* Article view panel */}
      <ArticleView />
    </>
  );
}
