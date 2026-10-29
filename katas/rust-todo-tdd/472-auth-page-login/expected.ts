import { Page, expect } from '@playwright/test';
import { PASSWORD, open } from './helpers';

export class AuthPage {
  constructor(private readonly page: Page) {}

  async register(email: string, password = PASSWORD): Promise<void> {
    await open(this.page, '/register');
    await this.page.getByLabel('Email').fill(email);
    await this.page.getByLabel(/Password/).fill(password);
    await this.page.getByRole('button', { name: 'Register' }).click();
    await expect(this.page.getByRole('heading', { name: 'My todos' })).toBeVisible();
  }

  async login(email: string, password = PASSWORD): Promise<void> {
    await open(this.page, '/login');
    await this.page.getByLabel('Email').fill(email);
    await this.page.getByLabel('Password').fill(password);
    await this.page.getByRole('button', { name: 'Log in' }).click();
  }

  async logout(): Promise<void> {
    await this.page.getByRole('button', { name: 'Log out' }).click();
  }

  errorAlert() {
    return this.page.getByRole('alert');
  }
}
