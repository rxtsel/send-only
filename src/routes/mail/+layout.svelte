<script lang="ts">
  import { page } from "$app/state";
  import AppSidebar from "@/lib/components/app-sidebar.svelte";
  import * as Breadcrumb from "@/lib/components/ui/breadcrumb";
  import { Separator } from "@/lib/components/ui/separator";
  import * as Sidebar from "@/lib/components/ui/sidebar";
  import { Send } from "@lucide/svelte";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import { buttonVariants } from "$lib/components/ui/button";
  import { useSidebar } from "@/lib/components/ui/sidebar/context.svelte.js";
  import { goto } from "$app/navigation";

  let { children } = $props();

  let isConfirmDialogOpen = $state(false);
  const sidebar = useSidebar();

  const pathname = $derived(page.url.pathname);

  // Route helpers
  const isComposer = $derived(pathname === "/mail/composer");
  const isSentRoot = $derived(pathname === "/mail/sent");
  const isSentDetail = $derived(pathname.startsWith("/mail/sent/"));

  // Extract only the id when we are on /mail/sent/:id
  const emailId = $derived(
    isSentDetail ? pathname.slice("/mail/sent/".length) : "",
  );
</script>

<Sidebar.Provider style="--sidebar-width: 450px;">
  <AppSidebar />
  <Sidebar.Inset>
    <header
      class="bg-background z-1 fixed w-full top-0 flex shrink-0 items-center gap-2 border-b px-4 py-4.5 max-h-[65px] h-[65px]"
    >
      {#if !isComposer}
        <Sidebar.Trigger class="-ms-1" />

        <Separator
          orientation="vertical"
          class="me-2 data-[orientation=vertical]:h-4"
        />

        <Breadcrumb.Root>
          <Breadcrumb.List>
            <Breadcrumb.Item class="hidden md:block">
              {#if isSentRoot}
                <!-- On /mail/sent show it as current page -->
                <Breadcrumb.Page>All sent</Breadcrumb.Page>
              {:else}
                <!-- On /mail/sent/:id use it as link -->
                <Breadcrumb.Link href="/mail/sent">All sent</Breadcrumb.Link>
              {/if}
            </Breadcrumb.Item>

            {#if isSentDetail && emailId}
              <Breadcrumb.Separator class="hidden md:block" />
              <Breadcrumb.Item>
                <!-- Only show the id, nothing else -->
                <Breadcrumb.Page>
                  ...{emailId.slice(-4)}
                </Breadcrumb.Page>
              </Breadcrumb.Item>
            {/if}
          </Breadcrumb.List>
        </Breadcrumb.Root>
      {:else}
        <div class="flex w-full justify-between items-center pr-13 gap-x-2">
          <h2 class="text-lg font-semibold">New Email</h2>

          {@render ConfirmDialog()}
        </div>
      {/if}
    </header>

    <div class="flex flex-1 flex-col gap-4 pb-4 pt-20 px-4">
      {@render children()}
    </div>
  </Sidebar.Inset>
</Sidebar.Provider>

{#snippet ConfirmDialog()}
  <AlertDialog.Root bind:open={isConfirmDialogOpen}>
    <AlertDialog.Trigger class={buttonVariants({ variant: "default" })}>
      <Send />
      Send
    </AlertDialog.Trigger>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
        <AlertDialog.Description>
          This action cannot be undone. This will permanently send your email.
        </AlertDialog.Description>
      </AlertDialog.Header>
      <AlertDialog.Footer>
        <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
        <AlertDialog.Action
          form="compose-email-form"
          type="submit"
          onclick={() => (isConfirmDialogOpen = false)}
        >
          Continue
        </AlertDialog.Action>
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{/snippet}
