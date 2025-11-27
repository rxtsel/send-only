<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  import { hasApiKey, saveApiKey } from "@/lib/commom/store";
  import Logo from "@/lib/components/logo.svelte";

  import { toast } from "svelte-sonner";

  import { Button } from "@/lib/components/ui/button";
  import * as Field from "@/lib/components/ui/field";
  import { Input } from "@/lib/components/ui/input";
  import { ArrowRight, Loader } from "@lucide/svelte";
  import { profileSchema } from "@/lib/schemas/profile.schema";
  import { apiKeySchema } from "@/lib/schemas/api-key.schema";
  import { emailOptionSchema } from "@/lib/schemas/email-option.schema";
  import { saveProfile } from "@/lib/commom/profile";
  import { createFromEmail } from "@/lib/commom/from-emails";

  /* ---------------------------------------------------------
   * MULTISTEP STATE
   * --------------------------------------------------------- */
  let step = $state<1 | 2 | 3>(1);

  let isChecking = $state(true);
  let isSaving = $state(false);
  let errors = $state<Record<string, string>>({});

  /* ---------------------------------------------------------
   * STEP 1 — API KEY
   * --------------------------------------------------------- */
  let apiKeyValue = $state("");

  /* ---------------------------------------------------------
   * STEP 2 — PROFILE INFO
   * --------------------------------------------------------- */
  let firstName = $state("");
  let lastName = $state("");
  let username = $state("");
  let domain = $state("");

  /* ---------------------------------------------------------
   * STEP 3 — FROM EMAIL OPTIONS
   * --------------------------------------------------------- */
  let emailLabel = $state("");
  let emailAddress = $state("");

  let emailOptions = $state<{ label: string; address: string }[]>([]);

  /* ---------------------------------------------------------
   * CHECK API KEY ON LOAD
   * --------------------------------------------------------- */
  onMount(async () => {
    isChecking = true;
    try {
      const exists = await hasApiKey();

      if (exists) {
        goto("/mail/composer");
        return;
      }

      isChecking = false;
    } catch (err) {
      console.error(err);
      isChecking = false;
    }
  });

  /* ---------------------------------------------------------
   * STEP HANDLERS
   * --------------------------------------------------------- */
  function nextStep() {
    errors = {};
    if (step < 3) step = (step + 1) as 1 | 2 | 3;
  }

  function prevStep() {
    errors = {};
    if (step > 1) step = (step - 1) as 1 | 2 | 3;
  }

  /* STEP 1 */
  function submitStep1(e: SubmitEvent) {
    e.preventDefault();
    errors = {};

    const result = apiKeySchema.safeParse(apiKeyValue);
    if (!result.success) {
      errors.apiKey = result.error.issues[0].message;
      return;
    }

    nextStep();
  }

  /* STEP 2 */
  function submitStep2(e: SubmitEvent) {
    e.preventDefault();
    errors = {};

    const result = profileSchema.safeParse({
      firstName,
      lastName,
      username,
      domain,
    });

    if (!result.success) {
      for (const issue of result.error.issues) {
        errors[issue.path[0]?.toString()] = issue.message;
      }
      return;
    }

    nextStep();
  }

  /* STEP 3 */
  async function submitStep3(e: SubmitEvent) {
    e.preventDefault();
    errors = {};

    if (emailOptions.length === 0) {
      errors.fromEmail = "Add at least one email option";
      return;
    }

    try {
      isSaving = true;

      // Save data in storage
      await Promise.all([
        saveApiKey(apiKeyValue),
        saveProfile({
          firstName,
          lastName,
          username,
          domain,
        }),
        ...emailOptions.map((opt, i) =>
          createFromEmail({
            label: opt.label,
            address: opt.address,
            isDefault: i === 0,
          }),
        ),
      ]);

      toast.success("Setup complete! Welcome to SendOnly");
      goto("/mail/composer");
    } catch (err) {
      console.error(err);
      toast.error("Failed to save. Try again.");
    } finally {
      isSaving = false;
    }
  }

  /* Add email option in step 3 */
  function addEmailOption() {
    errors = {};

    const result = emailOptionSchema.safeParse({
      label: emailLabel,
      address: emailAddress,
    });

    if (!result.success) {
      for (const issue of result.error.issues) {
        errors[issue.path[0]?.toString()] = issue.message;
      }
      return;
    }

    emailOptions = [
      ...emailOptions,
      { label: emailLabel, address: emailAddress },
    ];

    emailLabel = "";
    emailAddress = "";
  }
</script>

