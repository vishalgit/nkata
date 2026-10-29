import { Page, expect } from '@playwright/test';
import { open } from './helpers';

export class TodosPage {
  constructor(private readonly page: Page) {}

  async visit(): Promise<void> {
    await open(this.page, '/');
  }

  items() {
    return this.page.getByTestId('todo-item');
  }

  item(title: string) {
    return this.items().filter({ hasText: title });
  }

  async add(title: string): Promise<void> {
    await this.page.getByLabel('New todo').fill(title);
    await this.page.getByRole('button', { name: 'Add' }).click();
    await expect(this.item(title)).toBeVisible();
  }

  async complete(title: string): Promise<void> {
    await this.item(title).getByRole('button', { name: 'Done' }).click();
    await expect(this.item(title)).toHaveClass(/todo-item--completed/);
  }

  async reopen(title: string): Promise<void> {
    await this.item(title).getByRole('button', { name: 'Undo' }).click();
    await expect(this.item(title)).not.toHaveClass(/todo-item--completed/);
  }

  async rename(from: string, to: string): Promise<void> {
    const item = this.item(from);
    await item.getByText('Rename').click();
    await item.getByLabel('New title').fill(to);
    await item.getByRole('button', { name: 'Save' }).click();
    await expect(this.item(to)).toBeVisible();
  }

  async remove(title: string): Promise<void> {
    await this.item(title).getByRole('button', { name: 'Delete' }).click();
    await expect(this.item(title)).toHaveCount(0);
  }

  async showOnly(filter: 'All' | 'Active' | 'Completed'): Promise<void> {
    await this.page.getByRole('link', { name: filter, exact: true }).click();
  }

  async search(term: string): Promise<void> {
    await this.page.getByLabel('Search todos').fill(term);
    await this.page.getByRole('button', { name: 'Search' }).click();
    await this.page.waitForURL(/[?&]q=/);
    await this.page.locator('body[data-hydrated]').waitFor();
  }
}
