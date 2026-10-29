import { test, expect } from '@playwright/test';
import { AuthPage } from './pages/auth-page';
import { uniqueEmail } from './pages/helpers';

test('registering lands you on your empty board', async ({ page }) => {
  const auth = new AuthPage(page);
  const email = uniqueEmail();

  await auth.register(email);

  await expect(page.getByText(email)).toBeVisible(); // the header
  await expect(page.getByText('Nothing to do')).toBeVisible();
});
