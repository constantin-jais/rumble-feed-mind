"use client";

import { Star, Check, ExternalLink } from "lucide-react";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { Article } from "@/lib/api";

interface ArticleCardProps {
  article: Article;
  isSelected: boolean;
  onSelect: () => void;
  onToggleRead: () => void;
  onToggleStar: () => void;
}

/**
 * Extract first image URL from HTML content
 */
function extractFirstImage(content: string | null | undefined): string | null {
  if (!content) return null;

  // Try to find img tag
  const imgMatch = content.match(/<img[^>]+src="([^"]+)"/i);
  if (imgMatch) {
    return imgMatch[1];
  }

  // Try to find image URL pattern
  const urlMatch = content.match(/https?:\/\/[^\s"'<>]+\.(jpg|jpeg|png|gif|webp)/i);
  if (urlMatch) {
    return urlMatch[0];
  }

  return null;
}

/**
 * Strip HTML tags from content
 */
function stripHtml(html: string | null | undefined): string {
  if (!html) return "";
  return html
    .replace(/<[^>]*>/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

/**
 * Format relative date
 */
function formatRelativeDate(dateStr: string | null): string {
  if (!dateStr) return "";

  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 60) {
    return `${diffMins}m`;
  } else if (diffHours < 24) {
    return `${diffHours}h`;
  } else if (diffDays < 7) {
    return `${diffDays}d`;
  } else {
    return date.toLocaleDateString("en-US", { month: "short", day: "numeric" });
  }
}

export function ArticleCard({
  article,
  isSelected,
  onSelect,
  onToggleRead,
  onToggleStar,
}: ArticleCardProps) {
  const thumbnail = article.image_url || extractFirstImage(article.content);
  const summary = article.summary || stripHtml(article.content)?.slice(0, 150);

  return (
    <Card
      className={cn(
        "cursor-pointer transition-all hover:shadow-md",
        isSelected && "ring-2 ring-primary",
        article.is_read && "opacity-60"
      )}
      onClick={onSelect}
    >
      {thumbnail && (
        <div className="aspect-video overflow-hidden rounded-t-lg bg-muted">
          <img
            src={thumbnail}
            alt=""
            className="h-full w-full object-cover"
            loading="lazy"
            onError={(e) => {
              (e.target as HTMLImageElement).parentElement!.style.display = "none";
            }}
          />
        </div>
      )}
      <CardHeader className="pb-2 pt-3 px-3">
        <h3
          className={cn(
            "line-clamp-2 text-sm font-medium leading-tight",
            article.is_read && "text-muted-foreground"
          )}
        >
          {article.title}
        </h3>
      </CardHeader>
      <CardContent className="px-3 pb-2">
        {summary && (
          <p className="line-clamp-2 text-xs text-muted-foreground">{summary}</p>
        )}
      </CardContent>
      <CardFooter className="flex items-center justify-between px-3 pb-3 pt-0">
        <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
          {article.feed_title && (
            <span className="truncate max-w-[100px]">{article.feed_title}</span>
          )}
          {article.feed_title && article.published_at && (
            <span className="text-muted-foreground/50">·</span>
          )}
          <span>{formatRelativeDate(article.published_at)}</span>
        </div>
        <div className="flex items-center gap-1" onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="icon"
            className="h-7 w-7"
            onClick={onToggleStar}
            title={article.is_starred ? "Unstar" : "Star"}
          >
            <Star
              className={cn(
                "h-4 w-4",
                article.is_starred
                  ? "fill-yellow-400 text-yellow-400"
                  : "text-muted-foreground"
              )}
            />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            className="h-7 w-7"
            onClick={onToggleRead}
            title={article.is_read ? "Mark as unread" : "Mark as read"}
          >
            <Check
              className={cn(
                "h-4 w-4",
                article.is_read ? "text-primary" : "text-muted-foreground"
              )}
            />
          </Button>
          {article.url && (
            <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7"
              onClick={(e) => {
                e.stopPropagation();
                window.open(article.url!, "_blank");
              }}
              title="Open in new tab"
            >
              <ExternalLink className="h-4 w-4 text-muted-foreground" />
            </Button>
          )}
        </div>
      </CardFooter>
    </Card>
  );
}
