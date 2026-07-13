import { expect, test } from '@playwright/test';

test('reviews the bundled curated export without browser network or persistence', async ({ page }) => {
  const remoteRequests: string[] = [];
  const failedResponses: string[] = [];
  const pageErrors: Error[] = [];
  page.on('request', request => {
    const url = new URL(request.url());
    if (url.hostname !== '127.0.0.1') remoteRequests.push(request.url());
  });
  page.on('response', response => {
    if (response.status() >= 400) failedResponses.push(`${response.status()} ${response.url()}`);
  });
  page.on('pageerror', error => pageErrors.push(error));

  await page.goto('./');

  await expect(page.locator('link[rel="icon"]')).toHaveAttribute('href', /^data:image\/svg\+xml/);
  await expect(page.getByRole('heading', { level: 1 })).toContainText('raison visible');
  await expect(page.locator('#item-title')).not.toBeEmpty();
  await expect(page.getByLabel('Décision de curation')).toContainText('saved');

  const liveProof = process.env.FEED_RADAR_EXPECT_LIVE === '1';
  if (liveProof) {
    await expect(page.locator('main')).toHaveAttribute('data-review-mode', 'live-sync');
    await expect(page.getByText(/synchronisation publique bornée/)).toBeVisible();
  } else {
    await expect(page.getByRole('heading', { name: /Rust-first local feed curation/ })).toBeVisible();
    await expect(page.getByText('Keep Rust sovereignty articles')).toBeVisible();
    await expect(page.getByText('Title matches pattern')).toBeVisible();
  }

  const disclosure = page.getByText('Examiner la preuve technique');
  await disclosure.focus();
  await page.keyboard.press('Enter');
  if (liveProof) {
    await expect(page.locator('.proof code').first()).toHaveText(/^sha256:[0-9a-f]{64}$/);
  } else {
    await expect(page.getByText('sha256:ef16a2af327c962a100b861bdf452bcde87fe1c8ecbf9a48e83ddce6e6b69b46')).toBeVisible();
  }

  const disclosureBox = await disclosure.boundingBox();
  expect(disclosureBox?.height ?? 0).toBeGreaterThanOrEqual(44);
  const dimensions = await page.evaluate(() => ({
    scrollWidth: document.documentElement.scrollWidth,
    clientWidth: document.documentElement.clientWidth,
    local: localStorage.length,
    session: sessionStorage.length,
  }));
  expect(dimensions.scrollWidth).toBeLessThanOrEqual(dimensions.clientWidth);
  expect(dimensions.local).toBe(0);
  expect(dimensions.session).toBe(0);
  expect(await page.evaluate(async () => (await navigator.serviceWorker?.getRegistrations() ?? []).length)).toBe(0);
  const stylesheets = await page.evaluate(() =>
    [...document.styleSheets].map(sheet => sheet.href ?? '').filter(Boolean),
  );
  for (const expected of ['tokens', 'themes', 'components', 'styles']) {
    expect(stylesheets.some(href => href.includes(expected))).toBe(true);
  }
  expect(remoteRequests).toEqual([]);
  expect(failedResponses).toEqual([]);
  expect(pageErrors).toEqual([]);
});
