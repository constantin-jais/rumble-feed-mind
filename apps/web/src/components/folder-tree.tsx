"use client";

import { useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  Folder,
  FolderOpen,
  MoreHorizontal,
  Plus,
  Pencil,
  Trash2,
  Rss,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";
import { useFolders, useFeeds, useCreateFolder, useUpdateFolder, useDeleteFolder } from "@/lib/queries";
import { useUIStore } from "@/lib/store";
import type { Feed } from "@/lib/api";

interface FolderWithStats {
  id: string;
  name: string;
  parent_id: string | null;
  position: number;
  feed_count: number;
  unread_count: number;
}

interface FolderNodeProps {
  folder: FolderWithStats;
  folders: FolderWithStats[];
  feeds: Feed[];
  level: number;
  selectedFolderId: string | null;
  selectedFeedId: string | null;
  onSelectFolder: (id: string | null) => void;
  onSelectFeed: (id: string | null) => void;
  onEditFolder: (folder: FolderWithStats) => void;
  onDeleteFolder: (id: string) => void;
  onCreateSubfolder: (parentId: string) => void;
}

function FolderNode({
  folder,
  folders,
  feeds,
  level,
  selectedFolderId,
  selectedFeedId,
  onSelectFolder,
  onSelectFeed,
  onEditFolder,
  onDeleteFolder,
  onCreateSubfolder,
}: FolderNodeProps) {
  const [isExpanded, setIsExpanded] = useState(true);

  const childFolders = folders.filter((f) => f.parent_id === folder.id);
  const folderFeeds = feeds.filter((f) => f.folder_id === folder.id);
  const hasChildren = childFolders.length > 0 || folderFeeds.length > 0;
  const isSelected = selectedFolderId === folder.id;

  return (
    <div>
      <div
        className={cn(
          "group flex items-center gap-1 py-1.5 px-2 rounded-md cursor-pointer",
          isSelected
            ? "bg-sidebar-accent text-sidebar-accent-foreground"
            : "hover:bg-sidebar-accent/50"
        )}
        style={{ paddingLeft: `${level * 12 + 8}px` }}
      >
        <button
          className="p-0.5 hover:bg-black/10 rounded"
          onClick={(e) => {
            e.stopPropagation();
            setIsExpanded(!isExpanded);
          }}
        >
          {hasChildren ? (
            isExpanded ? (
              <ChevronDown className="h-4 w-4" />
            ) : (
              <ChevronRight className="h-4 w-4" />
            )
          ) : (
            <div className="w-4" />
          )}
        </button>

        <button
          className="flex-1 flex items-center gap-2 min-w-0"
          onClick={() => onSelectFolder(folder.id)}
        >
          {isExpanded ? (
            <FolderOpen className="h-4 w-4 text-muted-foreground shrink-0" />
          ) : (
            <Folder className="h-4 w-4 text-muted-foreground shrink-0" />
          )}
          <span className="truncate text-sm">{folder.name}</span>
          {folder.unread_count > 0 && (
            <span className="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded ml-auto">
              {folder.unread_count}
            </span>
          )}
        </button>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className="h-6 w-6 opacity-0 group-hover:opacity-100"
            >
              <MoreHorizontal className="h-4 w-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem onClick={() => onCreateSubfolder(folder.id)}>
              <Plus className="mr-2 h-4 w-4" />
              New Subfolder
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onEditFolder(folder)}>
              <Pencil className="mr-2 h-4 w-4" />
              Rename
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="text-destructive"
              onClick={() => onDeleteFolder(folder.id)}
            >
              <Trash2 className="mr-2 h-4 w-4" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      {isExpanded && (
        <div>
          {/* Child folders */}
          {childFolders.map((child) => (
            <FolderNode
              key={child.id}
              folder={child}
              folders={folders}
              feeds={feeds}
              level={level + 1}
              selectedFolderId={selectedFolderId}
              selectedFeedId={selectedFeedId}
              onSelectFolder={onSelectFolder}
              onSelectFeed={onSelectFeed}
              onEditFolder={onEditFolder}
              onDeleteFolder={onDeleteFolder}
              onCreateSubfolder={onCreateSubfolder}
            />
          ))}

          {/* Feeds in this folder */}
          {folderFeeds.map((feed) => (
            <FeedItem
              key={feed.id}
              feed={feed}
              level={level + 1}
              isSelected={selectedFeedId === feed.id}
              onSelect={() => onSelectFeed(feed.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function FeedItem({
  feed,
  level,
  isSelected,
  onSelect,
}: {
  feed: Feed;
  level: number;
  isSelected: boolean;
  onSelect: () => void;
}) {
  return (
    <button
      onClick={onSelect}
      className={cn(
        "w-full flex items-center gap-2 py-1.5 px-2 rounded-md text-sm",
        isSelected
          ? "bg-sidebar-accent text-sidebar-accent-foreground"
          : "hover:bg-sidebar-accent/50"
      )}
      style={{ paddingLeft: `${level * 12 + 28}px` }}
    >
      {feed.icon_url ? (
        <img
          src={feed.icon_url}
          alt=""
          className="w-4 h-4 rounded shrink-0"
          onError={(e) => {
            (e.target as HTMLImageElement).style.display = "none";
          }}
        />
      ) : (
        <Rss className="w-4 h-4 text-muted-foreground shrink-0" />
      )}
      <span className="truncate">{feed.title}</span>
      {feed.unread_count > 0 && (
        <span className="text-xs bg-muted text-muted-foreground px-1.5 py-0.5 rounded ml-auto">
          {feed.unread_count}
        </span>
      )}
    </button>
  );
}

interface FolderTreeProps {
  className?: string;
}

export function FolderTree({ className }: FolderTreeProps) {
  const { data: foldersRaw } = useFolders();
  const { data: feeds } = useFeeds();
  const createFolder = useCreateFolder();
  const updateFolder = useUpdateFolder();
  const deleteFolder = useDeleteFolder();

  const {
    selectedFolderId,
    selectedFeedId,
    setSelectedFolder,
    setSelectedFeed,
  } = useUIStore();

  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [showEditDialog, setShowEditDialog] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [editingFolder, setEditingFolder] = useState<FolderWithStats | null>(null);
  const [newFolderName, setNewFolderName] = useState("");
  const [parentFolderId, setParentFolderId] = useState<string | null>(null);

  // Cast folders to include computed fields
  const folders = (foldersRaw || []) as FolderWithStats[];
  const rootFolders = folders.filter((f) => f.parent_id === null);
  const rootFeeds = (feeds || []).filter((f) => f.folder_id === null);

  const handleCreateFolder = () => {
    if (!newFolderName.trim()) return;
    createFolder.mutate(
      { name: newFolderName.trim(), parentId: parentFolderId || undefined },
      {
        onSuccess: () => {
          setShowCreateDialog(false);
          setNewFolderName("");
          setParentFolderId(null);
        },
      }
    );
  };

  const handleEditFolder = () => {
    if (!editingFolder || !newFolderName.trim()) return;
    updateFolder.mutate(
      { id: editingFolder.id, name: newFolderName.trim() },
      {
        onSuccess: () => {
          setShowEditDialog(false);
          setEditingFolder(null);
          setNewFolderName("");
        },
      }
    );
  };

  const handleDeleteFolder = () => {
    if (!editingFolder) return;
    deleteFolder.mutate(editingFolder.id, {
      onSuccess: () => {
        setShowDeleteDialog(false);
        setEditingFolder(null);
        if (selectedFolderId === editingFolder.id) {
          setSelectedFolder(null);
        }
      },
    });
  };

  const openCreateDialog = (parentId: string | null = null) => {
    setParentFolderId(parentId);
    setNewFolderName("");
    setShowCreateDialog(true);
  };

  const openEditDialog = (folder: FolderWithStats) => {
    setEditingFolder(folder);
    setNewFolderName(folder.name);
    setShowEditDialog(true);
  };

  const openDeleteDialog = (folderId: string) => {
    const folder = folders.find((f) => f.id === folderId);
    if (folder) {
      setEditingFolder(folder);
      setShowDeleteDialog(true);
    }
  };

  return (
    <div className={cn("space-y-1", className)}>
      {/* Header with create button */}
      <div className="flex items-center justify-between px-2 py-1">
        <span className="text-xs font-medium text-muted-foreground uppercase">
          Folders
        </span>
        <Button
          variant="ghost"
          size="icon"
          className="h-6 w-6"
          onClick={() => openCreateDialog(null)}
          title="Create folder"
        >
          <Plus className="h-4 w-4" />
        </Button>
      </div>

      {/* Root folders */}
      {rootFolders.map((folder) => (
        <FolderNode
          key={folder.id}
          folder={folder}
          folders={folders}
          feeds={feeds || []}
          level={0}
          selectedFolderId={selectedFolderId}
          selectedFeedId={selectedFeedId}
          onSelectFolder={setSelectedFolder}
          onSelectFeed={setSelectedFeed}
          onEditFolder={openEditDialog}
          onDeleteFolder={openDeleteDialog}
          onCreateSubfolder={openCreateDialog}
        />
      ))}

      {/* Root feeds (no folder) */}
      {rootFeeds.length > 0 && (
        <div className="pt-2">
          <div className="px-2 py-1">
            <span className="text-xs font-medium text-muted-foreground uppercase">
              Uncategorized
            </span>
          </div>
          {rootFeeds.map((feed) => (
            <FeedItem
              key={feed.id}
              feed={feed}
              level={0}
              isSelected={selectedFeedId === feed.id}
              onSelect={() => setSelectedFeed(feed.id)}
            />
          ))}
        </div>
      )}

      {/* Create folder dialog */}
      <Dialog open={showCreateDialog} onOpenChange={setShowCreateDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Create Folder</DialogTitle>
            <DialogDescription>
              {parentFolderId
                ? "Create a new subfolder"
                : "Create a new folder to organize your feeds"}
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="folder-name">Folder name</Label>
              <Input
                id="folder-name"
                value={newFolderName}
                onChange={(e) => setNewFolderName(e.target.value)}
                placeholder="Enter folder name"
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleCreateFolder();
                }}
              />
            </div>
          </div>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowCreateDialog(false)}
            >
              Cancel
            </Button>
            <Button
              onClick={handleCreateFolder}
              disabled={!newFolderName.trim() || createFolder.isPending}
            >
              {createFolder.isPending ? "Creating..." : "Create"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Edit folder dialog */}
      <Dialog open={showEditDialog} onOpenChange={setShowEditDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Rename Folder</DialogTitle>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="edit-folder-name">Folder name</Label>
              <Input
                id="edit-folder-name"
                value={newFolderName}
                onChange={(e) => setNewFolderName(e.target.value)}
                placeholder="Enter folder name"
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleEditFolder();
                }}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowEditDialog(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleEditFolder}
              disabled={!newFolderName.trim() || updateFolder.isPending}
            >
              {updateFolder.isPending ? "Saving..." : "Save"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete folder dialog */}
      <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Folder</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete &quot;{editingFolder?.name}&quot;?
              This will also delete all feeds and articles in this folder.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowDeleteDialog(false)}
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={handleDeleteFolder}
              disabled={deleteFolder.isPending}
            >
              {deleteFolder.isPending ? "Deleting..." : "Delete"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
