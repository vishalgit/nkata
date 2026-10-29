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

test('logging out and back in', async ({ page }) => {
  const auth = new AuthPage(page);
  const email = uniqueEmail();
  await auth.register(email);

  await auth.logout();
  await expect(page).toHaveURL(/\/login$/);

  await auth.login(email);
  await expect(page.getByRole('heading', { name: 'My todos' })).toBeVisible();
});

test('a wrong password shows a generic error', async ({ page }) => {
  const auth = new AuthPage(page);
  const email = uniqueEmail();
  await auth.register(email);
  await auth.logout();

  await auth.login(email, 'definitely not it');

  await expect(auth.errorAlert()).toHaveText('invalid email or password');
  await expect(page).toHaveURL(/\/login$/);
});
