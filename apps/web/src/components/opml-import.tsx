"use client";

import { useState, useRef, useCallback } from "react";
import { Upload, FileText, Folder, Rss, CheckCircle2, AlertCircle, X } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { useImportOpml } from "@/lib/queries";
import { cn } from "@/lib/utils";

interface ParsedOpml {
  title?: string;
  feeds: Array<{
    title: string;
    xmlUrl: string;
    folder?: string;
  }>;
  folders: string[];
}

type ImportState = "idle" | "preview" | "importing" | "success" | "error";

interface OpmlImportProps {
  trigger?: React.ReactNode;
  onSuccess?: () => void;
}

export function OpmlImport({ trigger, onSuccess }: OpmlImportProps) {
  const [open, setOpen] = useState(false);
  const [state, setState] = useState<ImportState>("idle");
  const [file, setFile] = useState<File | null>(null);
  const [parsedOpml, setParsedOpml] = useState<ParsedOpml | null>(null);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const importMutation = useImportOpml();

  const resetState = useCallback(() => {
    setState("idle");
    setFile(null);
    setParsedOpml(null);
    setError(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  }, []);

  const handleOpenChange = useCallback((isOpen: boolean) => {
    setOpen(isOpen);
    if (!isOpen) {
      resetState();
    }
  }, [resetState]);

  const parseOpmlFile = useCallback(async (opmlFile: File): Promise<ParsedOpml> => {
    const text = await opmlFile.text();
    const parser = new DOMParser();
    const doc = parser.parseFromString(text, "text/xml");

    // Check for parsing errors
    const parseError = doc.querySelector("parsererror");
    if (parseError) {
      throw new Error("Le fichier OPML est invalide ou mal forme");
    }

    const title = doc.querySelector("title")?.textContent || undefined;
    const feeds: ParsedOpml["feeds"] = [];
    const foldersSet = new Set<string>();

    // Parse outlines - handle both flat and nested structures
    const processOutline = (outline: Element, parentFolder?: string) => {
      const xmlUrl = outline.getAttribute("xmlUrl");
      const feedTitle = outline.getAttribute("title") || outline.getAttribute("text") || "Sans titre";

      if (xmlUrl) {
        // This is a feed
        feeds.push({
          title: feedTitle,
          xmlUrl,
          folder: parentFolder,
        });
        if (parentFolder) {
          foldersSet.add(parentFolder);
        }
      } else {
        // This is a folder, process children
        const folderName = outline.getAttribute("title") || outline.getAttribute("text");
        const children = outline.querySelectorAll(":scope > outline");
        children.forEach((child) => processOutline(child, folderName || undefined));
      }
    };

    const bodyOutlines = doc.querySelectorAll("body > outline");
    bodyOutlines.forEach((outline) => processOutline(outline));

    return {
      title,
      feeds,
      folders: Array.from(foldersSet).sort(),
    };
  }, []);

  const handleFileSelect = useCallback(async (selectedFile: File) => {
    if (!selectedFile.name.toLowerCase().endsWith(".opml") &&
        !selectedFile.name.toLowerCase().endsWith(".xml")) {
      setError("Veuillez selectionner un fichier OPML (.opml ou .xml)");
      setState("error");
      return;
    }

    setFile(selectedFile);
    setError(null);

    try {
      const parsed = await parseOpmlFile(selectedFile);
      if (parsed.feeds.length === 0) {
        setError("Aucun flux RSS trouve dans ce fichier OPML");
        setState("error");
        return;
      }
      setParsedOpml(parsed);
      setState("preview");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors de l'analyse du fichier");
      setState("error");
    }
  }, [parseOpmlFile]);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    const droppedFile = e.dataTransfer.files[0];
    if (droppedFile) {
      handleFileSelect(droppedFile);
    }
  }, [handleFileSelect]);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
  }, []);

  const handleImport = useCallback(async () => {
    if (!file) return;

    setState("importing");
    setError(null);

    try {
      await importMutation.mutateAsync(file);
      setState("success");
      // Close dialog after a short delay on success
      setTimeout(() => {
        handleOpenChange(false);
        onSuccess?.();
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur lors de l'import");
      setState("error");
    }
  }, [file, importMutation, handleOpenChange, onSuccess]);

  const defaultTrigger = (
    <Button variant="outline" size="sm" className="gap-2">
      <Upload className="h-4 w-4" />
      Importer OPML
    </Button>
  );

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger asChild>
        {trigger || defaultTrigger}
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Importer des flux OPML</DialogTitle>
          <DialogDescription>
            Importez vos abonnements depuis un fichier OPML exporte d&apos;un autre lecteur RSS.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          {/* Idle state - File picker */}
          {state === "idle" && (
            <div
              onDrop={handleDrop}
              onDragOver={handleDragOver}
              onClick={() => fileInputRef.current?.click()}
              className={cn(
                "border-2 border-dashed rounded-lg p-8 text-center cursor-pointer",
                "hover:border-primary/50 hover:bg-muted/50 transition-colors"
              )}
            >
              <Upload className="h-10 w-10 mx-auto text-muted-foreground mb-4" />
              <p className="text-sm text-muted-foreground mb-2">
                Glissez-deposez votre fichier OPML ici
              </p>
              <p className="text-xs text-muted-foreground">
                ou cliquez pour parcourir
              </p>
              <input
                ref={fileInputRef}
                type="file"
                accept=".opml,.xml"
                onChange={(e) => {
                  const selectedFile = e.target.files?.[0];
                  if (selectedFile) {
                    handleFileSelect(selectedFile);
                  }
                }}
                className="hidden"
              />
            </div>
          )}

          {/* Preview state */}
          {state === "preview" && parsedOpml && (
            <div className="space-y-4">
              <div className="flex items-center gap-3 p-3 bg-muted rounded-lg">
                <FileText className="h-5 w-5 text-muted-foreground" />
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium truncate">{file?.name}</p>
                  {parsedOpml.title && (
                    <p className="text-xs text-muted-foreground">{parsedOpml.title}</p>
                  )}
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8"
                  onClick={resetState}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="flex items-center gap-2 p-3 bg-muted/50 rounded-lg">
                  <Rss className="h-4 w-4 text-primary" />
                  <div>
                    <p className="text-2xl font-bold">{parsedOpml.feeds.length}</p>
                    <p className="text-xs text-muted-foreground">flux</p>
                  </div>
                </div>
                <div className="flex items-center gap-2 p-3 bg-muted/50 rounded-lg">
                  <Folder className="h-4 w-4 text-primary" />
                  <div>
                    <p className="text-2xl font-bold">{parsedOpml.folders.length}</p>
                    <p className="text-xs text-muted-foreground">dossiers</p>
                  </div>
                </div>
              </div>

              {parsedOpml.folders.length > 0 && (
                <div className="text-xs text-muted-foreground">
                  <span className="font-medium">Dossiers: </span>
                  {parsedOpml.folders.join(", ")}
                </div>
              )}

              <div className="flex gap-2">
                <Button variant="outline" className="flex-1" onClick={resetState}>
                  Annuler
                </Button>
                <Button className="flex-1" onClick={handleImport}>
                  Importer {parsedOpml.feeds.length} flux
                </Button>
              </div>
            </div>
          )}

          {/* Importing state */}
          {state === "importing" && (
            <div className="space-y-4 py-4">
              <div className="text-center">
                <p className="text-sm font-medium mb-2">Import en cours...</p>
                <p className="text-xs text-muted-foreground">
                  Cela peut prendre quelques instants
                </p>
              </div>
              <Progress value={undefined} className="h-2" />
            </div>
          )}

          {/* Success state */}
          {state === "success" && (
            <div className="py-8 text-center">
              <CheckCircle2 className="h-12 w-12 mx-auto text-green-500 mb-4" />
              <p className="text-sm font-medium">Import reussi !</p>
              <p className="text-xs text-muted-foreground mt-1">
                {importMutation.data?.imported ?? parsedOpml?.feeds.length ?? 0} flux importes
              </p>
            </div>
          )}

          {/* Error state */}
          {state === "error" && (
            <div className="space-y-4">
              <Alert variant="destructive">
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>{error}</AlertDescription>
              </Alert>
              <Button variant="outline" className="w-full" onClick={resetState}>
                Reessayer
              </Button>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
