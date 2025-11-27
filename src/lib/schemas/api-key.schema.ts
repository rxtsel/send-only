import z from "zod/v4";

export const apiKeySchema = z
  .string()
  .nonempty("API Key is required")
  .refine((v) => v.startsWith("re_"), {
    message: "API Key must start with 're_'",
  })
  .length(36, "API Key must be exactly 36 characters long");
