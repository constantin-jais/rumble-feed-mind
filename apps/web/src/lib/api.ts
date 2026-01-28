// API client for FeedMind backend

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

// Types
export interface User {
  id: string;
  email: string;
  display_name: string | null;
  tier: "free" | "pro_trial" | "pro" | "team";
}

export interface AuthResponse {
  data: {
    token: string;
    expires_at: string;
    user: User;
  };
}

export interface Feed {
  id: string;
  url: string;
  title: string;
  description: string | null;
  site_url: string | null;
  icon_url: string | null;
  feed_type: string | null;
  priority: "hot" | "warm" | "cold";
  folder_id: string | null;
  article_count: number;
  unread_count: number;
  error_count: number;
  last_error: string | null;
  last_fetched_at: string | null;
  created_at: string;
}

export interface Article {
  id: string;
  feed_id: string;
  feed_title?: string;
  url: string | null;
  title: string;
  author: string | null;
  summary: string | null;
  content?: string | null;
  image_url: string | null;
  published_at: string | null;
  is_read: boolean;
  is_starred: boolean;
  is_hidden?: boolean;
  word_count: number | null;
  created_at?: string;
}

export interface Folder {
  id: string;
  name: string;
  parent_id: string | null;
  position: number;
}

export interface Rule {
  id: string;
  name: string;
  description: string | null;
  rule_type: "regex" | "ai";
  config: Record<string, unknown>;
  action: "hide" | "star" | "tag" | "mark_read";
  action_params: Record<string, unknown> | null;
  feed_id: string | null;
  folder_id: string | null;
  priority: number;
  stop_on_match: boolean;
  is_active: boolean;
  match_count: number;
  last_match_at: string | null;
  created_at: string;
}

export interface ListResponse<T> {
  data: T[];
  meta: {
    total: number;
    cursor: string | null;
    has_more: boolean;
  };
}

// API Error
export class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

// Token storage
let authToken: string | null = null;

export function setAuthToken(token: string | null) {
  authToken = token;
  if (token) {
    localStorage.setItem("feedmind_token", token);
  } else {
    localStorage.removeItem("feedmind_token");
  }
}

export function getAuthToken(): string | null {
  if (authToken) return authToken;
  if (typeof window !== "undefined") {
    authToken = localStorage.getItem("feedmind_token");
  }
  return authToken;
}

// Base fetch wrapper
async function apiFetch<T>(
  endpoint: string,
  options: RequestInit = {},
  skipContentType = false,
): Promise<T> {
  const token = getAuthToken();

  const headers: Record<string, string> = {};

  // Only set Content-Type for non-FormData requests
  if (!skipContentType) {
    headers["Content-Type"] = "application/json";
  }

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      ...headers,
      ...(options.headers as Record<string, string>),
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new ApiError(
      response.status,
      error.code || "UNKNOWN_ERROR",
      error.message || `Request failed with status ${response.status}`,
    );
  }

  return response.json();
}

// Auth API
export const authApi = {
  register: (email: string, password: string, displayName?: string) =>
    apiFetch<AuthResponse>("/api/v1/auth/register", {
      method: "POST",
      body: JSON.stringify({ email, password, display_name: displayName }),
    }),

  login: (email: string, password: string) =>
    apiFetch<AuthResponse>("/api/v1/auth/login", {
      method: "POST",
      body: JSON.stringify({ email, password }),
    }),

  logout: () =>
    apiFetch<{ data: { success: boolean } }>("/api/v1/auth/logout", {
      method: "POST",
    }),

  me: () => apiFetch<{ data: User }>("/api/v1/auth/me"),

  refresh: () =>
    apiFetch<AuthResponse>("/api/v1/auth/refresh", { method: "POST" }),
};

