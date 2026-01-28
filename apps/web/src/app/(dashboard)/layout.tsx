"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { Sidebar } from "@/components/sidebar";
import { useAuthStore } from "@/lib/store";
import { setAuthToken } from "@/lib/api";

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const { token, isAuthenticated } = useAuthStore();

  useEffect(() => {
    // Restore token to API client
    if (token) {
      setAuthToken(token);
    }
  }, [token]);

  useEffect(() => {
    // Redirect to login if not authenticated
    if (!isAuthenticated && !token) {
      router.push("/login");
    }
  }, [isAuthenticated, token, router]);

  // Show nothing while checking auth
  if (!isAuthenticated && !token) {
    return null;
  }

  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <main className="flex-1 flex flex-col overflow-hidden">{children}</main>
    </div>
  );
}
