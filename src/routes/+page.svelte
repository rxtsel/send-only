<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { hasApiKey, saveApiKey } from "@/lib/commom/store";
  import Logo from "@/lib/components/logo.svelte";
  import { toast } from "svelte-sonner";
  import { z } from "zod/v4";
  import { Button } from "@/lib/components/ui/button";
  import * as Field from "@/lib/components/ui/field";
  import { Input } from "@/lib/components/ui/input";

  const apiKeySchema = z
    .string()
    .nonempty("API Key is required")
    .refine((v) => v.startsWith("re_"), {
      message: "API Key must start with 're_'",
    })
    .length(36, "API Key must be exactly 36 characters long");

  let errors: Record<string, string> = {};
  let isSaving = false;
  let isChecking = true;
  let apiKeyValue = "";

  // Check if API key exists on mount
  onMount(async () => {
    try {
      console.log("Checking if API key exists...");
      const exists = await hasApiKey();
      console.log("API key exists:", exists);
      if (exists) {
        // If the API key already exists, go directly to the composer
        goto("/mail/composer");
      }
    } catch (err) {
      console.error("Error checking API key:", err);
    } finally {
      isChecking = false;
    }
  });

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    errors = {};

    const result = apiKeySchema.safeParse(apiKeyValue);

    if (!result.success) {
      errors.apiKey = result.error.issues[0].message;
      return;
    }

    try {
      isSaving = true;
      await saveApiKey(result.data);
      toast.success("Setup complete! Welcome to SendOnly");
      goto("/composer");
    } catch (err) {
      console.error("Error saving API key:", err);
      toast.error("Failed to save API key. Please try again.");
    } finally {
      isSaving = false;
    }
  }
</script>

{#if isChecking}
  <div class="min-h-svh flex justify-center items-center">
    <div class="text-center">
      <Logo class="mx-auto mb-4" />
      <p class="text-muted-foreground">Loading...</p>
    </div>
  </div>
{:else}
  <main
    class="container min-h-svh flex justify-center items-center mx-auto flex-col"
  >
    <header class="text-center mb-8">
      <Logo class="mx-auto mb-6" />
      <h1 class="text-3xl font-bold mb-4">Welcome to SendOnly</h1>
      <p class="max-w-prose text-balance">
        Send emails from your own domain using Resend. No email hosting or inbox
        setup required.
      </p>
    </header>

    <form class="w-full max-w-sm" onsubmit={handleSubmit}>
      <Field.Group>
        <Field.Field>
          <Field.Label for="apiKey">Resend API Key</Field.Label>
          <Input
            id="apiKey"
            name="apiKey"
            bind:value={apiKeyValue}
            placeholder="re_123abc456def789gho0tevecovc94htoi"
            required
            disabled={isSaving}
            aria-invalid={!!errors.apiKey}
          />
          {#if errors.apiKey}
            <Field.Error>{errors.apiKey}</Field.Error>
          {/if}
          <Field.Description>
            Enter your Resend API key to get started. You can find it in your

            <a
              href="https://resend.com/api-keys"
              target="_blank"
              rel="noopener noreferrer"
              class="underline hover:text-foreground"
            >
              Resend dashboard
            </a>.
          </Field.Description>
        </Field.Field>
        <Field.Field>
          <Button type="submit" disabled={isSaving} class="w-full">
            {isSaving ? "Saving..." : "Continue"}
          </Button>
        </Field.Field>
      </Field.Group>
    </form>
  </main>
{/if}
