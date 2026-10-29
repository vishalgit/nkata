import { Page } from '@playwright/test';

export const PASSWORD = 'correct horse battery';

export function uniqueEmail(prefix = 'user'): string {
  return `${prefix}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}@example.com`;
}

/** Navigate and wait until Leptos has hydrated the page. */
export async function open(page: Page, path: string): Promise<void> {
  await page.goto(path);
  await page.locator('body[data-hydrated]').waitFor();
}
