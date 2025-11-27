import z from "zod/v4";

export const emailOptionSchema = z.object({
  label: z.string().nonempty("Label required"),
  address: z.email("Invalid email"),
});
