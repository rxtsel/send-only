import { invoke } from "@tauri-apps/api/core";
import type z from "zod";
import type { profileSchema } from "../schemas/profile.schema";

type Profile = z.infer<typeof profileSchema>;

export async function saveProfile(profile: Profile): Promise<void> {
  await invoke("save_profile_command", profile);
}

export async function getProfile(): Promise<Profile | null> {
  return await invoke<Profile | null>("get_profile");
}
