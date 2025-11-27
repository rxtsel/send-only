import { invoke } from "@tauri-apps/api/core";
import type { SentEmail } from "../types/sent.type";
import { emailCache } from "../stores/email-cache.svelte";

export async function listSentEmails(
  limit?: number,
  offset?: number,
  forceRefresh = false
): Promise<SentEmail[]> {
  // Only use cache for initial load (offset 0)
  if (!forceRefresh && offset === 0) {
    const cached = emailCache.getList();
    if (cached) return cached
  }

  const emails = await invoke<SentEmail[]>("list_sent_emails", { limit, offset });

  // Cache only the initial load
  if (offset === 0) {
    emailCache.setList(emails);
  }

  return emails;
}

export async function getSentEmail(emailId: string): Promise<SentEmail> {
  // Check cache first
  const cached = emailCache.get(emailId);
  if (cached) {
    console.log(`Using cached email: ${emailId}`);
    return cached;
  }

  console.log(`Fetching email from API: ${emailId}`);
  const email = await invoke<SentEmail>("get_sent_email", { emailId });

  // Cache the result
  emailCache.set(emailId, email);

  return email;
}

export function invalidateEmailCache(): void {
  emailCache.invalidateList();
}

export function clearEmailCache(): void {
  emailCache.clear();
}

export function formatEmailDate(dateString: string): string {
  try {
    // Resend returns dates like "2023-04-03T22:13:42.674981+00:00"
    let date: Date;

    // Try parsing directly first
    date = new Date(dateString);

    // If invalid, try cleaning the string
    if (isNaN(date.getTime())) {
      // Remove microseconds if present (keep only milliseconds)
      const cleaned = dateString.replace(/(\.\d{3})\d+/, "$1");
      date = new Date(cleaned);
    }

    // Check if date is valid
    if (isNaN(date.getTime())) {
      console.warn("Invalid date format:", dateString);
      return dateString;
    }

    const now = new Date();
    const diffInMs = now.getTime() - date.getTime();
    const diffInDays = Math.floor(diffInMs / (1000 * 60 * 60 * 24));

    if (diffInDays === 0) {
      return date.toLocaleTimeString("en-US", {
        hour: "2-digit",
        minute: "2-digit",
      });
    } else if (diffInDays === 1) {
      return "Yesterday";
    } else if (diffInDays < 7) {
      return `${diffInDays} days ago`;
    } else if (diffInDays < 30) {
      const weeks = Math.floor(diffInDays / 7);
      return `${weeks} week${weeks > 1 ? "s" : ""} ago`;
    } else {
      return date.toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: date.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
      });
    }
  } catch (error) {
    console.error("Error formatting date:", dateString, error);
    return dateString;
  }
}

// For conditional render beetween html or Editor Tap component
export function isTemplateHtml(html: string | null): boolean {
  if (!html) return false;

  // 1) Has <style> tag
  if (/<style[\s\S]*?>[\s\S]*?<\/style>/i.test(html)) return true;

  // 2) Inline styles: style="..."
  if (/style\s*=\s*["'][^"']+["']/i.test(html)) return true;

  // 3) Typical email-layout tags or attributes
  if (/(<table|<tr|<td|<font|<center|bgcolor\s*=)/i.test(html)) return true;

  // 4) Many classes (very rough heuristic)
  const classMatches = html.match(/class\s*=\s*["'][^"']+["']/gi);
  if (classMatches && classMatches.length > 5) return true;

  return false;
}

