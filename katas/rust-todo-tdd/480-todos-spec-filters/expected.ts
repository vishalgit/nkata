import { test, expect } from '@playwright/test';
import { AuthPage } from './pages/auth-page';
import { TodosPage } from './pages/todos-page';
import { uniqueEmail } from './pages/helpers';

test.beforeEach(async ({ page }) => {
  await new AuthPage(page).register(uniqueEmail());
});

test('the full lifecycle of a todo', async ({ page }) => {
  const todos = new TodosPage(page);

  await todos.add('Buy milk');
  await todos.complete('Buy milk');
  await todos.reopen('Buy milk');
  await todos.rename('Buy milk', 'Buy oat milk');
  await todos.remove('Buy oat milk');

  await expect(todos.items()).toHaveCount(0);
});

test('todos survive a full page reload', async ({ page }) => {
  const todos = new TodosPage(page);
  await todos.add('Persist me');

  await todos.visit();

  await expect(todos.item('Persist me')).toBeVisible();
});

test('filters show the right subset', async ({ page }) => {
  const todos = new TodosPage(page);
  await todos.add('Open one');
  await todos.add('Finished one');
  await todos.complete('Finished one');

  await todos.showOnly('Active');
  await expect(todos.items()).toHaveCount(1);
  await expect(todos.item('Open one')).toBeVisible();

  await todos.showOnly('Completed');
  await expect(todos.items()).toHaveCount(1);
  await expect(todos.item('Finished one')).toBeVisible();

  await todos.showOnly('All');
  await expect(todos.items()).toHaveCount(2);
});
