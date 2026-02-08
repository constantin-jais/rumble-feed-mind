// TanStack Query hooks for data fetching

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  feedsApi,
  foldersApi,
  articlesApi,
  rulesApi,
  authApi,
  settingsApi,
  opmlApi,
  setAuthToken,
} from "./api";
import { useAuthStore } from "./store";
import type { Feed, Folder, Article, Rule } from "./api";

// Query keys
export const queryKeys = {
  user: ["user"] as const,
  feeds: ["feeds"] as const,
  feed: (id: string) => ["feeds", id] as const,
  folders: ["folders"] as const,
  folder: (id: string) => ["folders", id] as const,
  articles: (params?: {
    feed_id?: string;
    folder_id?: string;
    status?: string;
  }) => ["articles", params] as const,
  article: (id: string) => ["articles", id] as const,
  rules: ["rules"] as const,
  rule: (id: string) => ["rules", id] as const,
  apiKeys: ["apiKeys"] as const,
};

// Auth hooks
export function useUser() {
  const { isAuthenticated } = useAuthStore();

  return useQuery({
    queryKey: queryKeys.user,
    queryFn: async () => {
      const response = await authApi.me();
      return response.data;
    },
    enabled: isAuthenticated,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

export function useLogin() {
  const queryClient = useQueryClient();
  const { setAuth } = useAuthStore();

  return useMutation({
    mutationFn: async ({
      email,
      password,
    }: {
      email: string;
      password: string;
    }) => {
      const response = await authApi.login(email, password);
      return response.data;
    },
    onSuccess: (data) => {
      setAuth(data.user, data.token, data.expires_at);
      queryClient.invalidateQueries({ queryKey: queryKeys.user });
    },
  });
}

export function useRegister() {
  const queryClient = useQueryClient();
  const { setAuth } = useAuthStore();

  return useMutation({
    mutationFn: async ({
      email,
      password,
      displayName,
    }: {
      email: string;
      password: string;
      displayName?: string;
    }) => {
      const response = await authApi.register(email, password, displayName);
      return response.data;
    },
    onSuccess: (data) => {
      setAuth(data.user, data.token, data.expires_at);
      queryClient.invalidateQueries({ queryKey: queryKeys.user });
    },
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  const { clearAuth } = useAuthStore();

  return useMutation({
    mutationFn: async () => {
      await authApi.logout();
    },
    onSuccess: () => {
      setAuthToken(null);
      clearAuth();
      queryClient.clear();
    },
  });
}

// Feeds hooks
export function useFeeds(folderId?: string) {
  return useQuery({
    queryKey: [...queryKeys.feeds, { folder_id: folderId }],
    queryFn: async () => {
      const response = await feedsApi.list({ folder_id: folderId });
      return response.data;
    },
  });
}

export function useFeed(id: string) {
  return useQuery({
    queryKey: queryKeys.feed(id),
    queryFn: async () => {
      const response = await feedsApi.get(id);
      return response.data;
    },
    enabled: !!id,
  });
}

export function useCreateFeed() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      url,
      folderId,
      title,
    }: {
      url: string;
      folderId?: string;
      title?: string;
    }) => {
      const response = await feedsApi.create(url, folderId, title);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
    },
  });
}

export function useUpdateFeed() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      id,
      ...data
    }: {
      id: string;
      title?: string;
      folder_id?: string;
      priority?: Feed["priority"];
    }) => {
      const response = await feedsApi.update(id, data);
      return response.data;
    },
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
      queryClient.invalidateQueries({ queryKey: queryKeys.feed(id) });
    },
  });
}

export function useDeleteFeed() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (id: string) => {
      await feedsApi.delete(id);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
      queryClient.invalidateQueries({ queryKey: ["articles"] });
    },
  });
}

export function useRefreshFeed() {
  return useMutation({
    mutationFn: async (id: string) => {
      const response = await feedsApi.refresh(id);
      return response.data;
    },
  });
}

// Articles hooks
export function useArticles(params?: {
  feed_id?: string;
  folder_id?: string;
  status?: "unread" | "read" | "starred";
}) {
  return useQuery({
    queryKey: queryKeys.articles(params),
    queryFn: async () => {
      const response = await articlesApi.list(params);
      return response;
    },
  });
}

