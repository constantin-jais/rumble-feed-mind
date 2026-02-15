"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { billingApi, type Invoice } from "@/lib/api";

export default function InvoicesPage() {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [loading, setLoading] = useState(true);
  const [hasMore, setHasMore] = useState(false);
  const [cursor, setCursor] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadInvoices();
  }, []);

  const loadInvoices = async (loadMore = false) => {
    try {
      if (!loadMore) setLoading(true);
      const res = await billingApi.getInvoices({
        limit: 20,
        cursor: loadMore ? cursor ?? undefined : undefined,
      });
      if (loadMore) {
        setInvoices((prev) => [...prev, ...res.data]);
      } else {
        setInvoices(res.data);
      }
      setHasMore(res.meta.has_more);
      setCursor(res.meta.cursor);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load invoices");
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (amount: number, currency: string) => {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: currency.toUpperCase(),
    }).format(amount / 100);
  };

  const formatDate = (date: string) => {
    return new Date(date).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  };

  const getStatusVariant = (status: Invoice["status"]) => {
    switch (status) {
      case "paid":
        return "default";
      case "open":
        return "secondary";
      case "draft":
        return "outline";
      case "void":
      case "uncollectible":
        return "destructive";
      default:
        return "secondary";
    }
  };

  if (loading) {
    return (
      <div className="container max-w-4xl py-8">
        <div className="animate-pulse space-y-4">
          <div className="h-8 w-48 bg-muted rounded" />
          <div className="h-64 bg-muted rounded" />
        </div>
      </div>
    );
  }

  return (
    <div className="container max-w-4xl py-8 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Invoices</h1>
          <p className="text-muted-foreground">View and download your invoices</p>
        </div>
        <Button variant="outline" asChild>
          <Link href="/settings/billing">Back to Billing</Link>
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      <Card>
        <CardHeader>
          <CardTitle>Invoice History</CardTitle>
          <CardDescription>All your past invoices</CardDescription>
        </CardHeader>
        <CardContent>
          {invoices.length > 0 ? (
            <div className="space-y-2">
              <div className="grid grid-cols-5 gap-4 px-4 py-2 text-sm font-medium text-muted-foreground border-b">
                <div>Invoice</div>
                <div>Date</div>
                <div>Period</div>
                <div>Amount</div>
                <div>Status</div>
              </div>
              {invoices.map((invoice) => (
                <div
                  key={invoice.id}
                  className="grid grid-cols-5 gap-4 px-4 py-3 items-center border-b last:border-0 hover:bg-muted/50 rounded-lg"
                >
                  <div className="font-medium">{invoice.number}</div>
                  <div className="text-sm text-muted-foreground">
                    {formatDate(invoice.created_at)}
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {formatDate(invoice.period_start)} - {formatDate(invoice.period_end)}
                  </div>
                  <div>{formatCurrency(invoice.amount, invoice.currency)}</div>
                  <div className="flex items-center gap-2">
                    <Badge variant={getStatusVariant(invoice.status)}>
                      {invoice.status}
                    </Badge>
                    {invoice.pdf_url && (
                      <Button variant="ghost" size="sm" asChild>
                        <a href={invoice.pdf_url} target="_blank" rel="noopener noreferrer">
                          <svg
                            className="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              strokeWidth={2}
                              d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                            />
                          </svg>
                        </a>
                      </Button>
                    )}
                  </div>
                </div>
              ))}

              {hasMore && (
                <div className="pt-4 text-center">
                  <Button variant="outline" onClick={() => loadInvoices(true)}>
                    Load More
                  </Button>
                </div>
              )}
            </div>
          ) : (
            <p className="text-center py-8 text-muted-foreground">
              No invoices yet. Invoices will appear here after your first payment.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
