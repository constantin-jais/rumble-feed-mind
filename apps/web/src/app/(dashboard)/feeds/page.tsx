"use client";

import { useState, useMemo } from "react";
import {
  Plus,
  Rss,
  MoreHorizontal,
  Pencil,
  Trash2,
  RefreshCw,
  ExternalLink,
  ChevronDown,
  ChevronRight,
  AlertCircle,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import {
  useFeeds,
  useFolders,
  useCreateFeed,
  useUpdateFeed,
  useDeleteFeed,
  useRefreshFeed,
  useCreateFolder,
} from "@/lib/queries";
import type { Feed, Folder as FolderType } from "@/lib/api";

// Group feeds by folder
interface FeedGroup {
  folder: FolderType | null;
  feeds: Feed[];
}

function groupFeedsByFolder(feeds: Feed[], folders: FolderType[]): FeedGroup[] {
  const folderMap = new Map<string, FolderType>();
  folders.forEach((f) => folderMap.set(f.id, f));

  const groups: Map<string | null, Feed[]> = new Map();
  groups.set(null, []); // Uncategorized group

  feeds.forEach((feed) => {
    const folderId = feed.folder_id;
    if (!groups.has(folderId)) {
      groups.set(folderId, []);
    }
    groups.get(folderId)!.push(feed);
  });

  const result: FeedGroup[] = [];

  // Add folder groups first (sorted by name)
  const sortedFolderIds = Array.from(groups.keys())
    .filter((id) => id !== null)
    .sort((a, b) => {
      const folderA = folderMap.get(a!);
      const folderB = folderMap.get(b!);
      return (folderA?.name ?? "").localeCompare(folderB?.name ?? "");
    });

  for (const folderId of sortedFolderIds) {
    const folder = folderMap.get(folderId!);
    if (folder) {
      result.push({ folder, feeds: groups.get(folderId) ?? [] });
    }
  }

  // Add uncategorized at the end
  const uncategorized = groups.get(null) ?? [];
  if (uncategorized.length > 0) {
    result.push({ folder: null, feeds: uncategorized });
  }

  return result;
}

export default function FeedsPage() {
  const { data: feeds = [], isLoading: feedsLoading } = useFeeds();
  const { data: folders = [], isLoading: foldersLoading } = useFolders();

  const [addDialogOpen, setAddDialogOpen] = useState(false);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [selectedFeed, setSelectedFeed] = useState<Feed | null>(null);
  const [collapsedFolders, setCollapsedFolders] = useState<Set<string>>(
    new Set(),
  );

  const feedGroups = useMemo(
    () => groupFeedsByFolder(feeds, folders),
    [feeds, folders],
  );

  const isLoading = feedsLoading || foldersLoading;

  const toggleFolder = (folderId: string) => {
    setCollapsedFolders((prev) => {
      const next = new Set(prev);
      if (next.has(folderId)) {
        next.delete(folderId);
      } else {
        next.add(folderId);
      }
      return next;
    });
  };

  const handleEditClick = (feed: Feed) => {
    setSelectedFeed(feed);
    setEditDialogOpen(true);
  };

  const handleDeleteClick = (feed: Feed) => {
    setSelectedFeed(feed);
    setDeleteDialogOpen(true);
  };

  const totalFeeds = feeds.length;
  const totalArticles = feeds.reduce((acc, f) => acc + f.article_count, 0);
  const totalUnread = feeds.reduce((acc, f) => acc + f.unread_count, 0);

  return (
    <div className="flex-1 overflow-auto">
      <div className="max-w-4xl mx-auto p-6 space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold">Feeds</h1>
            <p className="text-muted-foreground">
              Manage your RSS subscriptions
            </p>
          </div>
          <Button onClick={() => setAddDialogOpen(true)}>
            <Plus className="w-4 h-4 mr-2" />
            Add Feed
          </Button>
        </div>

        {/* Stats Cards */}
        <div className="grid grid-cols-3 gap-4">
          <Card className="py-4">
            <CardContent className="pt-0">
              <div className="text-2xl font-bold">{totalFeeds}</div>
              <p className="text-xs text-muted-foreground">Total Feeds</p>
            </CardContent>
          </Card>
          <Card className="py-4">
            <CardContent className="pt-0">
              <div className="text-2xl font-bold">{totalArticles}</div>
              <p className="text-xs text-muted-foreground">Total Articles</p>
            </CardContent>
          </Card>
          <Card className="py-4">
            <CardContent className="pt-0">
              <div className="text-2xl font-bold">{totalUnread}</div>
              <p className="text-xs text-muted-foreground">Unread Articles</p>
            </CardContent>
          </Card>
        </div>

        {/* Feed Groups */}
        {isLoading ? (
          <div className="text-center py-12 text-muted-foreground">
            Loading feeds...
          </div>
        ) : feeds.length === 0 ? (
          <Card>
            <CardContent className="py-12 text-center">
              <Rss className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
              <h3 className="text-lg font-medium mb-2">No feeds yet</h3>
              <p className="text-muted-foreground mb-4">
                Add your first RSS feed to get started
              </p>
              <Button onClick={() => setAddDialogOpen(true)}>
                <Plus className="w-4 h-4 mr-2" />
                Add Feed
              </Button>
            </CardContent>
          </Card>
        ) : (
          <div className="space-y-4">
            {feedGroups.map((group) => (
              <FeedGroupCard
                key={group.folder?.id ?? "uncategorized"}
                group={group}
                isCollapsed={
                  group.folder ? collapsedFolders.has(group.folder.id) : false
                }
                onToggle={() => group.folder && toggleFolder(group.folder.id)}
                onEdit={handleEditClick}
                onDelete={handleDeleteClick}
              />
            ))}
          </div>
        )}

        {/* Add Feed Dialog */}
        <AddFeedDialog
          open={addDialogOpen}
          onOpenChange={setAddDialogOpen}
          folders={folders}
        />

        {/* Edit Feed Dialog */}
        {selectedFeed && (
          <EditFeedDialog
            open={editDialogOpen}
            onOpenChange={setEditDialogOpen}
            feed={selectedFeed}
            folders={folders}
          />
        )}

        {/* Delete Feed Dialog */}
        {selectedFeed && (
          <DeleteFeedDialog
            open={deleteDialogOpen}
            onOpenChange={setDeleteDialogOpen}
            feed={selectedFeed}
          />
        )}
      </div>
    </div>
  );
}

// Feed Group Card Component
function FeedGroupCard({
  group,
  isCollapsed,
  onToggle,
  onEdit,
  onDelete,
}: {
  group: FeedGroup;
  isCollapsed: boolean;
  onToggle: () => void;
  onEdit: (feed: Feed) => void;
  onDelete: (feed: Feed) => void;
}) {
  const isUncategorized = !group.folder;
  const feedCount = group.feeds.length;
  const unreadCount = group.feeds.reduce((acc, f) => acc + f.unread_count, 0);

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <button
            onClick={onToggle}
            className="flex items-center gap-2 hover:text-foreground text-left"
            disabled={isUncategorized}
          >
            {isUncategorized ? (
              <Rss className="w-4 h-4 text-muted-foreground" />
            ) : isCollapsed ? (
              <ChevronRight className="w-4 h-4" />
            ) : (
              <ChevronDown className="w-4 h-4" />
            )}
            <CardTitle className="text-base">
              {isUncategorized ? "Uncategorized" : group.folder!.name}
            </CardTitle>
            <Badge variant="secondary" className="ml-2">
              {feedCount} {feedCount === 1 ? "feed" : "feeds"}
            </Badge>
            {unreadCount > 0 && (
              <Badge variant="default" className="ml-1">
                {unreadCount} unread
              </Badge>
            )}
          </button>
        </div>
      </CardHeader>
      {(!isCollapsed || isUncategorized) && (
        <CardContent className="pt-0">
          <div className="divide-y">
            {group.feeds.map((feed) => (
              <FeedRow
                key={feed.id}
                feed={feed}
                onEdit={() => onEdit(feed)}
                onDelete={() => onDelete(feed)}
              />
            ))}
          </div>
        </CardContent>
      )}
    </Card>
  );
}