{#if isChecking}
  <div class="min-h-dvh w-full flex items-center justify-center">
    <Loader class="animate-spin" />
  </div>
{:else}
  <main class="container min-h-svh flex justify-center items-center flex-col">
    <header class="text-center mb-8">
      <Logo class="mx-auto mb-6" />
      <h1 class="text-3xl font-bold mb-4">
        {#if step === 1}Welcome to SendOnly{/if}
        {#if step === 2}Profile Setup{/if}
        {#if step === 3}Email Options{/if}
      </h1>
      <p class="max-w-prose text-balance">
        {#if step === 1}
          Enter your Resend API key to get started.
        {/if}
        {#if step === 2}
          Tell us a bit about your profile.
        {/if}
        {#if step === 3}
          Add the email identities you will use to send emails.
        {/if}
      </p>
    </header>

    <!-- -----------------------------------------------------
         STEP 1
    ------------------------------------------------------ -->
    {#if step === 1}
      <form class="w-full max-w-sm" onsubmit={submitStep1}>
        <Field.Group>
          <Field.Field>
            <Field.Label for="apiKey">Resend API Key</Field.Label>
            <Input
              id="apiKey"
              bind:value={apiKeyValue}
              placeholder="re_123abc..."
              aria-invalid={!!errors.apiKey}
              autofocus
            />
            {#if errors.apiKey}
              <Field.Error>{errors.apiKey}</Field.Error>
            {/if}
          </Field.Field>
          <Field.Field>
            <Field.Description>
              You can find your API key in your{" "}
              <a
                href="https://resend.com/api-keys"
                target="_blank"
                class="underline underline-offset-4 font-medium"
              >
                Resend dashboard
              </a>.
            </Field.Description>
          </Field.Field>

          <Button type="submit" class="w-full">Continue <ArrowRight /></Button>
        </Field.Group>
      </form>
    {/if}

    <!-- -----------------------------------------------------
         STEP 2
    ------------------------------------------------------ -->
    {#if step === 2}
      <form class="w-full max-w-sm" onsubmit={submitStep2}>
        <Field.Group>
          <div class="grid grid-cols-2 sm:grid-cols-2 gap-x-2">
            <Field.Field>
              <Field.Label for="firstName">First Name</Field.Label>
              <Input
                id="firstName"
                placeholder="John"
                bind:value={firstName}
                aria-invalid={!!errors.firstName}
                autofocus
              />
              {#if errors.firstName}
                <Field.Error>{errors.firstName}</Field.Error>
              {/if}
            </Field.Field>

            <Field.Field>
              <Field.Label for="lastName">Last Name</Field.Label>
              <Input
                id="lastName"
                placeholder="Doe"
                bind:value={lastName}
                aria-invalid={!!errors.lastName}
                autofocus
              />
              {#if errors.lastName}
                <Field.Error>{errors.lastName}</Field.Error>
              {/if}
            </Field.Field>
          </div>

          <Field.Field>
            <Field.Label for="username">Username</Field.Label>
            <Input
              id="username"
              placeholder="john_doe"
              bind:value={username}
              aria-invalid={!!errors.username}
              autofocus
            />
            {#if errors.username}
              <Field.Error>{errors.username}</Field.Error>
            {/if}
          </Field.Field>

          <Field.Field>
            <Field.Label for="domain">Domain</Field.Label>
            <Input
              id="domain"
              bind:value={domain}
              placeholder="yourdomain.com"
              aria-invalid={!!errors.domain}
            />
            {#if errors.domain}
              <Field.Error>{errors.domain}</Field.Error>
            {/if}
          </Field.Field>

          <div class="flex gap-2">
            <Button
              type="button"
              variant="outline"
              class="flex-1"
              onclick={prevStep}
            >
              <ArrowRight class="-rotate-180" /> Back
            </Button>
            <Button type="submit" class="flex-1">Continue <ArrowRight /></Button
            >
          </div>
        </Field.Group>
      </form>
    {/if}

    <!-- -----------------------------------------------------
         STEP 3
    ------------------------------------------------------ -->
    {#if step === 3}
      <form class="w-full max-w-sm" onsubmit={submitStep3}>
        <Field.Group>
          <Field.Field>
            <Field.Label>From Email Options</Field.Label>

            <div class="flex gap-2 mb-3">
              <Input
                placeholder="Label"
                bind:value={emailLabel}
                aria-invalid={!!errors.label}
                autofocus
              />
              <Input
                placeholder="email@domain.com"
                bind:value={emailAddress}
                aria-invalid={!!errors.address}
              />
            </div>

            {#if errors.label}
              <Field.Error>{errors.label}</Field.Error>
            {/if}
            {#if errors.address}
              <Field.Error>{errors.address}</Field.Error>
            {/if}

            <Button type="button" class="w-full mb-4" onclick={addEmailOption}>
              Add email
            </Button>

            <!-- LIST -->
            {#each emailOptions as opt}
              <p class="border px-3 py-1 rounded bg-accent/20">
                {opt.label} - {opt.address}
              </p>
            {/each}

            {#if errors.fromEmail}
              <Field.Error>{errors.fromEmail}</Field.Error>
            {/if}
          </Field.Field>

          <div class="flex gap-2">
            <Button
              type="button"
              variant="outline"
              class="flex-1"
              onclick={prevStep}
            >
              <ArrowRight class="-rotate-180" /> Back
            </Button>
            <Button type="submit" class="flex-1" disabled={isSaving}>
              {isSaving ? "Saving..." : "Finish"}
            </Button>
          </div>
        </Field.Group>
      </form>
    {/if}
  </main>
{/if}
