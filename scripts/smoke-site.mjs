import assert from "node:assert/strict";
import { createServer } from "node:http";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";

import { chromium } from "playwright";

const siteRoot = path.resolve(process.env.MER3LY_SITE_DIR ?? "html");
const receiptRoot = path.resolve(
  process.env.MER3LY_RECEIPT_DIR ?? ".tmp/m6-headed",
);
const headless = process.env.MER3LY_HEADLESS !== "false";
const mimeTypes = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".jpg": "image/jpeg",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
};

await mkdir(receiptRoot, { recursive: true });

const server = createServer(async (request, response) => {
  try {
    const url = new URL(request.url, "http://127.0.0.1");
    let pathname = decodeURIComponent(url.pathname);
    if (pathname === "/") pathname = "/index.html";
    if (pathname === "/favicon.ico") {
      response.writeHead(204);
      response.end();
      return;
    }
    if (pathname === "/radio" || pathname === "/radio/") {
      pathname = "/radio.html";
    }
    if (pathname.endsWith("/")) pathname += "index.html";
    const candidate = path.resolve(siteRoot, `.${pathname}`);
    const rootPrefix = `${siteRoot}${path.sep}`;
    if (candidate !== siteRoot && !candidate.startsWith(rootPrefix)) {
      response.writeHead(403);
      response.end("forbidden");
      return;
    }
    const bytes = await readFile(candidate);
    response.writeHead(200, {
      "Cache-Control": "no-store",
      "Content-Type":
        mimeTypes[path.extname(candidate)] ?? "application/octet-stream",
    });
    response.end(bytes);
  } catch {
    response.writeHead(404);
    response.end("not found");
  }
});

await new Promise((resolve, reject) => {
  server.once("error", reject);
  server.listen(0, "127.0.0.1", resolve);
});

const port = server.address().port;
const baseUrl = `http://127.0.0.1:${port}`;
const browser = await chromium.launch({
  channel: "chromium",
  headless,
  args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader"],
});

const receipt = {
  schema: "mer3ly.browser-smoke-receipt/v1",
  source_sha: process.env.GITHUB_SHA ?? "local",
  browser: `Chromium ${browser.version()}`,
  mode: headless ? "headless" : "headed",
  routes: {},
  desktop: {},
  mobile: {},
  reduced_motion: {},
  fallback: {},
};

