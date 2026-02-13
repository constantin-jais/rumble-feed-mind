"use client";

import { formatDistanceToNow } from "date-fns";
import { Star, Circle, ExternalLink } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import { useArticles, useUpdateArticle } from "@/lib/queries";
import { useUIStore } from "@/lib/store";
import { ArticleCard } from "@/components/article-card";
import type { Article } from "@/lib/api";

export function ArticleList() {
  const {
    selectedFeedId,
    selectedFolderId,
    articleFilter,
    selectedArticleId,
    setSelectedArticle,
    selectedCategories,
    viewMode,
  } = useUIStore();

  const { data, isLoading, error } = useArticles({
    feed_id: selectedFeedId ?? undefined,
    folder_id: selectedFolderId ?? undefined,
    status: articleFilter === "all" ? undefined : articleFilter,
    categories: selectedCategories.length > 0 ? selectedCategories : undefined,
  });

  const updateArticle = useUpdateArticle();

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

  // Card view
  if (viewMode === "card") {
    return (
      <div className="flex-1 overflow-y-auto p-4">
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {data.data.map((article) => (
            <ArticleCard
              key={article.id}
              article={article}
              isSelected={selectedArticleId === article.id}
              onSelect={() => {
                if (!article.is_read) {
                  updateArticle.mutate({ id: article.id, is_read: true });
                }
                setSelectedArticle(article.id);
              }}
              onToggleRead={() => {
                updateArticle.mutate({
                  id: article.id,
                  is_read: !article.is_read,
                });
              }}
              onToggleStar={() => {
                updateArticle.mutate({
                  id: article.id,
                  is_starred: !article.is_starred,
                });
              }}
            />
          ))}
        </div>
      </div>
    );
  }

  // List view (default)
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

          {/* Categories */}
          {article.categories && article.categories.length > 0 && (
            <div className="flex flex-wrap gap-1 mb-2">
              {article.categories.slice(0, 3).map((category) => (
                <Badge
                  key={category}
                  variant="secondary"
                  className="text-[10px] px-1.5 py-0"
                >
                  {category}
                </Badge>
              ))}
              {article.categories.length > 3 && (
                <span className="text-[10px] text-muted-foreground">
                  +{article.categories.length - 3}
                </span>
              )}
            </div>
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
