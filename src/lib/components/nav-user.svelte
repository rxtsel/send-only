<script lang="ts">
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import LogOutIcon from "@lucide/svelte/icons/log-out";
  import SparklesIcon from "@lucide/svelte/icons/sparkles";

  import * as Avatar from "@/lib/components/ui/avatar/index.js";
  import * as DropdownMenu from "@/lib/components/ui/dropdown-menu/index.js";
  import * as Sidebar from "@/lib/components/ui/sidebar/index.js";
  import { useSidebar } from "@/lib/components/ui/sidebar/index.js";
  import type { Profile } from "../types";
  import { getProfile } from "../commom/profile";
  import { onMount } from "svelte";

  const sidebar = useSidebar();

  let user = $state<Profile>({
    firstName: "Send",
    lastName: "Only",
    username: "send.only",
    domain: "sendonly.example",
  });

  onMount(async () => {
    const userData = await getProfile();
    if (userData) {
      user = userData;
    }
  });
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Sidebar.MenuButton
            {...props}
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground md:h-8 md:p-0"
          >
            <Avatar.Root class="size-8 rounded-lg">
              <Avatar.Image
                src="/icon-512.png"
                alt="{user.firstName} {user.lastName}"
              />
              <Avatar.Fallback class="rounded-lg">
                {user.firstName.charAt(0).toUpperCase() ?? "O"}{user.lastName
                  .charAt(0)
                  .toUpperCase() ?? "S"}
              </Avatar.Fallback>
            </Avatar.Root>
            <div class="grid flex-1 text-start text-sm leading-tight">
              <span class="truncate font-medium"
                >{user.firstName} {user.lastName}</span
              >
              <span class="truncate text-xs">{user.username}</span>
            </div>
            <ChevronsUpDownIcon class="ms-auto size-4" />
          </Sidebar.MenuButton>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content
        class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
        side={sidebar.isMobile ? "bottom" : "right"}
        align="end"
        sideOffset={4}
      >
        <DropdownMenu.Label class="p-0 font-normal">
          <div class="flex items-center gap-2 px-1 py-1.5 text-start text-sm">
            <Avatar.Root class="size-8 rounded-lg">
              <Avatar.Image
                src={user.username}
                alt="{user.firstName} {user.lastName}"
              />
              <Avatar.Fallback class="rounded-lg">
                {user.firstName.charAt(0) ?? "O"}{user.lastName.charAt(0) ??
                  "S"}
              </Avatar.Fallback>
            </Avatar.Root>
            <div class="grid flex-1 text-start text-sm leading-tight">
              <span class="truncate font-medium"
                >{user.firstName} {user.lastName}</span
              >
              <span class="truncate text-xs">{user.username}</span>
            </div>
          </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Group>
          <DropdownMenu.Item class="p-0 m-0">
            <a
              href="https://donate.stripe.com/00wdR8dOd0YF7Ipce0a7C04"
              target="_blank"
              rel="noopener noreferrer"
              class="flex w-full item-center gap-2 py-2 px-2"
            >
              <SparklesIcon />
              Support us
            </a>
          </DropdownMenu.Item>
        </DropdownMenu.Group>
        <DropdownMenu.Separator />
        <DropdownMenu.Item>
          <LogOutIcon />
          Log out
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Sidebar.MenuItem>
</Sidebar.Menu>