try {
  for (const route of ["/", "/radio.html"]) {
    const page = await browser.newPage({ viewport: { width: 900, height: 900 } });
    const diagnostics = collectDiagnostics(page);
    const response = await page.goto(`${baseUrl}${route}`, {
      waitUntil: "networkidle",
    });
    assert.equal(response?.status(), 200, `${route} did not return 200`);
    assert.equal(await page.locator("h1").count(), 1, `${route} needs one h1`);
    assert.equal(await horizontalOverflow(page), 0, `${route} overflowed`);
    assert.deepEqual(diagnostics, [], `${route} emitted browser errors`);
    receipt.routes[route] = { status: 200, horizontal_overflow: 0 };
    await page.close();
  }

  const desktop = await browser.newPage({
    viewport: { width: 1440, height: 900 },
  });
  const desktopDiagnostics = collectDiagnostics(desktop);
  const desktopResponse = await desktop.goto(`${baseUrl}/repos/`, {
    waitUntil: "networkidle",
  });
  assert.equal(desktopResponse?.status(), 200);
  await waitForGraphState(desktop);
  const desktopState = await graphState(desktop);
  assert.equal(desktopState.repositories, 16);
  assert.equal(desktopState.relation_text_projections, 50);
  assert.equal(desktopState.graph_nodes, 16);
  assert.equal(desktopState.graph_edges, 25);
  assert.equal(desktopState.horizontal_overflow, 0);
  assert.ok(
    desktopState.state === "ready" || desktopState.state === "unavailable",
    "graph did not settle into ready or fallback state",
  );

  if (desktopState.state === "ready") {
    const mere = desktop.getByRole("button", {
      name: "Mere, platform, active",
    });
    await mere.click();
    await mere.press("ArrowRight");
    assert.equal(
      await desktop
        .locator(".repository-graph-node.is-selected")
        .getAttribute("data-graph-node-id"),
      "retinue",
    );
    await desktop
      .getByRole("button", { name: "Retinue, platform, active" })
      .press("Enter");
    assert.equal(
      await desktop.evaluate(() => document.activeElement?.id),
      "repo-retinue",
    );
  }
  assert.deepEqual(desktopDiagnostics, [], "desktop emitted browser errors");
  await desktop.locator("[data-repository-graph]").screenshot({
    path: path.join(receiptRoot, "desktop-repository-graph.png"),
  });
  receipt.desktop = desktopState;
  await desktop.close();

  const mobile = await browser.newPage({
    viewport: { width: 420, height: 900 },
  });
  const mobileDiagnostics = collectDiagnostics(mobile);
  await mobile.goto(`${baseUrl}/repos/`, { waitUntil: "networkidle" });
  await waitForGraphState(mobile);
  const mobileState = await graphState(mobile);
  assert.equal(mobileState.repositories, 16);
  assert.equal(mobileState.horizontal_overflow, 0);
  assert.deepEqual(mobileDiagnostics, [], "mobile emitted browser errors");
  await mobile.locator("[data-repository-graph]").screenshot({
    path: path.join(receiptRoot, "mobile-repository-graph.png"),
  });
  receipt.mobile = mobileState;
  await mobile.close();

  const reduced = await browser.newPage({
    viewport: { width: 420, height: 900 },
  });
  await reduced.emulateMedia({ reducedMotion: "reduce" });
  const reducedDiagnostics = collectDiagnostics(reduced);
  await reduced.goto(`${baseUrl}/repos/`, { waitUntil: "networkidle" });
  await waitForGraphState(reduced);
  const reducedState = await graphState(reduced);
  const reducedMediaMatches = await reduced.evaluate(() =>
    window.matchMedia("(prefers-reduced-motion: reduce)").matches,
  );
  assert.equal(reducedMediaMatches, true);
  if (reducedState.state === "ready") {
    assert.equal(
      await reduced
        .locator("[data-repository-graph]")
        .getAttribute("data-reduced-motion"),
      "true",
    );
  }
  assert.equal(reducedState.horizontal_overflow, 0);
  assert.deepEqual(
    reducedDiagnostics,
    [],
    "reduced-motion path emitted browser errors",
  );
  receipt.reduced_motion = {
    graph_state: reducedState.state,
    horizontal_overflow: reducedState.horizontal_overflow,
    media_query_matches: reducedMediaMatches,
    graph_client_acknowledged:
      reducedState.state === "ready" ? true : "fallback-not-applicable",
  };
  await reduced.close();

  const fallback = await browser.newPage({
    viewport: { width: 375, height: 812 },
  });
  const fallbackDiagnostics = collectDiagnostics(fallback);
  await fallback.goto(`${baseUrl}/repos/?graph=no-webgpu`, {
    waitUntil: "networkidle",
  });
  await waitForGraphState(fallback);
  const fallbackState = await graphState(fallback);
  assert.equal(fallbackState.state, "unavailable");
  assert.equal(fallbackState.repositories, 16);
  assert.equal(fallbackState.horizontal_overflow, 0);
  assert.equal(
    await fallback
      .locator("[data-graph-interface]")
      .evaluate((element) => element.hidden),
    true,
  );
  assert.deepEqual(
    fallbackDiagnostics,
    [],
    "forced fallback emitted browser errors",
  );
  await fallback.screenshot({
    path: path.join(receiptRoot, "webgpu-fallback.png"),
    fullPage: true,
  });
  receipt.fallback = fallbackState;
  await fallback.close();

  await writeFile(
    path.join(receiptRoot, "receipt.json"),
    `${JSON.stringify(receipt, null, 2)}\n`,
    "utf8",
  );
  process.stdout.write(
    `${headless ? "browser" : "headed"} smoke accepted: ${receipt.desktop.repositories} repositories, ${receipt.desktop.graph_edges} graph edges\n`,
  );
} finally {
  await browser.close();
  await new Promise((resolve) => server.close(resolve));
}

function collectDiagnostics(page) {
  const diagnostics = [];
  page.on("pageerror", () => diagnostics.push("pageerror"));
  page.on("console", (message) => {
    if (message.type() === "error") {
      if (process.env.MER3LY_DEBUG_DIAGNOSTICS === "1") {
        process.stderr.write(`browser console error: ${message.text()}\n`);
      }
      diagnostics.push("console-error");
    }
  });
  return diagnostics;
}

async function waitForGraphState(page) {
  await page.waitForFunction(() => {
    const state = document.querySelector("[data-repository-graph]")?.dataset
      .graphState;
    return state === "ready" || state === "unavailable";
  });
}

async function horizontalOverflow(page) {
  return page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
}

async function graphState(page) {
  return page.evaluate(() => {
    const payload = JSON.parse(
      document.querySelector("#repository-graph-data").textContent,
    );
    return {
      state: document.querySelector("[data-repository-graph]").dataset
        .graphState,
      repositories: document.querySelectorAll("[data-repository-id]").length,
      relation_text_projections: document.querySelectorAll("[data-relation-id]")
        .length,
      graph_nodes: payload.nodes.length,
      graph_edges: payload.edges.length,
      horizontal_overflow:
        document.documentElement.scrollWidth -
        document.documentElement.clientWidth,
    };
  });
}
