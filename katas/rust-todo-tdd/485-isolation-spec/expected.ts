import { test, expect } from '@playwright/test';
import { AuthPage } from './pages/auth-page';
import { TodosPage } from './pages/todos-page';
import { uniqueEmail } from './pages/helpers';

test("one user never sees another user's todos", async ({ browser }) => {
  const alice = await browser.newContext(); // separate cookie jars
  const bob = await browser.newContext();
  const alicePage = await alice.newPage();
  const bobPage = await bob.newPage();

  await new AuthPage(alicePage).register(uniqueEmail('alice'));
  await new TodosPage(alicePage).add('Alice secret');

  await new AuthPage(bobPage).register(uniqueEmail('bob'));
  const bobTodos = new TodosPage(bobPage);
  await bobTodos.visit();

  await expect(bobTodos.item('Alice secret')).toHaveCount(0);
  await alice.close();
  await bob.close();
});
