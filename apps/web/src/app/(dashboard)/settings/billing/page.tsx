"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  billingApi,
  type Subscription,
  type UsageSummary,
  type Invoice,
  type PaymentMethod,
} from "@/lib/api";

export default function BillingPage() {
  const [subscription, setSubscription] = useState<Subscription | null>(null);
  const [usage, setUsage] = useState<UsageSummary | null>(null);
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [paymentMethods, setPaymentMethods] = useState<PaymentMethod[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadBillingData();
  }, []);

  const loadBillingData = async () => {
    try {
      setLoading(true);
      const [subRes, usageRes, invoicesRes, pmRes] = await Promise.all([
        billingApi.getSubscription(),
        billingApi.getUsage(),
        billingApi.getInvoices({ limit: 3 }),
        billingApi.getPaymentMethods(),
      ]);
      setSubscription(subRes.data);
      setUsage(usageRes.data);
      setInvoices(invoicesRes.data);
      setPaymentMethods(pmRes.data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load billing data");
    } finally {
      setLoading(false);
    }
  };

  const handleManageBilling = async () => {
    try {
      const res = await billingApi.createPortalSession();
      window.location.href = res.data.portal_url;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to open billing portal");
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

  if (loading) {
    return (
      <div className="container max-w-4xl py-8">
        <div className="animate-pulse space-y-4">
          <div className="h-8 w-48 bg-muted rounded" />
          <div className="h-32 bg-muted rounded" />
          <div className="h-32 bg-muted rounded" />
        </div>
      </div>
    );
  }

  return (
    <div className="container max-w-4xl py-8 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Billing</h1>
          <p className="text-muted-foreground">Manage your subscription and billing</p>
        </div>
        <Button variant="outline" onClick={handleManageBilling}>
          Manage Billing
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Current Plan */}
      <Card>
        <CardHeader>
          <CardTitle>Current Plan</CardTitle>
          <CardDescription>Your subscription details</CardDescription>
        </CardHeader>
        <CardContent>
          {subscription ? (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="text-lg font-semibold">{subscription.plan_name}</h3>
                  <p className="text-sm text-muted-foreground">
                    {subscription.interval === "annual" ? "Annual" : "Monthly"} billing
                  </p>
                </div>
                <Badge
                  variant={
                    subscription.status === "active"
                      ? "default"
                      : subscription.status === "past_due"
                        ? "destructive"
                        : "secondary"
                  }
                >
                  {subscription.status}
                </Badge>
              </div>

              {subscription.cancel_at_period_end && (
                <Alert>
                  <AlertDescription>
                    Your subscription will end on {formatDate(subscription.current_period_end)}
                  </AlertDescription>
                </Alert>
              )}

              <div className="text-sm text-muted-foreground">
                Current period: {formatDate(subscription.current_period_start)} -{" "}
                {formatDate(subscription.current_period_end)}
              </div>

              <div className="flex gap-2">
                <Button asChild>
                  <Link href="/settings/billing/plans">Change Plan</Link>
                </Button>
                {subscription.cancel_at_period_end ? (
                  <Button variant="outline" onClick={() => billingApi.reactivate()}>
                    Reactivate
                  </Button>
                ) : (
                  <Button
                    variant="ghost"
                    className="text-destructive"
                    onClick={() => billingApi.cancel()}
                  >
                    Cancel Subscription
                  </Button>
                )}
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              <p className="text-muted-foreground">You are on the Free plan</p>
              <Button asChild>
                <Link href="/settings/billing/plans">Upgrade to Pro</Link>
              </Button>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Usage */}
      {usage && (
        <Card>
          <CardHeader>
            <CardTitle>Usage This Period</CardTitle>
            <CardDescription>
              {formatDate(usage.period_start)} - {formatDate(usage.period_end)}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* AI Tokens */}
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>AI Tokens</span>
                <span>
                  {usage.ai_tokens.used.toLocaleString()} / {usage.ai_tokens.limit.toLocaleString()}
                </span>
              </div>
              <Progress value={(usage.ai_tokens.used / usage.ai_tokens.limit) * 100} />
              <p className="text-xs text-muted-foreground">
                {usage.ai_tokens.remaining.toLocaleString()} remaining
              </p>
            </div>

            {/* API Calls */}
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>API Calls</span>
                <span>
                  {usage.api_calls.used.toLocaleString()} / {usage.api_calls.limit.toLocaleString()}
                </span>
              </div>
              <Progress value={(usage.api_calls.used / usage.api_calls.limit) * 100} />
              <p className="text-xs text-muted-foreground">
                {usage.api_calls.remaining.toLocaleString()} remaining
              </p>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Payment Methods */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Payment Methods</CardTitle>
            <CardDescription>Manage your payment methods</CardDescription>
          </div>
          <Button asChild variant="outline" size="sm">
            <Link href="/settings/billing/payment">Manage</Link>
          </Button>
        </CardHeader>
        <CardContent>
          {paymentMethods.length > 0 ? (
            <div className="space-y-2">
              {paymentMethods.map((pm) => (
                <div key={pm.id} className="flex items-center justify-between p-3 border rounded-lg">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-6 bg-muted rounded flex items-center justify-center text-xs font-medium">
                      {pm.brand.toUpperCase()}
                    </div>
                    <span>•••• {pm.last4}</span>
                    <span className="text-sm text-muted-foreground">
                      Expires {pm.exp_month}/{pm.exp_year}
                    </span>
                  </div>
                  {pm.is_default && <Badge variant="secondary">Default</Badge>}
                </div>
              ))}
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">No payment methods on file</p>
          )}
        </CardContent>
      </Card>

      {/* Recent Invoices */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Recent Invoices</CardTitle>
            <CardDescription>Your billing history</CardDescription>
          </div>
          <Button asChild variant="outline" size="sm">
            <Link href="/settings/billing/invoices">View All</Link>
          </Button>
        </CardHeader>
        <CardContent>
          {invoices.length > 0 ? (
            <div className="space-y-2">
              {invoices.map((invoice) => (
                <div
                  key={invoice.id}
                  className="flex items-center justify-between p-3 border rounded-lg"
                >
                  <div>
                    <p className="font-medium">{invoice.number}</p>
                    <p className="text-sm text-muted-foreground">{formatDate(invoice.created_at)}</p>
                  </div>
                  <div className="flex items-center gap-4">
                    <span>{formatCurrency(invoice.amount, invoice.currency)}</span>
                    <Badge
                      variant={invoice.status === "paid" ? "default" : "secondary"}
                    >
                      {invoice.status}
                    </Badge>
                    {invoice.pdf_url && (
                      <Button variant="ghost" size="sm" asChild>
                        <a href={invoice.pdf_url} target="_blank" rel="noopener noreferrer">
                          PDF
                        </a>
                      </Button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">No invoices yet</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
