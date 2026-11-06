const { test, expect } = require("@playwright/test");

test.describe("Juokseva Johtaja Site & Game Navigation", () => {
  test("homepage loads and contains a working link to /game/", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(/Juokseva Johtaja/);

    const gameLink = page.locator('a[href="/game/"]');
    await expect(gameLink).toBeVisible();
    await expect(gameLink).toContainText("Pelaa podcast-juoksupeliä");

    await gameLink.click();
    await expect(page).toHaveURL(/\/game\/?/);
    await expect(page).toHaveTitle(/Juokseva Johtaja - Podcast Runner Game/);
  });
});

test.describe("Juokseva Johtaja 2D Running Game UI", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/game/");
  });

  test("loads game canvas and controls header", async ({ page }) => {
    const canvas = page.locator("#game-canvas");
    await expect(canvas).toBeVisible();

    const backLink = page.locator('header a[href="/"]');
    await expect(backLink).toBeVisible();

    const stateLabel = page.locator("#game-state-label");
    await expect(stateLabel).toBeVisible();
    await expect(stateLabel).toHaveText("START");

    const distanceValue = page.locator("#distance-value");
    await expect(distanceValue).toHaveText("0");

    const scoreValue = page.locator("#score-value");
    await expect(scoreValue).toHaveText("0");
  });

  test("starts running on space key press and updates distance/score", async ({ page }) => {
    const stateLabel = page.locator("#game-state-label");
    const distanceValue = page.locator("#distance-value");
    const canvas = page.locator("#game-canvas");

    await expect(stateLabel).toHaveText("START");

    // Click canvas or press Space to start
    await canvas.click();
    await page.keyboard.press("Space");

    // State changes to RUNNING
    await expect(stateLabel).toHaveText("RUNNING", { timeout: 5000 });

    // Distance should increase as the runner moves forward
    await expect(async () => {
      const dist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
      expect(dist).toBeGreaterThan(0);
    }).toPass({ timeout: 5000 });
  });

  test("quiz overlay modal elements exist and are accessible", async ({ page }) => {
    const quizOverlay = page.locator("#quiz-overlay");
    const questionText = page.locator("#quiz-question-text");
    const choiceA = page.locator("#btn-choice-a");
    const choiceB = page.locator("#btn-choice-b");

    // Initially hidden until checkpoint is reached
    await expect(quizOverlay).toHaveClass(/hidden/);
    await expect(questionText).toBeAttached();
    await expect(choiceA).toBeAttached();
    await expect(choiceB).toBeAttached();
  });

  test("serves episode 1 level JSON correctly", async ({ page }) => {
    const response = await page.request.get("/game/assets/levels/episode_1_undisclosed.json");
    expect(response.ok()).toBeTruthy();

    const level = await response.json();
    expect(level.id).toBe("1-1-undisclosed");
    expect(level.trackLength).toBeGreaterThan(0);
    expect(level.checkpoints.length).toBeGreaterThanOrEqual(1);

    for (const cp of level.checkpoints) {
      expect(cp.options.length).toBe(2);
      const correctOptions = cp.options.filter((o) => o.isCorrect);
      expect(correctOptions.length).toBe(1);
    }
  });

  test("touch jump button is functional", async ({ page }) => {
    const jumpButton = page.locator("#btn-jump-touch");
    await expect(jumpButton).toBeVisible();

    const stateLabel = page.locator("#game-state-label");
    await expect(stateLabel).toHaveText("START");

    await jumpButton.click();
    await expect(stateLabel).toHaveText("RUNNING", { timeout: 5000 });
  });
});
