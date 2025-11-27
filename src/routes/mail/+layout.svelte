<script lang="ts">
  import { page } from "$app/state";
  import AppSidebar from "@/lib/components/app-sidebar.svelte";
  import * as Breadcrumb from "@/lib/components/ui/breadcrumb";
  import Button from "@/lib/components/ui/button/button.svelte";
  import { Separator } from "@/lib/components/ui/separator";
  import * as Sidebar from "@/lib/components/ui/sidebar";

  let { children } = $props();
</script>

<Sidebar.Provider style="--sidebar-width: 350px;">
  <AppSidebar />
  <Sidebar.Inset>
    <header
      class="bg-background sticky top-0 flex shrink-0 items-center gap-2 border-b p-4"
    >
      {#if page.url.pathname !== "/mail/composer"}
        <Sidebar.Trigger class="-ms-1" />

        <Separator
          orientation="vertical"
          class="me-2 data-[orientation=vertical]:h-4"
        />
        <Breadcrumb.Root>
          <Breadcrumb.List>
            <Breadcrumb.Item class="hidden md:block">
              <Breadcrumb.Link href="##">All Inboxes</Breadcrumb.Link>
            </Breadcrumb.Item>
            <Breadcrumb.Separator class="hidden md:block" />
            <Breadcrumb.Item>
              <Breadcrumb.Page>Inbox</Breadcrumb.Page>
            </Breadcrumb.Item>
          </Breadcrumb.List>
        </Breadcrumb.Root>
      {:else}
        <Button form="compose-email-form" type="submit" class="w-fit ml-auto">
          Send
        </Button>
      {/if}
    </header>
    <div class="flex flex-1 flex-col gap-4 p-4">
      {@render children()}
    </div>
  </Sidebar.Inset>
</Sidebar.Provider>
