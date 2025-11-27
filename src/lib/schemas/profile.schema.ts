import z from "zod/v4";

export const profileSchema = z.object({
  firstName: z.string().nonempty("First Name is required"),
  lastName: z.string().nonempty("Last Name is required"),
  username: z
    .string()
    .nonempty("Username is required")
    .min(3, "Username must be at least 3 characters")
    .max(30, "Username must be at most 30 characters")
    .refine((v) => {
      const usernamePattern = /^[a-zA-Z0-9_]+$/;
      return usernamePattern.test(v);
    }, "Username can only contain letters, numbers, and underscores"),
  domain: z
    .string()
    .nonempty("Domain is required")
    .refine((v) => {
      // Wiouth protocol
      const domainPattern = /^(?!-)(?:[a-zA-Z0-9-]{1,63}\.)+[a-zA-Z]{2,63}$/;
      return domainPattern.test(v);
    }, "Must be a valid domain (e.g., yourdomain.com)"),
});