// Feed Row Component
function FeedRow({
  feed,
  onEdit,
  onDelete,
}: {
  feed: Feed;
  onEdit: () => void;
  onDelete: () => void;
}) {
  const refreshFeed = useRefreshFeed();

  const handleRefresh = () => {
    refreshFeed.mutate(feed.id);
  };

  const lastFetched = feed.last_fetched_at
    ? new Date(feed.last_fetched_at).toLocaleDateString("fr-FR", {
        day: "numeric",
        month: "short",
        hour: "2-digit",
        minute: "2-digit",
      })
    : "Never";

  return (
    <div className="flex items-center justify-between py-3 group">
      <div className="flex items-center gap-3 min-w-0 flex-1">
        {feed.icon_url ? (
          <img
            src={feed.icon_url}
            alt=""
            className="w-6 h-6 rounded flex-shrink-0"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = "none";
            }}
          />
        ) : (
          <Rss className="w-6 h-6 text-muted-foreground flex-shrink-0" />
        )}
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <span className="font-medium truncate">{feed.title}</span>
            {feed.error_count > 0 && (
              <span title={feed.last_error ?? "Error fetching feed"}>
                <AlertCircle className="w-4 h-4 text-destructive flex-shrink-0" />
              </span>
            )}
          </div>
          <div className="text-xs text-muted-foreground flex items-center gap-2">
            <span>
              {feed.article_count} articles
              {feed.unread_count > 0 && ` (${feed.unread_count} unread)`}
            </span>
            <span>-</span>
            <span>Last fetched: {lastFetched}</span>
          </div>
        </div>
      </div>
      <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <Button
          variant="ghost"
          size="icon"
          onClick={handleRefresh}
          disabled={refreshFeed.isPending}
          title="Refresh feed"
        >
          <RefreshCw
            className={cn("w-4 h-4", refreshFeed.isPending && "animate-spin")}
          />
        </Button>
        {feed.site_url && (
          <Button variant="ghost" size="icon" asChild title="Visit site">
            <a href={feed.site_url} target="_blank" rel="noopener noreferrer">
              <ExternalLink className="w-4 h-4" />
            </a>
          </Button>
        )}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon">
              <MoreHorizontal className="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={onEdit}>
              <Pencil className="w-4 h-4 mr-2" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              onClick={onDelete}
              className="text-destructive focus:text-destructive"
            >
              <Trash2 className="w-4 h-4 mr-2" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
}

