const { test, expect } = require("@playwright/test");

const EPISODE_PATH = "/jaksot/undisclosed";
const EPISODE_TITLE = "Undisclosed | Juokseva Johtaja";

/** Silent 8-bit mono WAV, long enough to seek to the highlights. */
function silentWav(seconds = 1200, sampleRate = 4000) {
  const samples = seconds * sampleRate;
  const buffer = Buffer.alloc(44 + samples, 128);
  buffer.write("RIFF", 0);
  buffer.writeUInt32LE(36 + samples, 4);
  buffer.write("WAVEfmt ", 8);
  buffer.writeUInt32LE(16, 16);
  buffer.writeUInt16LE(1, 20);
  buffer.writeUInt16LE(1, 22);
  buffer.writeUInt32LE(sampleRate, 24);
  buffer.writeUInt32LE(sampleRate, 28);
  buffer.writeUInt16LE(1, 32);
  buffer.writeUInt16LE(8, 34);
  buffer.write("data", 36);
  buffer.writeUInt32LE(samples, 40);
  return buffer;
}

const AUDIO = silentWav();

/** Answers like an audio CDN, including HTTP Range requests (needed for seeking). */
function fulfillAudio(route) {
  const range = /bytes=(\d+)-(\d*)/.exec(route.request().headers().range || "");
  if (!range) {
    return route.fulfill({
      body: AUDIO,
      contentType: "audio/wav",
      headers: { "Accept-Ranges": "bytes" },
    });
  }
  const start = Number(range[1]);
  const end = range[2] ? Math.min(Number(range[2]), AUDIO.length - 1) : AUDIO.length - 1;
  return route.fulfill({
    status: 206,
    body: AUDIO.subarray(start, end + 1),
    contentType: "audio/wav",
    headers: { "Accept-Ranges": "bytes", "Content-Range": `bytes ${start}-${end}/${AUDIO.length}` },
  });
}

/** Episode audio is hosted elsewhere; serve a local fixture instead. Analytics is blocked. */
async function stubExternal(page) {
  await page.route(/\.mp3(\?.*)?$/, fulfillAudio);
  await page.route(/mc\.yandex\.ru/, (route) => route.abort());
}

async function ready(page) {
  await expect(page.locator("body")).toHaveClass(/is-ready/);
}

function audioState(page) {
  return page
    .locator("audio")
    .evaluate((audio) => ({ paused: audio.paused, time: audio.currentTime }));
}

test.beforeEach(async ({ page }) => {
  await stubExternal(page);
});

test.describe("Pre-rendered pages (search engines, social previews, no JavaScript)", () => {
  test.use({ javaScriptEnabled: false });

  test("episode page is complete HTML with SEO metadata", async ({ page }) => {
    const response = await page.goto(EPISODE_PATH);
    expect(response.status()).toBe(200);
    await expect(page).toHaveTitle(EPISODE_TITLE);
    await expect(page.locator("h1")).toHaveText("Undisclosed");
    await expect(page.locator(".guest-card")).toContainText("Vieras");
    await expect(page.locator(".highlight-list__item")).toHaveCount(3);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute(
      "href",
      `https://juoksevajohtaja.fi${EPISODE_PATH}`,
    );
    await expect(page.locator('meta[property="og:type"]')).toHaveAttribute("content", "article");
    const jsonLd = JSON.parse(
      await page.locator('script[type="application/ld+json"]').textContent(),
    );
    expect(jsonLd["@type"]).toBe("PodcastEpisode");
    expect(jsonLd.associatedMedia.contentUrl).toMatch(/\.mp3$/);
  });

  test("home page lists episodes and shows the running-host loader", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator(".episode-card")).not.toHaveCount(0);
    await expect(page.locator("#boot-loader .runner")).toBeVisible();
    await expect(page.locator("#boot-loader")).toContainText("Ladataan");
  });

  test("unknown paths get the pre-rendered 404 page", async ({ page }) => {
    const response = await page.goto("/ei-tallaista-sivua");
    expect(response.status()).toBe(404);
    await expect(page.locator("h1")).toHaveText("Sivua ei löytynyt");
    await expect(page.locator('meta[name="robots"]')).toHaveAttribute("content", "noindex");
  });

  test("feed, sitemap and robots.txt are published", async ({ page }) => {
    const feed = await page.request.get("/feed.xml");
    expect(feed.ok()).toBeTruthy();
    const xml = await feed.text();
    expect(xml).toContain("<itunes:author>Jukka Paasonen</itunes:author>");
    expect(xml).toContain("<enclosure ");
    expect(await (await page.request.get("/sitemap.xml")).text()).toContain(
      `https://juoksevajohtaja.fi${EPISODE_PATH}`,
    );
    expect(await (await page.request.get("/robots.txt")).text()).toContain(
      "Sitemap: https://juoksevajohtaja.fi/sitemap.xml",
    );
  });
});

