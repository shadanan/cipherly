import { ZodError, z } from "zod";

// Checkout https://github.com/colinhacks/zod for example validation logic
export const AuthEncryptFormSchema = z.object({
  emails: z.array(z.string().email().endsWith("@gmail.com")),
  plainText: z.string().min(1),
});
export type AuthEncryptFormData = z.infer<typeof AuthEncryptFormSchema>;

export function hasError(
  validationError: ZodError | undefined,
  property: string,
): boolean {
  if (!validationError || validationError.isEmpty) return false;
  return validationError.issues.some((e) => e.path.includes(property));
}

export function getError(
  validationError: ZodError | undefined,
  property: string,
): string | null {
  if (!validationError || validationError.isEmpty) return null;
  const error = validationError.issues.find((e) => e.path.includes(property));
  if (!error) return null;
  return error.message;
}