// Add Feed Dialog
function AddFeedDialog({
  open,
  onOpenChange,
  folders,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  folders: FolderType[];
}) {
  const [url, setUrl] = useState("");
  const [folderId, setFolderId] = useState<string>("");
  const [newFolderName, setNewFolderName] = useState("");
  const [showNewFolder, setShowNewFolder] = useState(false);

  const createFeed = useCreateFeed();
  const createFolder = useCreateFolder();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    let targetFolderId = folderId === "none" ? undefined : folderId;

    // Create new folder if needed
    if (showNewFolder && newFolderName.trim()) {
      try {
        const newFolder = await createFolder.mutateAsync({
          name: newFolderName.trim(),
        });
        targetFolderId = newFolder.id;
      } catch {
        return; // Stop if folder creation fails
      }
    }

    try {
      await createFeed.mutateAsync({
        url: url.trim(),
        folderId: targetFolderId,
      });
      // Reset form and close
      setUrl("");
      setFolderId("");
      setNewFolderName("");
      setShowNewFolder(false);
      onOpenChange(false);
    } catch {
      // Error is handled by mutation
    }
  };

  const isLoading = createFeed.isPending || createFolder.isPending;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add Feed</DialogTitle>
          <DialogDescription>
            Enter the URL of an RSS or Atom feed to subscribe.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="url">Feed URL</Label>
            <Input
              id="url"
              type="url"
              placeholder="https://example.com/feed.xml"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="folder">Folder (optional)</Label>
            {!showNewFolder ? (
              <Select value={folderId} onValueChange={setFolderId}>
                <SelectTrigger>
                  <SelectValue placeholder="Select a folder" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="none">No folder</SelectItem>
                  {folders.map((folder) => (
                    <SelectItem key={folder.id} value={folder.id}>
                      {folder.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ) : (
              <Input
                placeholder="New folder name"
                value={newFolderName}
                onChange={(e) => setNewFolderName(e.target.value)}
              />
            )}
            <Button
              type="button"
              variant="link"
              className="px-0 h-auto text-xs"
              onClick={() => {
                setShowNewFolder(!showNewFolder);
                if (!showNewFolder) {
                  setFolderId("");
                } else {
                  setNewFolderName("");
                }
              }}
            >
              {showNewFolder ? "Select existing folder" : "Create new folder"}
            </Button>
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isLoading || !url.trim()}>
              {isLoading ? "Adding..." : "Add Feed"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

// Edit Feed Dialog
function EditFeedDialog({
  open,
  onOpenChange,
  feed,
  folders,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  feed: Feed;
  folders: FolderType[];
}) {
  const [title, setTitle] = useState(feed.title);
  const [folderId, setFolderId] = useState<string>(feed.folder_id ?? "none");

  const updateFeed = useUpdateFeed();

  // Reset form when feed changes
  useState(() => {
    setTitle(feed.title);
    setFolderId(feed.folder_id ?? "none");
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      await updateFeed.mutateAsync({
        id: feed.id,
        title: title.trim(),
        folder_id: folderId === "none" ? undefined : folderId,
      });
      onOpenChange(false);
    } catch {
      // Error is handled by mutation
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Feed</DialogTitle>
          <DialogDescription>
            Update the title or folder for this feed.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="edit-title">Title</Label>
            <Input
              id="edit-title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="edit-folder">Folder</Label>
            <Select value={folderId} onValueChange={setFolderId}>
              <SelectTrigger>
                <SelectValue placeholder="Select a folder" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">No folder</SelectItem>
                {folders.map((folder) => (
                  <SelectItem key={folder.id} value={folder.id}>
                    {folder.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="text-xs text-muted-foreground">
            Feed URL: {feed.url}
          </div>
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={updateFeed.isPending || !title.trim()}
            >
              {updateFeed.isPending ? "Saving..." : "Save Changes"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

// Delete Feed Dialog
function DeleteFeedDialog({
  open,
  onOpenChange,
  feed,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  feed: Feed;
}) {
  const deleteFeed = useDeleteFeed();

  const handleDelete = async () => {
    try {
      await deleteFeed.mutateAsync(feed.id);
      onOpenChange(false);
    } catch {
      // Error is handled by mutation
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Feed</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete &ldquo;{feed.title}&rdquo;? This
            will also delete all {feed.article_count} articles from this feed.
            This action cannot be undone.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button
            variant="destructive"
            onClick={handleDelete}
            disabled={deleteFeed.isPending}
          >
            {deleteFeed.isPending ? "Deleting..." : "Delete Feed"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
