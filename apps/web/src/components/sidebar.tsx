"use client";

import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  ChevronDown,
  ChevronRight,
  Folder,
  Rss,
  Star,
  Settings,
  Plus,
  RefreshCw,
  LogOut,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useFeeds, useLogout } from "@/lib/queries";
import { useUIStore, useAuthStore } from "@/lib/store";
import type { Feed } from "@/lib/api";

export function Sidebar() {
  const pathname = usePathname();
  const { data: feeds, isLoading } = useFeeds();
  const logout = useLogout();
  const { user } = useAuthStore();
  const { selectedFeedId, setSelectedFeed, articleFilter, setArticleFilter } = useUIStore();
  const [showAddFeed, setShowAddFeed] = useState(false);

  const totalUnread = feeds?.reduce((acc, feed) => acc + feed.unread_count, 0) ?? 0;

  return (
    <aside className="w-64 h-screen bg-sidebar border-r border-sidebar-border flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-sidebar-border">
        <Link href="/" className="flex items-center gap-2">
          <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
            <Rss className="w-5 h-5 text-primary-foreground" />
          </div>
          <span className="font-semibold text-lg">FeedMind</span>
        </Link>
      </div>

      {/* Quick filters */}
      <div className="p-2 space-y-1">
        <button
          onClick={() => {
            setSelectedFeed(null);
            setArticleFilter("unread");
          }}
          className={cn(
            "w-full flex items-center justify-between px-3 py-2 rounded-md text-sm",
            articleFilter === "unread" && !selectedFeedId
              ? "bg-sidebar-accent text-sidebar-accent-foreground"
              : "text-sidebar-foreground hover:bg-sidebar-accent/50"
          )}
        >
          <span>Unread</span>
          {totalUnread > 0 && (
            <span className="text-xs bg-primary text-primary-foreground px-2 py-0.5 rounded-full">
              {totalUnread}
            </span>
          )}
        </button>
        <button
          onClick={() => {
            setSelectedFeed(null);
            setArticleFilter("starred");
          }}
          className={cn(
            "w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm",
            articleFilter === "starred" && !selectedFeedId
              ? "bg-sidebar-accent text-sidebar-accent-foreground"
              : "text-sidebar-foreground hover:bg-sidebar-accent/50"
          )}
        >
          <Star className="w-4 h-4" />
          <span>Starred</span>
        </button>
        <button
          onClick={() => {
            setSelectedFeed(null);
            setArticleFilter("all");
          }}
          className={cn(
            "w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm",
            articleFilter === "all" && !selectedFeedId
              ? "bg-sidebar-accent text-sidebar-accent-foreground"
              : "text-sidebar-foreground hover:bg-sidebar-accent/50"
          )}
        >
          <span>All Articles</span>
        </button>
      </div>

      {/* Feeds list */}
      <div className="flex-1 overflow-y-auto p-2">
        <div className="flex items-center justify-between px-3 py-2">
          <span className="text-xs font-medium text-muted-foreground uppercase">
            Feeds
          </span>
          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6"
            onClick={() => setShowAddFeed(true)}
          >
            <Plus className="w-4 h-4" />
          </Button>
        </div>

        {isLoading ? (
          <div className="px-3 py-2 text-sm text-muted-foreground">
            Loading feeds...
          </div>
        ) : feeds?.length === 0 ? (
          <div className="px-3 py-2 text-sm text-muted-foreground">
            No feeds yet. Add your first feed!
          </div>
        ) : (
          <div className="space-y-1">
            {feeds?.map((feed) => (
              <FeedItem
                key={feed.id}
                feed={feed}
                isSelected={selectedFeedId === feed.id}
                onClick={() => setSelectedFeed(feed.id)}
              />
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="p-2 border-t border-sidebar-border space-y-1">
        <Link
          href="/settings"
          className={cn(
            "flex items-center gap-2 px-3 py-2 rounded-md text-sm",
            pathname === "/settings"
              ? "bg-sidebar-accent text-sidebar-accent-foreground"
              : "text-sidebar-foreground hover:bg-sidebar-accent/50"
          )}
        >
          <Settings className="w-4 h-4" />
          <span>Settings</span>
        </Link>
        <button
          onClick={() => logout.mutate()}
          className="w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm text-sidebar-foreground hover:bg-sidebar-accent/50"
        >
          <LogOut className="w-4 h-4" />
          <span>Log out</span>
        </button>
        {user && (
          <div className="px-3 py-2 text-xs text-muted-foreground truncate">
            {user.email}
          </div>
        )}
      </div>
    </aside>
  );
}

function FeedItem({
  feed,
  isSelected,
  onClick,
}: {
  feed: Feed;
  isSelected: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "w-full flex items-center justify-between px-3 py-2 rounded-md text-sm",
        isSelected
          ? "bg-sidebar-accent text-sidebar-accent-foreground"
          : "text-sidebar-foreground hover:bg-sidebar-accent/50"
      )}
    >
      <div className="flex items-center gap-2 min-w-0">
        {feed.icon_url ? (
          <img
            src={feed.icon_url}
            alt=""
            className="w-4 h-4 rounded"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        ) : (
          <Rss className="w-4 h-4 text-muted-foreground" />
        )}
        <span className="truncate">{feed.title}</span>
      </div>
      {feed.unread_count > 0 && (
        <span className="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded">
          {feed.unread_count}
        </span>
      )}
    </button>
  );
}
