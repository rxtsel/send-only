<script lang="ts" module>
  import SendIcon from "@lucide/svelte/icons/send";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SettingsDialog from "./settings-dialog.svelte";
</script>

<script lang="ts">
  import NavUser from "./nav-user.svelte";
  import { useSidebar } from "@/lib/components/ui/sidebar/context.svelte.js";
  import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
  import { Button } from "@/lib/components/ui/button/index.js";
  import type { ComponentProps } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import type { SentEmail } from "../types";
  import { formatEmailDate, listSentEmails } from "../commom/sent";
  import { Loader } from "@lucide/svelte";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();

  const data = {
    navMain: [
      {
        title: "Sent",
        url: "/mail/sent",
        icon: SendIcon,
        isActive: true,
      },
    ],
  };

  let activeItem = $state(data.navMain[0]);
  let mails = $state<SentEmail[]>([]);
  let isLoadingEmails = $state(true);
  let isRefreshing = $state(false);
  let isLoadingMore = $state(false);
  let hasMore = $state(true);
  let currentPage = $state(0);

  const sidebar = useSidebar();
  const PAGE_SIZE = 16;

  async function loadSentEmails(append = false) {
    isLoadingEmails = true;
    try {
      if (append) {
        isLoadingMore = true;
      } else {
        isRefreshing = true;
        currentPage = 0;
      }

      const page = append ? currentPage + 1 : 0;
      const offset = page * PAGE_SIZE;

      console.log(
        `Loading emails - Page: ${page}, Offset: ${offset}, Append: ${append}`,
      );

      const newMails = await listSentEmails(PAGE_SIZE, offset);

      console.log(`Received ${newMails.length} emails`);

      if (append) {
        mails = [...mails, ...newMails];
        currentPage = page;
      } else {
        mails = newMails;
        currentPage = 0;
      }

      // Check if there are more emails
      hasMore = newMails.length === PAGE_SIZE;
    } catch (error) {
      console.error("Error loading sent emails:", error);
      toast.error("Failed to load sent emails");
    } finally {
      isRefreshing = false;
      isLoadingMore = false;
      isLoadingEmails = false;
    }
  }

  async function handleRefresh() {
    await loadSentEmails(false);
    toast.success("Emails refreshed");
  }

  async function handleLoadMore() {
    if (!isLoadingMore && hasMore) {
      await loadSentEmails(true);
    }
  }

  function handleEmailClick(mailId: string) {
    goto(`/mail/sent/${mailId}`);
  }

  // Export function to be called from composer after sending
  export function refreshEmails() {
    return loadSentEmails(false);
  }

  onMount(() => {
    loadSentEmails(false);

    // Listen for email sent events
    const handleEmailSent = () => {
      loadSentEmails(false);
    };

    window.addEventListener("email-sent", handleEmailSent);

    return () => {
      window.removeEventListener("email-sent", handleEmailSent);
    };
  });
</script>

<Sidebar.Root
  bind:ref
  collapsible="icon"
  class="overflow-hidden *:data-[sidebar=sidebar]:flex-row"
  {...restProps}
>
  <!-- This is the first sidebar -->
  <Sidebar.Root
    collapsible="none"
    class="w-[calc(var(--sidebar-width-icon)+1px)]! border-e"
  >
    <Sidebar.Header>
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <Sidebar.MenuButton
            variant="primary"
            size="lg"
            class="size-8 justify-center rounded-lg transition-colors p-0"
          >
            <a
              href="/mail/composer"
              class="flex h-full w-full items-center justify-center"
              onclick={() => {
                if (sidebar.open) sidebar.setOpen(false);
              }}
            >
              <PlusIcon class="text-white size-5" />
            </a>
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
      <Sidebar.Group>
        <Sidebar.GroupContent class="px-1.5 md:px-0">
          <Sidebar.Menu>
            {#each data.navMain as item (item.title)}
              <Sidebar.MenuItem>
                <Sidebar.MenuButton
                  tooltipContentProps={{
                    hidden: false,
                  }}
                  onclick={() => {
                    activeItem = item;
                    sidebar.setOpen(true);
                    goto(item.url);
                  }}
                  isActive={activeItem.title === item.title}
                  class="px-2.5 md:px-2"
                >
                  {#snippet tooltipContent()}
                    {item.title}
                  {/snippet}
                  <item.icon />
                  <span>{item.title}</span>
                </Sidebar.MenuButton>
              </Sidebar.MenuItem>
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    </Sidebar.Content>
    <Sidebar.Footer>
      <SettingsDialog />
      <NavUser />
    </Sidebar.Footer>
  </Sidebar.Root>

  <!-- This is the second sidebar -->
  <Sidebar.Root collapsible="none" class="hidden flex-1 md:flex">
    <Sidebar.Header class="gap-3.5 border-b p-4">
      <div class="flex w-full items-center justify-between">
        <div class="text-foreground text-base font-medium">
          {activeItem.title}
        </div>
        <Button
          variant="ghost"
          size="icon-sm"
          onclick={handleRefresh}
          disabled={isRefreshing}
          title="Refresh emails"
        >
          <RefreshCwIcon class={isRefreshing ? "animate-spin" : ""} />
        </Button>
      </div>
    </Sidebar.Header>
    <Sidebar.Content class="overflow-y-auto overflow-x-hidden max-w-[400px]">
      <Sidebar.Group class="px-0">
        <Sidebar.GroupContent>
          {#if mails.length === 0 && !isRefreshing}
            <div class="p-8 text-center text-sm text-muted-foreground">
              No sent emails yet
            </div>
          {:else if isLoadingEmails}
            {@const skeletons = Array.from({ length: PAGE_SIZE })}
            {#each skeletons}
              <div class="animate-pulse p-4 border-b last:border-b-0">
                <div class="h-4 bg-muted w-3/4 mb-2 rounded"></div>
                <div class="h-3 bg-muted w-1/2 rounded"></div>
              </div>
            {/each}
          {:else}
            {#each mails as mail (mail.id)}
              {@const isActive = page.params.id === mail.id}
              <button
                onclick={() => handleEmailClick(mail.id)}
                class="hover:bg-sidebar-accent hover:text-sidebar-accent-foreground flex w-full flex-col items-start gap-2 whitespace-nowrap border-b p-4 text-sm leading-tight last:border-b-0 text-left {isActive
                  ? 'bg-sidebar-accent'
                  : ''}"
              >
                <div class="flex w-full items-center gap-2">
                  <span class="font-semibold truncate">{mail.to[0]}</span>
                  <span class="ms-auto text-xs shrink-0">
                    {formatEmailDate(mail.created_at)}
                  </span>
                </div>
                <span class="text-muted-foreground truncate w-full"
                  >{mail.subject}</span
                >
              </button>
            {/each}

            {#if hasMore}
              <button
                onclick={handleLoadMore}
                disabled={isLoadingMore}
                class="w-full p-4 text-sm text-center text-muted-foreground hover:bg-sidebar-accent transition-colors"
              >
                {#if isLoadingMore}
                  <Loader class="size-4 animate-spin mx-auto" />
                {:else}
                  <span>Load more</span>
                {/if}
              </button>
            {/if}
          {/if}
        </Sidebar.GroupContent>
      </Sidebar.Group>
    </Sidebar.Content>
  </Sidebar.Root>
</Sidebar.Root>