test.describe("Single-page application", () => {
  test("shows the running host while the WebAssembly loads, then the live app", async ({
    page,
  }) => {
    await page.route(/\.wasm$/, async (route) => {
      await new Promise((resolve) => setTimeout(resolve, 1500));
      await route.continue();
    });
    await page.goto("/");
    await expect(page.locator("#boot-loader .runner")).toBeVisible();
    await expect(page.locator(".home-hero__title")).toBeVisible();
    await ready(page);
    await expect(page.locator("#boot-loader")).toHaveCount(0);
  });

  test("episode card opens the episode without a full page reload", async ({ page }) => {
    await page.goto("/");
    await ready(page);
    await page.evaluate(() => {
      window.__sameDocument = true;
    });
    await page.locator(".episode-card__link").first().click();
    await expect(page).toHaveURL(new RegExp(`${EPISODE_PATH}$`));
    await expect(page).toHaveTitle(EPISODE_TITLE);
    expect(await page.evaluate(() => window.__sameDocument)).toBe(true);
  });

  test("deep link to an episode boots the app on that page with one set of head tags", async ({
    page,
  }) => {
    await page.goto(EPISODE_PATH);
    await ready(page);
    await expect(page.locator("h1")).toHaveText("Undisclosed");
    await expect(page).toHaveTitle(EPISODE_TITLE);
    await expect(page.locator('meta[name="description"]')).toHaveCount(1);
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
    await expect(page.locator(".site-header__link[aria-current=page]")).toHaveText("Jaksot");
  });

  test("podcast keeps playing while browsing and playing the game", async ({ page }) => {
    await page.goto(EPISODE_PATH);
    await ready(page);
    await page.locator(".episode-page .play-button").click();
    await expect(page.locator(".audio-player__bar")).toBeVisible();
    await expect.poll(async () => (await audioState(page)).time).toBeGreaterThan(0.2);

    await page.locator(".site-header__brand").click();
    await expect(page).toHaveURL(/\/$/);
    await page.locator('.site-header__link[href="/peli"]').click();
    await expect(page.locator(".game-page__iframe")).toBeVisible();

    const before = await audioState(page);
    expect(before.paused).toBe(false);
    await expect.poll(async () => (await audioState(page)).time).toBeGreaterThan(before.time + 0.5);
    await expect(page.locator(".audio-player__title")).toHaveText("Undisclosed");
  });

  test("timestamped highlight starts the episode from that moment", async ({ page }) => {
    await page.goto(EPISODE_PATH);
    await ready(page);
    await page.locator(".highlight-list__time", { hasText: "15:30" }).click();
    await expect.poll(async () => (await audioState(page)).time).toBeGreaterThanOrEqual(930);
    expect((await audioState(page)).paused).toBe(false);
  });

  test("player controls pause and skip", async ({ page }) => {
    await page.goto(EPISODE_PATH);
    await ready(page);
    await page.locator(".highlight-list__time", { hasText: "2:00" }).click();
    await expect.poll(async () => (await audioState(page)).time).toBeGreaterThanOrEqual(120);
    await page.getByRole("button", { name: "30 sekuntia eteenpäin" }).click();
    await expect.poll(async () => (await audioState(page)).time).toBeGreaterThanOrEqual(150);
    await page.getByRole("button", { name: "Toista tai tauko" }).click();
    await expect.poll(async () => (await audioState(page)).paused).toBe(true);
    await expect(page.locator(".episode-page .play-button")).toHaveAttribute(
      "aria-pressed",
      "false",
    );
  });

  test("game WebAssembly is downloaded only when the game page is opened", async ({ page }) => {
    const gameRequests = [];
    page.on("request", (request) => {
      if (request.url().includes("/peli-app/")) {
        gameRequests.push(request.url());
      }
    });
    await page.goto("/");
    await ready(page);
    await page.goto(EPISODE_PATH);
    await ready(page);
    expect(gameRequests).toEqual([]);

    await page.locator('.site-header__link[href="/peli"]').click();
    await expect.poll(() => gameRequests.some((url) => url.endsWith(".wasm"))).toBe(true);
  });

  test("embedded game hides its own back link and runs inside the site", async ({ page }) => {
    await page.goto("/peli");
    await ready(page);
    const game = page.frameLocator(".game-page__iframe");
    await expect(game.locator("#game-canvas")).toBeVisible();
    await expect(game.locator("#btn-back-link")).toBeHidden();
    await expect(game.locator("#game-loader")).toHaveCount(0);
    await expect(page.locator(".game-page .runner-loader")).toHaveCount(0);
  });
});
