"use client";

import { formatDistanceToNow } from "date-fns";
import { Star, ExternalLink, ArrowLeft } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useArticle, useUpdateArticle } from "@/lib/queries";
import { useUIStore } from "@/lib/store";

export function ArticleView() {
  const { selectedArticleId, setSelectedArticle } = useUIStore();
  const { data: article, isLoading } = useArticle(selectedArticleId ?? "");
  const updateArticle = useUpdateArticle();

  if (!selectedArticleId) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        <p>Select an article to read</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        Loading article...
      </div>
    );
  }

  if (!article) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        Article not found
      </div>
    );
  }

  const handleToggleStar = () => {
    updateArticle.mutate({
      id: article.id,
      is_starred: !article.is_starred,
    });
  };

  const handleToggleRead = () => {
    updateArticle.mutate({
      id: article.id,
      is_read: !article.is_read,
    });
  };

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {/* Header */}
      <header className="flex items-center justify-between p-4 border-b border-border">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setSelectedArticle(null)}
          className="gap-1"
        >
          <ArrowLeft className="w-4 h-4" />
          <span className="md:hidden">Back</span>
        </Button>

        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            onClick={handleToggleStar}
            className={cn(article.is_starred && "text-yellow-500")}
          >
            <Star
              className={cn("w-5 h-5", article.is_starred && "fill-current")}
            />
          </Button>
          <Button variant="ghost" size="sm" onClick={handleToggleRead}>
            {article.is_read ? "Mark unread" : "Mark read"}
          </Button>
          {article.url && (
            <Button variant="ghost" size="icon" asChild>
              <a href={article.url} target="_blank" rel="noopener noreferrer">
                <ExternalLink className="w-5 h-5" />
              </a>
            </Button>
          )}
        </div>
      </header>

      {/* Content */}
      <article className="flex-1 overflow-y-auto p-6 max-w-3xl mx-auto">
        {/* Title */}
        <h1 className="text-2xl font-bold mb-4">{article.title}</h1>

        {/* Meta */}
        <div className="flex items-center gap-4 text-sm text-muted-foreground mb-6">
          {article.author && <span>By {article.author}</span>}
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

        {/* Featured image */}
        {article.image_url && (
          <img
            src={article.image_url}
            alt=""
            className="w-full rounded-lg mb-6"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        )}

        {/* Content */}
        {article.content ? (
          <div
            className="prose prose-neutral dark:prose-invert max-w-none"
            dangerouslySetInnerHTML={{ __html: article.content }}
          />
        ) : article.summary ? (
          <div className="prose prose-neutral dark:prose-invert max-w-none">
            <p>{article.summary}</p>
            {article.url && (
              <p>
                <a
                  href={article.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  Read full article →
                </a>
              </p>
            )}
          </div>
        ) : (
          <p className="text-muted-foreground">
            No content available.{" "}
            {article.url && (
              <a
                href={article.url}
                target="_blank"
                rel="noopener noreferrer"
                className="text-primary hover:underline"
              >
                Read on original site →
              </a>
            )}
          </p>
        )}
      </article>
    </div>
  );
}
