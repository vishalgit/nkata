import { test, expect } from '@playwright/test';

test('the app is up', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'Todo' })).toBeVisible();
});