// Feeds API
export const feedsApi = {
  list: (params?: { folder_id?: string; limit?: number }) => {
    const searchParams = new URLSearchParams();
    if (params?.folder_id) searchParams.set("folder_id", params.folder_id);
    if (params?.limit) searchParams.set("limit", params.limit.toString());
    const query = searchParams.toString();
    return apiFetch<ListResponse<Feed>>(
      `/api/v1/feeds${query ? `?${query}` : ""}`,
    );
  },

  get: (id: string) => apiFetch<{ data: Feed }>(`/api/v1/feeds/${id}`),

  create: (url: string, folderId?: string, title?: string) =>
    apiFetch<{ data: Feed }>("/api/v1/feeds", {
      method: "POST",
      body: JSON.stringify({ url, folder_id: folderId, title }),
    }),

  update: (
    id: string,
    data: Partial<Pick<Feed, "title" | "folder_id" | "priority">>,
  ) =>
    apiFetch<{ data: Feed }>(`/api/v1/feeds/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  delete: (id: string) =>
    apiFetch<{ data: { success: boolean } }>(`/api/v1/feeds/${id}`, {
      method: "DELETE",
    }),

  refresh: (id: string) =>
    apiFetch<{ data: { queued: boolean } }>(`/api/v1/feeds/${id}/refresh`, {
      method: "POST",
    }),
};

// Articles API
export const articlesApi = {
  list: (params?: {
    feed_id?: string;
    folder_id?: string;
    status?: "unread" | "read" | "starred" | "hidden";
    limit?: number;
    cursor?: string;
  }) => {
    const searchParams = new URLSearchParams();
    if (params?.feed_id) searchParams.set("feed_id", params.feed_id);
    if (params?.folder_id) searchParams.set("folder_id", params.folder_id);
    if (params?.status) searchParams.set("status", params.status);
    if (params?.limit) searchParams.set("limit", params.limit.toString());
    if (params?.cursor) searchParams.set("cursor", params.cursor);
    const query = searchParams.toString();
    return apiFetch<ListResponse<Article>>(
      `/api/v1/articles${query ? `?${query}` : ""}`,
    );
  },

  get: (id: string) => apiFetch<{ data: Article }>(`/api/v1/articles/${id}`),

  update: (id: string, data: { is_read?: boolean; is_starred?: boolean }) =>
    apiFetch<{ data: Article }>(`/api/v1/articles/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  batchUpdate: (
    articleIds: string[],
    data: { is_read?: boolean; is_starred?: boolean },
  ) =>
    apiFetch<{ data: { updated: number } }>("/api/v1/articles/batch", {
      method: "PUT",
      body: JSON.stringify({ article_ids: articleIds, ...data }),
    }),

  markAllRead: (params?: { feed_id?: string; folder_id?: string }) => {
    const searchParams = new URLSearchParams();
    if (params?.feed_id) searchParams.set("feed_id", params.feed_id);
    if (params?.folder_id) searchParams.set("folder_id", params.folder_id);
    const query = searchParams.toString();
    return apiFetch<{ data: { updated: number } }>(
      `/api/v1/articles/mark-all-read${query ? `?${query}` : ""}`,
      { method: "POST" },
    );
  },
};

// OPML types
export interface OpmlFeedPreview {
  title: string;
  xmlUrl: string;
  htmlUrl?: string;
  folder?: string;
}

export interface OpmlParseResult {
  feeds: OpmlFeedPreview[];
  folders: string[];
  title?: string;
}

export interface OpmlImportResult {
  imported: number;
  skipped: number;
  feeds: Feed[];
  errors?: string[];
}

// OPML API
export const opmlApi = {
  import: (file: File) => {
    const formData = new FormData();
    formData.append("file", file);
    return apiFetch<{ data: OpmlImportResult }>(
      "/api/v1/opml/import",
      {
        method: "POST",
        body: formData,
      },
      true, // Skip Content-Type to let browser set multipart boundary
    );
  },

  export: () => apiFetch<Blob>("/api/v1/opml/export"),
};

// Rules API
export const rulesApi = {
  list: (params?: { feed_id?: string; active?: boolean }) => {
    const searchParams = new URLSearchParams();
    if (params?.feed_id) searchParams.set("feed_id", params.feed_id);
    if (params?.active !== undefined)
      searchParams.set("active", params.active.toString());
    const query = searchParams.toString();
    return apiFetch<ListResponse<Rule>>(
      `/api/v1/rules${query ? `?${query}` : ""}`,
    );
  },

  get: (id: string) => apiFetch<{ data: Rule }>(`/api/v1/rules/${id}`),

  create: (data: {
    name: string;
    pattern: string;
    action: Rule["action"];
    feed_id?: string;
    folder_id?: string;
    priority?: number;
    stop_on_match?: boolean;
  }) =>
    apiFetch<{ data: Rule }>("/api/v1/rules", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  update: (id: string, data: Partial<Rule>) =>
    apiFetch<{ data: Rule }>(`/api/v1/rules/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  delete: (id: string) =>
    apiFetch<{ data: { success: boolean } }>(`/api/v1/rules/${id}`, {
      method: "DELETE",
    }),

  test: (id: string, articleId: string) =>
    apiFetch<{ data: { matched: boolean; explanation: string } }>(
      `/api/v1/rules/${id}/test`,
      {
        method: "POST",
        body: JSON.stringify({ article_id: articleId }),
      },
    ),
};

// Folders API
export const foldersApi = {
  list: () => apiFetch<ListResponse<Folder>>("/api/v1/folders"),

  get: (id: string) => apiFetch<{ data: Folder }>(`/api/v1/folders/${id}`),

  create: (name: string, parentId?: string) =>
    apiFetch<{ data: Folder }>("/api/v1/folders", {
      method: "POST",
      body: JSON.stringify({ name, parent_id: parentId }),
    }),

  update: (
    id: string,
    data: Partial<Pick<Folder, "name" | "parent_id" | "position">>,
  ) =>
    apiFetch<{ data: Folder }>(`/api/v1/folders/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  delete: (id: string) =>
    apiFetch<{ data: { success: boolean } }>(`/api/v1/folders/${id}`, {
      method: "DELETE",
    }),
};

// API Keys types
export interface ApiKeys {
  anthropic_key_set: boolean;
  openai_key_set: boolean;
}

// Settings API
export const settingsApi = {
  getApiKeys: () => apiFetch<{ data: ApiKeys }>("/api/v1/settings/api-keys"),

  updateApiKeys: (data: { anthropic_key?: string; openai_key?: string }) =>
    apiFetch<{ data: ApiKeys }>("/api/v1/settings/api-keys", {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  deleteAccount: () =>
    apiFetch<{ data: { success: boolean } }>("/api/v1/settings/account", {
      method: "DELETE",
    }),
};
