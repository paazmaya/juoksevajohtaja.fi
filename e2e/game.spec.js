const { test, expect } = require("@playwright/test");

// The game is a separate Trunk build at /peli-app/. On the site it is embedded at /peli (see site.spec.js);
// these tests exercise it standalone.
test.describe("Juokseva Johtaja 2D Running Game UI", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/peli-app/");
  });

  test("removes the running-host loader once the game WebAssembly has started", async ({
    page,
  }) => {
    await expect(page.locator("#game-loader")).toHaveCount(0);
    await expect(page.locator("html")).not.toHaveClass(/embedded/);
  });

  test("loads full-screen canvas and in-game UI with back link and instructions button", async ({
    page,
  }) => {
    const canvas = page.locator("#game-canvas");
    await expect(canvas).toBeVisible();

    const backLink = page.locator("#btn-back-link");
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute("href", "/");

    const instructionsBtn = page.locator("#btn-instructions");
    await expect(instructionsBtn).toBeVisible();

    const stateLabel = page.locator("#game-state-label");
    await expect(stateLabel).toBeVisible();
    await expect(stateLabel).toHaveText("START");

    const distanceValue = page.locator("#distance-value");
    await expect(distanceValue).toHaveText("0");

    const scoreValue = page.locator("#score-value");
    await expect(scoreValue).toHaveText("0");
  });

  test("opens and closes the dimmed About / Instructions overlay", async ({ page }) => {
    const aboutOverlay = page.locator("#about-overlay");
    const instructionsBtn = page.locator("#btn-instructions");
    const closeBtn = page.locator("#btn-close-about");
    const continueBtn = page.locator("#btn-about-continue");

    // Initially hidden
    await expect(aboutOverlay).toHaveClass(/hidden/);

    // Open overlay
    await instructionsBtn.click();
    await expect(aboutOverlay).not.toHaveClass(/hidden/);
    await expect(page.locator("#about-title")).toContainText("Juokseva Johtaja");
    await expect(page.locator(".about-body")).toContainText("Nuoli oikealle");

    // Close overlay with close button
    await closeBtn.click();
    await expect(aboutOverlay).toHaveClass(/hidden/);

    // Reopen and close with continue button
    await instructionsBtn.click();
    await expect(aboutOverlay).not.toHaveClass(/hidden/);
    await continueBtn.click();
    await expect(aboutOverlay).toHaveClass(/hidden/);

    // Reopen and close with ESC key
    await instructionsBtn.click();
    await expect(aboutOverlay).not.toHaveClass(/hidden/);
    await page.keyboard.press("Escape");
    await expect(aboutOverlay).toHaveClass(/hidden/);
  });

  test("runners only move via arrow keys (not automatically)", async ({ page }) => {
    const stateLabel = page.locator("#game-state-label");
    const distanceValue = page.locator("#distance-value");
    const canvas = page.locator("#game-canvas");

    await expect(stateLabel).toHaveText("START");

    // Click canvas to start
    await canvas.click();
    await expect(stateLabel).toHaveText("RUNNING", { timeout: 5000 });

    // With no arrow keys pressed, distance should NOT move automatically
    await page.waitForTimeout(400);
    const initialDist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
    expect(initialDist).toBe(0);

    // Hold ArrowRight: distance should now increase
    await page.keyboard.down("ArrowRight");
    await expect(async () => {
      const currentDist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
      expect(currentDist).toBeGreaterThan(0);
    }).toPass({ timeout: 5000 });

    // Release ArrowRight: distance stops advancing
    await page.keyboard.up("ArrowRight");
    await page.waitForTimeout(200);
    const pausedDist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
    await page.waitForTimeout(300);
    const afterPauseDist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
    expect(afterPauseDist).toBe(pausedDist);
  });

  test("in-game quiz interactive elements exist within the game", async ({ page }) => {
    const quizLayer = page.locator("#quiz-interactive-layer");
    const questionText = page.locator("#quiz-question-text");
    const choiceA = page.locator("#btn-choice-a");
    const choiceB = page.locator("#btn-choice-b");

    // Initially hidden until checkpoint is reached
    await expect(quizLayer).toHaveClass(/hidden/);
    await expect(questionText).toBeAttached();
    await expect(choiceA).toBeAttached();
    await expect(choiceB).toBeAttached();
  });

  test("serves episode 1 level JSON correctly", async ({ page }) => {
    const response = await page.request.get("/peli-app/assets/levels/episode_1_undisclosed.json");
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

  test("on-screen touch buttons for arrows and jump are functional", async ({ page }) => {
    const stateLabel = page.locator("#game-state-label");
    const distanceValue = page.locator("#distance-value");
    const btnRight = page.locator("#btn-touch-right");
    const btnJump = page.locator("#btn-jump-touch");

    await expect(btnRight).toBeVisible();
    await expect(btnJump).toBeVisible();
    await expect(stateLabel).toHaveText("START");

    // Touch right button triggers start and movement
    await btnRight.dispatchEvent("pointerdown");
    await expect(stateLabel).toHaveText("RUNNING", { timeout: 5000 });

    await expect(async () => {
      const dist = Number.parseInt((await distanceValue.textContent()) || "0", 10);
      expect(dist).toBeGreaterThan(0);
    }).toPass({ timeout: 5000 });

    await btnRight.dispatchEvent("pointerup");

    // Touch jump button triggers jump
    await btnJump.click();
    await expect(stateLabel).toHaveText("RUNNING");
  });
});
