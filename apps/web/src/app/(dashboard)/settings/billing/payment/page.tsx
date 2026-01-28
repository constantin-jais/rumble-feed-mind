"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { billingApi, type PaymentMethod } from "@/lib/api";

export default function PaymentMethodsPage() {
  const [paymentMethods, setPaymentMethods] = useState<PaymentMethod[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deleting, setDeleting] = useState<string | null>(null);

  useEffect(() => {
    loadPaymentMethods();
  }, []);

  const loadPaymentMethods = async () => {
    try {
      setLoading(true);
      const res = await billingApi.getPaymentMethods();
      setPaymentMethods(res.data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load payment methods");
    } finally {
      setLoading(false);
    }
  };

  const handleAddPaymentMethod = async () => {
    try {
      setError(null);
      const res = await billingApi.addPaymentMethod();
      window.location.href = res.data.setup_url;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to add payment method");
    }
  };

  const handleSetDefault = async (id: string) => {
    try {
      setError(null);
      await billingApi.setDefaultPaymentMethod(id);
      await loadPaymentMethods();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to set default payment method");
    }
  };

  const handleDelete = async (id: string) => {
    try {
      setDeleting(id);
      setError(null);
      await billingApi.deletePaymentMethod(id);
      setPaymentMethods((prev) => prev.filter((pm) => pm.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete payment method");
    } finally {
      setDeleting(null);
    }
  };

  const getBrandIcon = (brand: string) => {
    switch (brand.toLowerCase()) {
      case "visa":
        return "VISA";
      case "mastercard":
        return "MC";
      case "amex":
        return "AMEX";
      case "discover":
        return "DISC";
      default:
        return brand.toUpperCase().slice(0, 4);
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
          <h1 className="text-2xl font-bold">Payment Methods</h1>
          <p className="text-muted-foreground">Manage your payment methods</p>
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
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Your Cards</CardTitle>
            <CardDescription>Cards saved to your account</CardDescription>
          </div>
          <Button onClick={handleAddPaymentMethod}>Add Card</Button>
        </CardHeader>
        <CardContent>
          {paymentMethods.length > 0 ? (
            <div className="space-y-4">
              {paymentMethods.map((pm) => (
                <div
                  key={pm.id}
                  className="flex items-center justify-between p-4 border rounded-lg"
                >
                  <div className="flex items-center gap-4">
                    <div className="w-14 h-10 bg-gradient-to-r from-gray-700 to-gray-900 rounded-md flex items-center justify-center text-white text-xs font-bold">
                      {getBrandIcon(pm.brand)}
                    </div>
                    <div>
                      <p className="font-medium">
                        {pm.brand.charAt(0).toUpperCase() + pm.brand.slice(1)} ending in {pm.last4}
                      </p>
                      <p className="text-sm text-muted-foreground">
                        Expires {pm.exp_month.toString().padStart(2, "0")}/{pm.exp_year}
                      </p>
                    </div>
                    {pm.is_default && (
                      <Badge variant="secondary">Default</Badge>
                    )}
                  </div>

                  <div className="flex items-center gap-2">
                    {!pm.is_default && (
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleSetDefault(pm.id)}
                      >
                        Set Default
                      </Button>
                    )}

                    <AlertDialog>
                      <AlertDialogTrigger asChild>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="text-destructive"
                          disabled={pm.is_default || deleting === pm.id}
                        >
                          {deleting === pm.id ? "Deleting..." : "Remove"}
                        </Button>
                      </AlertDialogTrigger>
                      <AlertDialogContent>
                        <AlertDialogHeader>
                          <AlertDialogTitle>Remove payment method?</AlertDialogTitle>
                          <AlertDialogDescription>
                            This will remove the card ending in {pm.last4} from your account.
                            This action cannot be undone.
                          </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                          <AlertDialogCancel>Cancel</AlertDialogCancel>
                          <AlertDialogAction
                            onClick={() => handleDelete(pm.id)}
                            className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                          >
                            Remove
                          </AlertDialogAction>
                        </AlertDialogFooter>
                      </AlertDialogContent>
                    </AlertDialog>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8 space-y-4">
              <div className="w-16 h-16 mx-auto bg-muted rounded-full flex items-center justify-center">
                <svg
                  className="w-8 h-8 text-muted-foreground"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
                  />
                </svg>
              </div>
              <div>
                <p className="font-medium">No payment methods</p>
                <p className="text-sm text-muted-foreground">
                  Add a card to enable automatic payments
                </p>
              </div>
              <Button onClick={handleAddPaymentMethod}>Add Your First Card</Button>
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Security</CardTitle>
          <CardDescription>Your payment information is secure</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4 text-sm text-muted-foreground">
          <div className="flex items-start gap-3">
            <svg
              className="w-5 h-5 text-primary mt-0.5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
              />
            </svg>
            <div>
              <p className="font-medium text-foreground">PCI DSS Compliant</p>
              <p>All card data is processed securely by Stripe</p>
            </div>
          </div>
          <div className="flex items-start gap-3">
            <svg
              className="w-5 h-5 text-primary mt-0.5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              />
            </svg>
            <div>
              <p className="font-medium text-foreground">Encrypted Storage</p>
              <p>We never store your full card number</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
