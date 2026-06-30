"use client";

import { useState, useEffect, useMemo } from "react";
import { AlertCircle, Eye, CheckCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { cn } from "@/lib/utils";
import { useCreateRule, useUpdateRule, usePreviewRule, useFolders, useFeeds } from "@/lib/queries";
import type { Rule, RuleConfig } from "@/lib/api";

interface RuleEditorProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  rule?: Rule;
  defaultFeedId?: string;
  defaultFolderId?: string;
}

const RULE_ACTIONS = [
  { value: "hide", label: "Hide article" },
  { value: "star", label: "Star article" },
  { value: "mark_read", label: "Mark as read" },
  { value: "tag", label: "Add tag" },
] as const;

const RULE_FIELDS = [
  { value: "title", label: "Title" },
  { value: "content", label: "Content" },
  { value: "summary", label: "Summary" },
  { value: "author", label: "Author" },
  { value: "url", label: "URL" },
] as const;

export function RuleEditor({
  open,
  onOpenChange,
  rule,
  defaultFeedId,
  defaultFolderId,
}: RuleEditorProps) {
  const createRule = useCreateRule();
  const updateRule = useUpdateRule();
  const previewRule = usePreviewRule();
  const { data: folders } = useFolders();
  const { data: feeds } = useFeeds();

  const [name, setName] = useState("");
  const [pattern, setPattern] = useState("");
  const [fields, setFields] = useState<string[]>(["title"]);
  const [caseSensitive, setCaseSensitive] = useState(false);
  const [action, setAction] = useState<string>("hide");
  const [tagName, setTagName] = useState("");
  const [feedId, setFeedId] = useState<string | undefined>(defaultFeedId);
  const [folderId, setFolderId] = useState<string | undefined>(defaultFolderId);
  const [stopOnMatch, setStopOnMatch] = useState(false);

  const isEditing = !!rule;

  useEffect(() => {
    if (rule) {
      // Form state is intentionally reset when opening the editor on another rule.
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setName(rule.name);
      const config = rule.config as unknown as RuleConfig;
      setPattern(config?.pattern || "");
      setFields(config?.fields || ["title"]);
      setCaseSensitive(config?.case_sensitive || false);
      setAction(rule.action);
      if (rule.action === "tag" && rule.action_params) {
        const params = rule.action_params as { tags?: string[] };
        setTagName(params.tags?.[0] || "");
      }
      setFeedId(rule.feed_id || undefined);
      setFolderId(rule.folder_id || undefined);
      setStopOnMatch(rule.stop_on_match);
    } else {
      setName("");
      setPattern("");
      setFields(["title"]);
      setCaseSensitive(false);
      setAction("hide");
      setTagName("");
      setFeedId(defaultFeedId);
      setFolderId(defaultFolderId);
      setStopOnMatch(false);
    }
  }, [rule, open, defaultFeedId, defaultFolderId]);

  const regexError = useMemo(() => {
    if (!pattern) {
      return null;
    }
    try {
      new RegExp(pattern, caseSensitive ? "" : "i");
      return null;
    } catch (e) {
      return (e as Error).message;
    }
  }, [pattern, caseSensitive]);

  const handleFieldToggle = (field: string) => {
    if (fields.includes(field)) {
      if (fields.length > 1) {
        setFields(fields.filter((f) => f !== field));
      }
    } else {
      setFields([...fields, field]);
    }
  };

  const handlePreview = () => {
    if (!pattern || regexError) return;

    const config: RuleConfig = {
      pattern,
      fields: fields as RuleConfig["fields"],
      case_sensitive: caseSensitive,
    };

    previewRule.mutate({
      config,
      feed_id: feedId,
      folder_id: folderId,
      limit: 50,
    });
  };

  const handleSubmit = () => {
    if (!name || !pattern || regexError) return;
    if (action === "tag" && !tagName) return;

    const config: RuleConfig = {
      pattern,
      fields: fields as RuleConfig["fields"],
      case_sensitive: caseSensitive,
    };

    const actionParams =
      action === "tag" ? { tags: [tagName] } : undefined;

    if (isEditing) {
      updateRule.mutate(
        {
          id: rule.id,
          name,
          config: config as unknown as Record<string, unknown>,
          action: action as Rule["action"],
          action_params: actionParams as Record<string, unknown> | undefined,
          feed_id: feedId || null,
          folder_id: folderId || null,
          stop_on_match: stopOnMatch,
        },
        {
          onSuccess: () => onOpenChange(false),
        }
      );
    } else {
      createRule.mutate(
        {
          name,
          pattern,
          action: action as Rule["action"],
          feed_id: feedId,
          folder_id: folderId,
          stop_on_match: stopOnMatch,
        },
        {
          onSuccess: () => onOpenChange(false),
        }
      );
    }
  };

  const isPending = createRule.isPending || updateRule.isPending;
  const isValid = name && pattern && !regexError && (action !== "tag" || tagName);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>{isEditing ? "Edit Rule" : "Create Rule"}</DialogTitle>
          <DialogDescription>
            Create a regex-based rule to automatically filter articles
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="rule-name">Rule name</Label>
            <Input
              id="rule-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Hide crypto spam"
            />
          </div>

          {/* Pattern */}
          <div className="space-y-2">
            <Label htmlFor="pattern">Regex pattern</Label>
            <Input
              id="pattern"
              value={pattern}
              onChange={(e) => setPattern(e.target.value)}
              placeholder="e.g., bitcoin|crypto|nft"
              className={cn(regexError && "border-destructive")}
            />
            {regexError && (
              <p className="text-xs text-destructive">{regexError}</p>
            )}
          </div>

          {/* Fields to match */}
          <div className="space-y-2">
            <Label>Match in fields</Label>
            <div className="flex flex-wrap gap-2">
              {RULE_FIELDS.map((field) => (
                <button
                  key={field.value}
                  type="button"
                  onClick={() => handleFieldToggle(field.value)}
                  className={cn(
                    "px-3 py-1.5 text-sm rounded-full border transition-colors",
                    fields.includes(field.value)
                      ? "bg-primary text-primary-foreground border-primary"
                      : "bg-background hover:bg-muted"
                  )}
                >
                  {field.label}
                </button>
              ))}
            </div>
          </div>

          {/* Case sensitive */}
          <div className="flex items-center gap-2">
            <input
              type="checkbox"
              id="case-sensitive"
              checked={caseSensitive}
              onChange={(e) => setCaseSensitive(e.target.checked)}
              className="h-4 w-4 rounded border-gray-300"
            />
            <Label htmlFor="case-sensitive" className="font-normal">
              Case sensitive
            </Label>
          </div>

          {/* Action */}
          <div className="space-y-2">
            <Label>Action</Label>
            <Select value={action} onValueChange={setAction}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {RULE_ACTIONS.map((a) => (
                  <SelectItem key={a.value} value={a.value}>
                    {a.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Tag name (if action is tag) */}
          {action === "tag" && (
            <div className="space-y-2">
              <Label htmlFor="tag-name">Tag name</Label>
              <Input
                id="tag-name"
                value={tagName}
                onChange={(e) => setTagName(e.target.value)}
                placeholder="e.g., important"
              />
            </div>
          )}

          {/* Scope */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Apply to folder</Label>
              <Select
                value={folderId || "all"}
                onValueChange={(v) => setFolderId(v === "all" ? undefined : v)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All folders" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All folders</SelectItem>
                  {folders?.map((folder) => (
                    <SelectItem key={folder.id} value={folder.id}>
                      {folder.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label>Apply to feed</Label>
              <Select
                value={feedId || "all"}
                onValueChange={(v) => setFeedId(v === "all" ? undefined : v)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All feeds" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All feeds</SelectItem>
                  {feeds?.map((feed) => (
                    <SelectItem key={feed.id} value={feed.id}>
                      {feed.title}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          {/* Stop on match */}
          <div className="flex items-center gap-2">
            <input
              type="checkbox"
              id="stop-on-match"
              checked={stopOnMatch}
              onChange={(e) => setStopOnMatch(e.target.checked)}
              className="h-4 w-4 rounded border-gray-300"
            />
            <Label htmlFor="stop-on-match" className="font-normal">
              Stop evaluating other rules if this matches
            </Label>
          </div>

          {/* Preview results */}
          {previewRule.data && (
            <Alert>
              <CheckCircle className="h-4 w-4" />
              <AlertDescription>
                Pattern would match {previewRule.data.matched_articles} of{" "}
                {previewRule.data.total_articles} recent articles
                {previewRule.data.sample_matches.length > 0 && (
                  <ul className="mt-2 space-y-1 text-xs">
                    {previewRule.data.sample_matches.slice(0, 3).map((match) => (
                      <li key={match.article_id} className="truncate">
                        &quot;{match.title}&quot; - {match.matched_field}
                      </li>
                    ))}
                  </ul>
                )}
              </AlertDescription>
            </Alert>
          )}

          {previewRule.isError && (
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertDescription>
                Failed to preview rule. Please check the pattern.
              </AlertDescription>
            </Alert>
          )}
        </div>

        <DialogFooter className="gap-2 sm:gap-0">
          <Button
            type="button"
            variant="outline"
            onClick={handlePreview}
            disabled={!pattern || !!regexError || previewRule.isPending}
          >
            <Eye className="mr-2 h-4 w-4" />
            Preview
          </Button>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={!isValid || isPending}>
            {isPending ? "Saving..." : isEditing ? "Save" : "Create"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
