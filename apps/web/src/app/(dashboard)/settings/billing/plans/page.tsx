"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { billingApi, type Plan, type Subscription } from "@/lib/api";

export default function PlansPage() {
  const router = useRouter();
  const [plans, setPlans] = useState<Plan[]>([]);
  const [subscription, setSubscription] = useState<Subscription | null>(null);
  const [loading, setLoading] = useState(true);
  const [subscribing, setSubscribing] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [billingInterval, setBillingInterval] = useState<"monthly" | "annual">("monthly");

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [plansRes, subRes] = await Promise.all([
        billingApi.getPlans(),
        billingApi.getSubscription(),
      ]);
      setPlans(plansRes.data);
      setSubscription(subRes.data);
      if (subRes.data) {
        setBillingInterval(subRes.data.interval);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load plans");
    } finally {
      setLoading(false);
    }
  };

  const handleSubscribe = async (plan: Plan) => {
    try {
      setSubscribing(plan.id);
      setError(null);

      const priceId = billingInterval === "annual"
        ? `${plan.tier}_annual`
        : `${plan.tier}_monthly`;

      if (subscription) {
        // Change existing subscription
        await billingApi.changePlan(priceId);
        router.push("/settings/billing");
      } else {
        // New subscription - redirect to checkout
        const res = await billingApi.subscribe(priceId);
        window.location.href = res.data.checkout_url;
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to subscribe");
    } finally {
      setSubscribing(null);
    }
  };

  const formatPrice = (plan: Plan) => {
    const price = billingInterval === "annual" ? plan.price_annual : plan.price_monthly;
    if (price === 0) return "Free";
    return `$${(price / 100).toFixed(0)}/mo`;
  };

  const getAnnualSavings = (plan: Plan) => {
    if (plan.price_monthly === 0) return null;
    const monthlyTotal = plan.price_monthly * 12;
    const savings = monthlyTotal - plan.price_annual;
    if (savings <= 0) return null;
    return Math.round((savings / monthlyTotal) * 100);
  };

  if (loading) {
    return (
      <div className="container max-w-5xl py-8">
        <div className="animate-pulse space-y-4">
          <div className="h-8 w-48 bg-muted rounded" />
          <div className="grid md:grid-cols-3 gap-4">
            {[1, 2, 3].map((i) => (
              <div key={i} className="h-96 bg-muted rounded" />
            ))}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="container max-w-5xl py-8 space-y-6">
      <div className="text-center space-y-2">
        <h1 className="text-3xl font-bold">Choose your plan</h1>
        <p className="text-muted-foreground">
          Select the plan that best fits your needs
        </p>
      </div>

      {/* Billing interval toggle */}
      <div className="flex justify-center">
        <div className="inline-flex items-center gap-2 p-1 bg-muted rounded-lg">
          <Button
            variant={billingInterval === "monthly" ? "default" : "ghost"}
            size="sm"
            onClick={() => setBillingInterval("monthly")}
          >
            Monthly
          </Button>
          <Button
            variant={billingInterval === "annual" ? "default" : "ghost"}
            size="sm"
            onClick={() => setBillingInterval("annual")}
          >
            Annual
            <Badge variant="secondary" className="ml-2">
              Save 20%
            </Badge>
          </Button>
        </div>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* Plans grid */}
      <div className="grid md:grid-cols-3 gap-6">
        {plans.map((plan) => {
          const isCurrentPlan = subscription?.plan_id === plan.id;
          const savings = getAnnualSavings(plan);

          return (
            <Card
              key={plan.id}
              className={`relative ${plan.is_popular ? "border-primary shadow-lg" : ""}`}
            >
              {plan.is_popular && (
                <div className="absolute -top-3 left-1/2 -translate-x-1/2">
                  <Badge>Most Popular</Badge>
                </div>
              )}

              <CardHeader>
                <CardTitle>{plan.name}</CardTitle>
                <CardDescription>{plan.description}</CardDescription>
              </CardHeader>

              <CardContent className="space-y-4">
                <div>
                  <span className="text-4xl font-bold">{formatPrice(plan)}</span>
                  {plan.price_monthly > 0 && billingInterval === "annual" && savings && (
                    <span className="text-sm text-muted-foreground ml-2">
                      Save {savings}%
                    </span>
                  )}
                </div>

                <ul className="space-y-2 text-sm">
                  {plan.features.map((feature, i) => (
                    <li key={i} className="flex items-center gap-2">
                      <svg
                        className="w-4 h-4 text-primary"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M5 13l4 4L19 7"
                        />
                      </svg>
                      {feature}
                    </li>
                  ))}
                </ul>

                <div className="pt-4 border-t text-sm text-muted-foreground space-y-1">
                  <p>{plan.limits.max_feeds.toLocaleString()} feeds</p>
                  <p>{plan.limits.max_rules.toLocaleString()} rules</p>
                  <p>{plan.limits.ai_tokens.toLocaleString()} AI tokens/mo</p>
                  <p>{plan.limits.api_calls.toLocaleString()} API calls/mo</p>
                </div>
              </CardContent>

              <CardFooter>
                {isCurrentPlan ? (
                  <Button className="w-full" variant="secondary" disabled>
                    Current Plan
                  </Button>
                ) : plan.tier === "free" ? (
                  <Button className="w-full" variant="outline" disabled>
                    Free Forever
                  </Button>
                ) : (
                  <Button
                    className="w-full"
                    onClick={() => handleSubscribe(plan)}
                    disabled={subscribing !== null}
                  >
                    {subscribing === plan.id ? "Processing..." : "Subscribe"}
                  </Button>
                )}
              </CardFooter>
            </Card>
          );
        })}
      </div>

      <div className="text-center text-sm text-muted-foreground">
        <p>All plans include a 14-day free trial. Cancel anytime.</p>
        <p>Prices shown in USD. Taxes may apply.</p>
      </div>
    </div>
  );
}
