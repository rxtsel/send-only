<script lang="ts">
  import * as Breadcrumb from "@/lib/components/ui/breadcrumb/index.js";
  import { Button } from "@/lib/components/ui/button/index.js";
  import * as Dialog from "@/lib/components/ui/dialog/index.js";
  import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
  import { Mail, User, Settings, SquarePen } from "@lucide/svelte";
  import * as Field from "@/lib/components/ui/field";
  import { Input } from "@/lib/components/ui/input";
  import type { FromEmail } from "../types";
  import { onMount } from "svelte";
  import { formatFromEmail, listFromEmails } from "../commom/from-emails";

  const data = {
    nav: [
      { name: "Profile", icon: User },
      { name: "Email sender options", icon: Mail },
    ],
  };

  let open = $state(false);
  let activeItem = $state("Profile");
  let errors: Record<string, string> = {};
  let isSaving = false;
  let fromEmails = $state<FromEmail[]>([]);

  function handleSubmitProfile(e: SubmitEvent) {
    e.preventDefault();
    console.log("Profile form submitted");
  }

  function handleSubmitEmailOptions(e: SubmitEvent) {
    e.preventDefault();
    console.log("Email sender options form submitted");
  }

  // Get From email options
  onMount(async () => {
    try {
      const list = await listFromEmails();
      fromEmails = list;
    } catch (e) {
      console.error("[fromEmails] load error", e);
    }
  });
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger>
    {#snippet child({ props })}
      <Button size="icon-sm" variant="ghost" {...props}>
        <Settings />
      </Button>
    {/snippet}
  </Dialog.Trigger>
  <Dialog.Content
    class="overflow-hidden p-0 md:max-h-[500px] md:max-w-[700px] lg:max-w-[800px]"
    trapFocus={false}
  >
    <Dialog.Title class="sr-only">Settings</Dialog.Title>
    <Dialog.Description class="sr-only"
      >Customize your settings here.</Dialog.Description
    >
    <Sidebar.Provider class="items-start">
      <Sidebar.Root collapsible="none" class="hidden md:flex">
        <Sidebar.Content>
          <Sidebar.Group>
            <Sidebar.GroupContent>
              <Sidebar.Menu>
                {#each data.nav as item (item.name)}
                  <Sidebar.MenuItem>
                    <Sidebar.MenuButton isActive={item.name === activeItem}>
                      {#snippet child({ props })}
                        <button
                          {...props}
                          onclick={() => (activeItem = item.name)}
                        >
                          <item.icon />
                          <span>{item.name}</span>
                        </button>
                      {/snippet}
                    </Sidebar.MenuButton>
                  </Sidebar.MenuItem>
                {/each}
              </Sidebar.Menu>
            </Sidebar.GroupContent>
          </Sidebar.Group>
        </Sidebar.Content>
      </Sidebar.Root>
      <main class="flex h-[480px] flex-1 flex-col overflow-hidden">
        <header
          class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12"
        >
          <div class="flex items-center gap-2 px-4">
            <Breadcrumb.Root>
              <Breadcrumb.List>
                <Breadcrumb.Item class="hidden md:block">
                  <Breadcrumb.Page>Settings</Breadcrumb.Page>
                </Breadcrumb.Item>
                <Breadcrumb.Separator class="hidden md:block" />
                <Breadcrumb.Item>
                  <Breadcrumb.Page>{activeItem}</Breadcrumb.Page>
                </Breadcrumb.Item>
              </Breadcrumb.List>
            </Breadcrumb.Root>
          </div>
        </header>
        <div class="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0 mr-7">
          {#if activeItem === "Profile"}
            {@render profile()}
          {:else if activeItem === "Email sender options"}
            {@render emailSenderOptions()}
          {/if}
        </div>
      </main>
    </Sidebar.Provider>
  </Dialog.Content>
</Dialog.Root>

{#snippet profile()}
  <form class="w-full" onsubmit={handleSubmitProfile}>
    <Field.Group>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-x-2">
        <Field.Field>
          <Field.Label for="firstName">First Name</Field.Label>
          <Input
            id="firstName"
            name="firstName"
            placeholder="Jonh"
            required
            aria-invalid={!!errors.firstName}
          />
          {#if errors.firstName}
            <Field.Error>{errors.firstName}</Field.Error>
          {/if}
        </Field.Field>

        <Field.Field>
          <Field.Label for="lastName">Last Name</Field.Label>
          <Input
            id="lastName"
            name="lastName"
            placeholder="Doe"
            required
            aria-invalid={!!errors.lastName}
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
          name="username"
          placeholder="john_doe"
          required
          aria-invalid={!!errors.username}
        />
        {#if errors.username}
          <Field.Error>{errors.username}</Field.Error>
        {/if}
      </Field.Field>
      <Field.Field>
        <Button type="submit" disabled={isSaving} class="w-full">
          {isSaving ? "Saving..." : "Save Changes"}
        </Button>
      </Field.Field>
    </Field.Group>
  </form>
{/snippet}

{#snippet emailSenderOptions()}
  <form class="w-full" onsubmit={handleSubmitProfile}>
    <Field.Group>
      <Field.Field>
        <Field.Label for="emai">Email</Field.Label>
        <Input
          id="email"
          name="email"
          placeholder="email@example.com"
          required
          aria-invalid={!!errors.email}
        />
        {#if errors.email}
          <Field.Error>{errors.email}</Field.Error>
        {/if}
      </Field.Field>
      <Field.Field>
        <Button type="submit" disabled={isSaving} class="w-full">
          {isSaving ? "Saving..." : "Create"}
        </Button>
      </Field.Field>
    </Field.Group>
  </form>

  <table
    class="w-full table-auto border-collapse border border-border rounded-md"
  >
    <thead>
      <tr>
        <th class="border border-border p-2 text-xs text-left">From Email</th>
        <th class="border border-border p-2 text-xs text-left">Actions</th>
      </tr>
    </thead>
    <tbody>
      {#each fromEmails as fromEmail (fromEmail.address)}
        <tr>
          <td class="border border-border px-4 py-2">
            {formatFromEmail(fromEmail)}
          </td>
          <td class="border border-border px-4 py-2">
            <Button variant="ghost" size="icon-sm">
              <SquarePen />
            </Button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
{/snippet}
