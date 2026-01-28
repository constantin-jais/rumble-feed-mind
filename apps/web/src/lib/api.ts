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
  category_filter: FeedCategoryFilter | null;
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
  categories: string[];
  created_at?: string;
}

export interface Category {
  category: string;
  article_count: number;
  feed_count: number;
}

export interface FeedCategory {
  category: string;
  article_count: number;
  first_seen_at: string;
  last_seen_at: string;
}

export interface FeedCategoryFilter {
  mode: "include" | "exclude";
  categories: string[];
}

export interface Folder {
  id: string;
  name: string;
  parent_id: string | null;
  position: number;
  feed_count?: number;
  unread_count?: number;
  created_at?: string;
  updated_at?: string;
}

export interface Tag {
  id: string;
  name: string;
  color: string | null;
  article_count: number;
  created_at: string;
}

export interface ArticleTag {
  tag_id: string;
  tag_name: string;
  tag_color: string | null;
  applied_by: string;
  created_at: string;
}

export interface RuleConfig {
  pattern: string;
  fields: ("title" | "content" | "summary" | "author" | "url")[];
  case_sensitive: boolean;
}

export interface RulePreviewResult {
  total_articles: number;
  matched_articles: number;
  sample_matches: Array<{
    article_id: string;
    title: string;
    matched_field: string;
    matched_text: string;
  }>;
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
    data: Partial<Pick<Feed, "title" | "folder_id" | "priority" | "category_filter">>,
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
    categories?: string[];
    limit?: number;
    cursor?: string;
  }) => {
    const searchParams = new URLSearchParams();
    if (params?.feed_id) searchParams.set("feed_id", params.feed_id);
    if (params?.folder_id) searchParams.set("folder_id", params.folder_id);
    if (params?.status) searchParams.set("status", params.status);
    if (params?.categories?.length)
      searchParams.set("categories", params.categories.join(","));
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

// Categories API
export const categoriesApi = {
  list: () => apiFetch<ListResponse<Category>>("/api/v1/categories"),

  listForFeed: (feedId: string) =>
    apiFetch<ListResponse<FeedCategory>>(`/api/v1/feeds/${feedId}/categories`),
};

// Tags API
export const tagsApi = {
  list: () => apiFetch<{ data: Tag[]; meta: { total: number } }>("/api/v1/tags"),

  get: (id: string) => apiFetch<{ data: Tag }>(`/api/v1/tags/${id}`),

  create: (data: { name: string; color?: string }) =>
    apiFetch<{ data: Tag }>("/api/v1/tags", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  update: (id: string, data: { name?: string; color?: string }) =>
    apiFetch<{ data: Tag }>(`/api/v1/tags/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    }),

  delete: (id: string) =>
    apiFetch<{ data: { success: boolean } }>(`/api/v1/tags/${id}`, {
      method: "DELETE",
    }),

  getArticleTags: (articleId: string) =>
    apiFetch<{ data: ArticleTag[] }>(`/api/v1/articles/${articleId}/tags`),

  addTagToArticle: (articleId: string, tagId: string) =>
    apiFetch<{ data: ArticleTag[] }>(`/api/v1/articles/${articleId}/tags`, {
      method: "POST",
      body: JSON.stringify({ tag_id: tagId }),
    }),

  removeTagFromArticle: (articleId: string, tagId: string) =>
    apiFetch<{ data: { success: boolean } }>(
      `/api/v1/articles/${articleId}/tags/${tagId}`,
      { method: "DELETE" }
    ),
};

// Extended Rules API with preview
export const rulesApiExtended = {
  ...rulesApi,

  preview: (data: {
    config: RuleConfig;
    feed_id?: string;
    folder_id?: string;
    limit?: number;
  }) =>
    apiFetch<{ data: RulePreviewResult }>("/api/v1/rules/preview", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  toggle: (id: string, isActive: boolean) =>
    apiFetch<{ data: Rule }>(`/api/v1/rules/${id}/toggle`, {
      method: "POST",
      body: JSON.stringify({ is_active: isActive }),
    }),

  reorder: (ruleIds: string[]) =>
    apiFetch<{ data: Rule[]; meta: { total: number } }>("/api/v1/rules/reorder", {
      method: "POST",
      body: JSON.stringify({ rule_ids: ruleIds }),
    }),
};

// ============================================================================
// BILLING API
// ============================================================================

export interface Plan {
  id: string;
  name: string;
  tier: "free" | "pro" | "team";
  description: string;
  price_monthly: number;
  price_annual: number;
  features: string[];
  limits: {
    max_feeds: number;
    max_rules: number;
    ai_tokens: number;
    api_calls: number;
  };
  is_current: boolean;
  is_popular: boolean;
}

export interface Subscription {
  id: string;
  plan_id: string;
  plan_name: string;
  status: "active" | "past_due" | "canceled" | "incomplete";
  interval: "monthly" | "annual";
  current_period_start: string;
  current_period_end: string;
  cancel_at_period_end: boolean;
  canceled_at: string | null;
  trial_end: string | null;
}

export interface UsageSummary {
  ai_tokens: {
    used: number;
    limit: number;
    remaining: number;
  };
  api_calls: {
    used: number;
    limit: number;
    remaining: number;
  };
  period_start: string;
  period_end: string;
}

export interface UsageHistory {
  date: string;
  ai_tokens: number;
  api_calls: number;
}

export interface Invoice {
  id: string;
  number: string;
  status: "draft" | "open" | "paid" | "void" | "uncollectible";
  amount: number;
  currency: string;
  period_start: string;
  period_end: string;
  paid_at: string | null;
  pdf_url: string | null;
  created_at: string;
}

export interface PaymentMethod {
  id: string;
  type: "card";
  brand: string;
  last4: string;
  exp_month: number;
  exp_year: number;
  is_default: boolean;
}

export const billingApi = {
  // Plans
  getPlans: () =>
    apiFetch<{ data: Plan[] }>("/api/v1/billing/plans"),

  // Subscription
  getSubscription: () =>
    apiFetch<{ data: Subscription | null }>("/api/v1/billing/subscription"),

  subscribe: (priceId: string) =>
    apiFetch<{ data: { checkout_url: string } }>("/api/v1/billing/subscribe", {
      method: "POST",
      body: JSON.stringify({ price_id: priceId }),
    }),

  changePlan: (priceId: string) =>
    apiFetch<{ data: Subscription }>("/api/v1/billing/change-plan", {
      method: "POST",
      body: JSON.stringify({ price_id: priceId }),
    }),

  cancel: (reason?: string) =>
    apiFetch<{ data: Subscription }>("/api/v1/billing/cancel", {
      method: "POST",
      body: JSON.stringify({ reason }),
    }),

  reactivate: () =>
    apiFetch<{ data: Subscription }>("/api/v1/billing/reactivate", {
      method: "POST",
    }),

  // Usage
  getUsage: () =>
    apiFetch<{ data: UsageSummary }>("/api/v1/billing/usage"),

  getUsageHistory: (params?: { days?: number }) => {
    const searchParams = new URLSearchParams();
    if (params?.days) searchParams.set("days", params.days.toString());
    const query = searchParams.toString();
    return apiFetch<{ data: UsageHistory[] }>(
      `/api/v1/billing/usage/history${query ? `?${query}` : ""}`
    );
  },

  // Invoices
  getInvoices: (params?: { limit?: number; cursor?: string }) => {
    const searchParams = new URLSearchParams();
    if (params?.limit) searchParams.set("limit", params.limit.toString());
    if (params?.cursor) searchParams.set("cursor", params.cursor);
    const query = searchParams.toString();
    return apiFetch<ListResponse<Invoice>>(
      `/api/v1/billing/invoices${query ? `?${query}` : ""}`
    );
  },

  getInvoice: (id: string) =>
    apiFetch<{ data: Invoice }>(`/api/v1/billing/invoices/${id}`),

  // Payment Methods
  getPaymentMethods: () =>
    apiFetch<{ data: PaymentMethod[] }>("/api/v1/billing/payment-methods"),

  addPaymentMethod: () =>
    apiFetch<{ data: { setup_url: string } }>("/api/v1/billing/payment-methods", {
      method: "POST",
    }),

  deletePaymentMethod: (id: string) =>
    apiFetch<{ data: { success: boolean } }>(`/api/v1/billing/payment-methods/${id}`, {
      method: "DELETE",
    }),

  setDefaultPaymentMethod: (id: string) =>
    apiFetch<{ data: PaymentMethod }>(`/api/v1/billing/payment-methods/${id}/default`, {
      method: "POST",
    }),

  // Customer Portal
  createPortalSession: () =>
    apiFetch<{ data: { portal_url: string } }>("/api/v1/billing/portal-session", {
      method: "POST",
    }),
};