export function useArticle(id: string) {
  return useQuery({
    queryKey: queryKeys.article(id),
    queryFn: async () => {
      const response = await articlesApi.get(id);
      return response.data;
    },
    enabled: !!id,
  });
}

export function useUpdateArticle() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      id,
      ...data
    }: {
      id: string;
      is_read?: boolean;
      is_starred?: boolean;
    }) => {
      const response = await articlesApi.update(id, data);
      return response.data;
    },
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ["articles"] });
      queryClient.invalidateQueries({ queryKey: queryKeys.article(id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
    },
  });
}

export function useBatchUpdateArticles() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      articleIds,
      ...data
    }: {
      articleIds: string[];
      is_read?: boolean;
      is_starred?: boolean;
    }) => {
      const response = await articlesApi.batchUpdate(articleIds, data);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["articles"] });
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
    },
  });
}

export function useMarkAllRead() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params?: { feed_id?: string; folder_id?: string }) => {
      const response = await articlesApi.markAllRead(params);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["articles"] });
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
    },
  });
}

// Rules hooks
export function useRules(feedId?: string) {
  return useQuery({
    queryKey: [...queryKeys.rules, { feed_id: feedId }],
    queryFn: async () => {
      const response = await rulesApi.list({ feed_id: feedId });
      return response.data;
    },
  });
}

export function useRule(id: string) {
  return useQuery({
    queryKey: queryKeys.rule(id),
    queryFn: async () => {
      const response = await rulesApi.get(id);
      return response.data;
    },
    enabled: !!id,
  });
}

export function useCreateRule() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: Parameters<typeof rulesApi.create>[0]) => {
      const response = await rulesApi.create(data);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.rules });
    },
  });
}

export function useUpdateRule() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({ id, ...data }: { id: string } & Partial<Rule>) => {
      const response = await rulesApi.update(id, data);
      return response.data;
    },
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.rules });
      queryClient.invalidateQueries({ queryKey: queryKeys.rule(id) });
    },
  });
}

export function useDeleteRule() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (id: string) => {
      await rulesApi.delete(id);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.rules });
    },
  });
}

// Settings hooks
export function useApiKeys() {
  const { isAuthenticated } = useAuthStore();

  return useQuery({
    queryKey: queryKeys.apiKeys,
    queryFn: async () => {
      const response = await settingsApi.getApiKeys();
      return response.data;
    },
    enabled: isAuthenticated,
  });
}

export function useUpdateApiKeys() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: {
      anthropic_key?: string;
      openai_key?: string;
    }) => {
      const response = await settingsApi.updateApiKeys(data);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.apiKeys });
    },
  });
}

export function useDeleteAccount() {
  const queryClient = useQueryClient();
  const { clearAuth } = useAuthStore();

  return useMutation({
    mutationFn: async () => {
      await settingsApi.deleteAccount();
    },
    onSuccess: () => {
      setAuthToken(null);
      clearAuth();
      queryClient.clear();
    },
  });
}

// OPML hooks
export function useImportOpml() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (file: File) => {
      const response = await opmlApi.import(file);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
      queryClient.invalidateQueries({ queryKey: queryKeys.folders });
    },
  });
}

// Folders hooks
export function useFolders() {
  return useQuery({
    queryKey: queryKeys.folders,
    queryFn: async () => {
      const response = await foldersApi.list();
      return response.data;
    },
  });
}

export function useFolder(id: string) {
  return useQuery({
    queryKey: queryKeys.folder(id),
    queryFn: async () => {
      const response = await foldersApi.get(id);
      return response.data;
    },
    enabled: !!id,
  });
}

export function useCreateFolder() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      name,
      parentId,
    }: {
      name: string;
      parentId?: string;
    }) => {
      const response = await foldersApi.create(name, parentId);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folders });
    },
  });
}

export function useUpdateFolder() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async ({
      id,
      ...data
    }: {
      id: string;
      name?: string;
      parent_id?: string;
      position?: number;
    }) => {
      const response = await foldersApi.update(id, data);
      return response.data;
    },
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folders });
      queryClient.invalidateQueries({ queryKey: queryKeys.folder(id) });
    },
  });
}

export function useDeleteFolder() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (id: string) => {
      await foldersApi.delete(id);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folders });
      queryClient.invalidateQueries({ queryKey: queryKeys.feeds });
    },
  });
}
