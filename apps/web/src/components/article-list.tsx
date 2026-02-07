"use client";

import { formatDistanceToNow } from "date-fns";
import { Star, Circle, ExternalLink } from "lucide-react";
import { cn } from "@/lib/utils";
import { useArticles, useUpdateArticle } from "@/lib/queries";
import { useUIStore } from "@/lib/store";
import type { Article } from "@/lib/api";

export function ArticleList() {
  const { selectedFeedId, selectedFolderId, articleFilter, selectedArticleId, setSelectedArticle } =
    useUIStore();

  const { data, isLoading, error } = useArticles({
    feed_id: selectedFeedId ?? undefined,
    folder_id: selectedFolderId ?? undefined,
    status: articleFilter === "all" ? undefined : articleFilter,
  });

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        Loading articles...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex-1 flex items-center justify-center text-destructive">
        Failed to load articles
      </div>
    );
  }

  if (!data?.data.length) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-muted-foreground">
        <p className="text-lg">No articles</p>
        <p className="text-sm">
          {articleFilter === "unread"
            ? "You're all caught up!"
            : "No articles match your filter"}
        </p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto">
      {data.data.map((article) => (
        <ArticleItem
          key={article.id}
          article={article}
          isSelected={selectedArticleId === article.id}
          onClick={() => setSelectedArticle(article.id)}
        />
      ))}
    </div>
  );
}

function ArticleItem({
  article,
  isSelected,
  onClick,
}: {
  article: Article;
  isSelected: boolean;
  onClick: () => void;
}) {
  const updateArticle = useUpdateArticle();

  const handleToggleStar = (e: React.MouseEvent) => {
    e.stopPropagation();
    updateArticle.mutate({
      id: article.id,
      is_starred: !article.is_starred,
    });
  };

  const handleMarkRead = () => {
    if (!article.is_read) {
      updateArticle.mutate({
        id: article.id,
        is_read: true,
      });
    }
    onClick();
  };

  return (
    <article
      onClick={handleMarkRead}
      className={cn(
        "p-4 border-b border-border cursor-pointer transition-colors",
        isSelected ? "bg-accent" : "hover:bg-muted/50",
        !article.is_read && "bg-primary/5"
      )}
    >
      <div className="flex items-start gap-3">
        {/* Unread indicator */}
        <div className="pt-1.5">
          {!article.is_read ? (
            <Circle className="w-2 h-2 fill-primary text-primary" />
          ) : (
            <div className="w-2 h-2" />
          )}
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          {/* Feed title */}
          {article.feed_title && (
            <p className="text-xs text-muted-foreground mb-1">
              {article.feed_title}
            </p>
          )}

          {/* Article title */}
          <h3
            className={cn(
              "text-sm leading-snug mb-1 line-clamp-2",
              !article.is_read ? "font-medium" : "font-normal"
            )}
          >
            {article.title}
          </h3>

          {/* Summary */}
          {article.summary && (
            <p className="text-xs text-muted-foreground line-clamp-2 mb-2">
              {article.summary}
            </p>
          )}

          {/* Meta */}
          <div className="flex items-center gap-3 text-xs text-muted-foreground">
            {article.author && <span>{article.author}</span>}
            {article.published_at && (
              <span>
                {formatDistanceToNow(new Date(article.published_at), {
                  addSuffix: true,
                })}
              </span>
            )}
            {article.word_count && (
              <span>{Math.ceil(article.word_count / 200)} min read</span>
            )}
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-1">
          <button
            onClick={handleToggleStar}
            className={cn(
              "p-1 rounded hover:bg-muted",
              article.is_starred ? "text-yellow-500" : "text-muted-foreground"
            )}
          >
            <Star
              className={cn("w-4 h-4", article.is_starred && "fill-current")}
            />
          </button>
          {article.url && (
            <a
              href={article.url}
              target="_blank"
              rel="noopener noreferrer"
              onClick={(e) => e.stopPropagation()}
              className="p-1 rounded text-muted-foreground hover:bg-muted"
            >
              <ExternalLink className="w-4 h-4" />
            </a>
          )}
        </div>

        {/* Thumbnail */}
        {article.image_url && (
          <img
            src={article.image_url}
            alt=""
            className="w-20 h-14 object-cover rounded"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        )}
      </div>
    </article>
  );
}
