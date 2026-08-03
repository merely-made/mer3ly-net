import assert from "node:assert/strict";
import { createServer } from "node:http";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";

import { chromium } from "playwright";

const siteRoot = path.resolve(process.env.MER3LY_SITE_DIR ?? "html");
const receiptRoot = path.resolve(
  process.env.MER3LY_RECEIPT_DIR ?? ".tmp/browser-smoke",
);
const headless = process.env.MER3LY_HEADLESS !== "false";
const mimeTypes = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".jpg": "image/jpeg",
  ".js": "text/javascript; charset=utf-8",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".txt": "text/plain; charset=utf-8",
  ".wasm": "application/wasm",
  ".xml": "application/xml; charset=utf-8",
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
  schema: "mer3ly.browser-smoke-receipt/v3",
  source_sha: process.env.GITHUB_SHA ?? "local",
  browser: `Chromium ${browser.version()}`,
  mode: headless ? "headless" : "headed",
  routes: {},
  desktop: {},
  mobile: {},
  reduced_motion: {},
  fallback: {},
  showcase: {},
  projects: {},
  discovery: {},
};

try {
  const sitemapResponse = await fetch(`${baseUrl}/sitemap.xml`);
  assert.equal(sitemapResponse.status, 200);
  assert.match(
    sitemapResponse.headers.get("content-type") ?? "",
    /^application\/xml/,
  );
  const sitemapText = await sitemapResponse.text();
  const sitemapUrls = [...sitemapText.matchAll(/<loc>([^<]+)<\/loc>/g)].map(
    (match) => match[1],
  );
  assert.equal(sitemapUrls.length, 22);
  assert.equal(new Set(sitemapUrls).size, 22);
  assert.equal(
    sitemapUrls.every((url) => url.startsWith("https://mer3ly.net/")),
    true,
  );
  for (const unsupported of ["lastmod", "changefreq", "priority"]) {
    assert.equal(sitemapText.includes(unsupported), false);
  }

  const robotsResponse = await fetch(`${baseUrl}/robots.txt`);
  assert.equal(robotsResponse.status, 200);
  assert.match(
    robotsResponse.headers.get("content-type") ?? "",
    /^text\/plain/,
  );
  assert.equal(
    await robotsResponse.text(),
    "User-agent: *\nAllow: /\nSitemap: https://mer3ly.net/sitemap.xml\n",
  );

  const faviconResponse = await fetch(`${baseUrl}/favicon.svg`);
  assert.equal(faviconResponse.status, 200);
  assert.match(
    faviconResponse.headers.get("content-type") ?? "",
    /^image\/svg\+xml/,
  );
  assert.ok((await faviconResponse.arrayBuffer()).byteLength > 0);
  receipt.discovery = {
    sitemap_urls: sitemapUrls.length,
    robots_policy: "allow-public",
    favicon: "favicon.svg",
  };

  for (const route of [
    "/",
    "/radio.html",
    "/projects/mere/",
    "/projects/mesocosm/",
  ]) {
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

  const showcaseDesktop = await browser.newPage({
    viewport: { width: 1440, height: 1000 },
  });
  const showcaseDesktopDiagnostics = collectDiagnostics(showcaseDesktop);
  await showcaseDesktop.goto(`${baseUrl}/`, { waitUntil: "networkidle" });
  assert.equal(
    await showcaseDesktop.locator(".home-showcase-card").count(),
    5,
  );
  const showcaseImages = showcaseDesktop.locator(".home-showcase-figure img");
  for (const image of await showcaseImages.all()) {
    await image.scrollIntoViewIfNeeded();
  }
  await showcaseDesktop.waitForFunction(() =>
    [...document.querySelectorAll(".home-showcase-figure img")].every(
      (image) => image.complete,
    ),
  );
  assert.equal(
    await showcaseImages.evaluateAll((images) =>
      images.every(
        (image) =>
          image.complete && image.naturalWidth > 0 && image.naturalHeight > 0,
      ),
    ),
    true,
    "desktop showcase images did not decode",
  );
  assert.equal(await horizontalOverflow(showcaseDesktop), 0);
  assert.deepEqual(
    showcaseDesktopDiagnostics,
    [],
    "desktop showcase emitted browser errors",
  );
  await showcaseDesktop.screenshot({
    path: path.join(receiptRoot, "home-showcase-desktop.png"),
    fullPage: true,
  });
  receipt.showcase.desktop = {
    cards: 5,
    images: 5,
    horizontal_overflow: 0,
  };
  await showcaseDesktop.close();

  const showcaseMobile = await browser.newPage({
    viewport: { width: 375, height: 812 },
  });
  const showcaseMobileDiagnostics = collectDiagnostics(showcaseMobile);
  await showcaseMobile.goto(`${baseUrl}/`, { waitUntil: "networkidle" });
  const mobileShowcaseImages = showcaseMobile.locator(
    ".home-showcase-figure img",
  );
  for (const image of await mobileShowcaseImages.all()) {
    await image.scrollIntoViewIfNeeded();
  }
  await showcaseMobile.waitForFunction(() =>
    [...document.querySelectorAll(".home-showcase-figure img")].every(
      (image) => image.complete,
    ),
  );
  assert.equal(
    await mobileShowcaseImages.evaluateAll((images) =>
      images.every(
        (image) =>
          image.complete && image.naturalWidth > 0 && image.naturalHeight > 0,
      ),
    ),
    true,
    "mobile showcase images did not decode",
  );
  assert.equal(await horizontalOverflow(showcaseMobile), 0);
  assert.deepEqual(
    showcaseMobileDiagnostics,
    [],
    "mobile showcase emitted browser errors",
  );
  await showcaseMobile.screenshot({
    path: path.join(receiptRoot, "home-showcase-mobile.png"),
    fullPage: true,
  });
  receipt.showcase.mobile = {
    cards: await showcaseMobile.locator(".home-showcase-card").count(),
    horizontal_overflow: 0,
  };
  await showcaseMobile.close();

  const visualProject = await browser.newPage({
    viewport: { width: 1200, height: 900 },
  });
  const visualProjectDiagnostics = collectDiagnostics(visualProject);
  await visualProject.goto(`${baseUrl}/projects/mere/`, {
    waitUntil: "networkidle",
  });
  assert.equal(
    await visualProject.locator("[data-project-id]").getAttribute(
      "data-project-id",
    ),
    "mere",
  );
  assert.equal(
    await visualProject.locator(".project-showcase-figure img").count(),
    1,
  );
  const visualMetadata = await projectMetadata(visualProject);
  assert.equal(
    visualMetadata.social_image,
    "https://mer3ly.net/showcase/mere.png",
  );
  assert.equal(visualMetadata.social_image_type, "image/png");
  assert.equal(visualMetadata.twitter_image, visualMetadata.social_image);
  assert.equal(
    visualMetadata.twitter_image_alt,
    visualMetadata.social_image_alt,
  );
  assert.ok(visualMetadata.social_image_alt.length > 0);
  assert.equal(visualMetadata.structured_type, "SoftwareSourceCode");
  assert.equal(
    visualMetadata.code_repository,
    "https://github.com/merely-made/mere",
  );
  assert.equal(await horizontalOverflow(visualProject), 0);
  assert.deepEqual(
    visualProjectDiagnostics,
    [],
    "visual project profile emitted browser errors",
  );
  await visualProject.screenshot({
    path: path.join(receiptRoot, "project-mere-desktop.png"),
    fullPage: true,
  });
  receipt.projects.visual = {
    repository: "mere",
    showcase_images: 1,
    social_image: visualMetadata.social_image,
    structured_type: visualMetadata.structured_type,
    horizontal_overflow: 0,
  };
  await visualProject.close();

  const textProject = await browser.newPage({
    viewport: { width: 375, height: 812 },
  });
  const textProjectDiagnostics = collectDiagnostics(textProject);
  await textProject.goto(`${baseUrl}/projects/mesocosm/`, {
    waitUntil: "networkidle",
  });
  assert.equal(
    await textProject.locator("[data-project-id]").getAttribute(
      "data-project-id",
    ),
    "mesocosm",
  );
  assert.equal(
    await textProject.locator(".project-showcase-figure").count(),
    0,
  );
  assert.equal(
    await textProject
      .locator(".project-no-image-copy")
      .getByText("intentionally text-first")
      .count(),
    1,
  );
  const textMetadata = await projectMetadata(textProject);
  assert.equal(textMetadata.social_image, "https://mer3ly.net/og.jpg");
  assert.equal(textMetadata.social_image_type, "image/jpeg");
  assert.equal(textMetadata.twitter_image, textMetadata.social_image);
  assert.equal(textMetadata.twitter_image_alt, textMetadata.social_image_alt);
  assert.ok(textMetadata.social_image_alt.length > 0);
  assert.equal(textMetadata.structured_type, "SoftwareSourceCode");
  assert.equal(
    textMetadata.code_repository,
    "https://github.com/merely-made/mesocosm",
  );
  assert.equal(await horizontalOverflow(textProject), 0);
  assert.deepEqual(
    textProjectDiagnostics,
    [],
    "text-only project profile emitted browser errors",
  );
  await textProject.screenshot({
    path: path.join(receiptRoot, "project-mesocosm-mobile.png"),
    fullPage: true,
  });
  receipt.projects.text_only = {
    repository: "mesocosm",
    showcase_images: 0,
    social_image: textMetadata.social_image,
    structured_type: textMetadata.structured_type,
    horizontal_overflow: 0,
  };
  await textProject.close();

  const desktop = await browser.newPage({
    viewport: { width: 1440, height: 900 },
  });
  const desktopDiagnostics = collectDiagnostics(desktop);
  const desktopResponse = await desktop.goto(`${baseUrl}/repos/`, {
    waitUntil: "networkidle",
  });
  assert.equal(desktopResponse?.status(), 200);
  await waitForGraphState(desktop);
  let desktopState = await graphState(desktop);
  assert.equal(desktopState.repositories, 19);
  assert.equal(desktopState.relation_text_projections, 50);
  assert.equal(desktopState.graph_nodes, 19);
  assert.equal(desktopState.graph_edges, 25);
  assert.equal(desktopState.horizontal_overflow, 0);
  assert.ok(
    desktopState.state === "ready" || desktopState.state === "unavailable",
    "graph did not settle into ready or fallback state",
  );

  let selectedProfile = "fallback-not-applicable";
  if (desktopState.state === "ready") {
    const mere = desktop.locator('[data-graph-node-id="mere"]');
    assert.equal(await mere.getAttribute("aria-label"), "Mere, platform, active");
    try {
      const arrangementPicker = desktop.locator("[data-graph-arrangement]");
      assert.equal(await arrangementPicker.locator("option").count(), 8);
      assert.equal(
        await arrangementPicker.locator("option:not(:disabled)").count(),
        7,
      );
      await desktop.waitForFunction(() =>
        [...document.querySelectorAll("[data-graph-node-id]")].every(
          (node) => node.style.left && node.style.top,
        ),
      );
      const beforeArrangement = await graphNodePositions(desktop);
      await arrangementPicker.selectOption("graph_layout:grid");
      await desktop.waitForFunction(
        () =>
          document.querySelector("[data-repository-graph]").dataset
            .graphMorphing === "false" &&
          document.querySelector("[data-repository-graph]").dataset
            .graphArrangement === "graph_layout:grid",
      );
      const afterArrangement = await graphNodePositions(desktop);
      assert.equal(
        beforeArrangement.some(
          (before, index) =>
            Math.hypot(
              before.x - afterArrangement[index].x,
              before.y - afterArrangement[index].y,
            ) > 8,
        ),
        true,
        "arrangement selection did not move repository nodes",
      );
      assert.equal(
        await desktop
          .locator("[data-repository-graph]")
          .getAttribute("data-graph-node-form"),
        "tile",
      );
      assert.match(
        await desktop.locator("[data-graph-scene-caption]").textContent(),
        /Index tiles/,
      );
      assert.equal(await mere.getAttribute("aria-pressed"), "true");
      desktopState.arrangements = 7;
      desktopState.morphed_to = "graph_layout:grid";
      await mere.click({ timeout: 2000 });
      await mere.press("ArrowRight", { timeout: 2000 });
      const selectedNode = desktop.locator(
        ".repository-graph-node.is-selected",
      );
      const selectedId = await selectedNode.getAttribute("data-graph-node-id", {
        timeout: 2000,
      });
      assert.ok(selectedId);
      assert.notEqual(selectedId, "mere");
      await desktop.locator("[data-repository-graph]").screenshot({
        path: path.join(receiptRoot, "desktop-repository-graph.png"),
      });
      await selectedNode.press("Enter", { timeout: 2000 });
      await desktop.waitForURL(`**/projects/${selectedId}/`);
      assert.equal(
        await desktop
          .locator("[data-project-id]")
          .getAttribute("data-project-id"),
        selectedId,
      );
      selectedProfile = selectedId;
    } catch (error) {
      desktopState = await graphState(desktop);
      if (desktopState.state !== "unavailable") {
        throw error;
      }
      await desktop.locator("[data-repository-graph]").screenshot({
        path: path.join(receiptRoot, "desktop-repository-graph.png"),
      });
    }
  } else {
    await desktop.locator("[data-repository-graph]").screenshot({
      path: path.join(receiptRoot, "desktop-repository-graph.png"),
    });
  }
  assert.deepEqual(desktopDiagnostics, [], "desktop emitted browser errors");
  receipt.desktop = { ...desktopState, selected_profile: selectedProfile };
  await desktop.close();

  const mobile = await browser.newPage({
    viewport: { width: 420, height: 900 },
  });
  const mobileDiagnostics = collectDiagnostics(mobile);
  await mobile.goto(`${baseUrl}/repos/`, { waitUntil: "networkidle" });
  await waitForGraphState(mobile);
  const mobileState = await graphState(mobile);
  assert.equal(mobileState.repositories, 19);
  assert.equal(mobileState.horizontal_overflow, 0);
  if (mobileState.state === "ready") {
    const arrangementPicker = mobile.locator("[data-graph-arrangement]");
    const arrangementScenes = [
      ["graph_layout:radial", "medallion", "orbits"],
      ["graph_layout:grid", "tile", "index"],
      ["graph_layout:phyllotaxis", "seed", "field"],
      ["graph_layout:timeline", "flag", "timeline"],
      ["graph_layout:kanban", "card", "lanes"],
      ["graph_layout:penrose", "facet", "tessellation"],
      ["graph_layout:lsystem", "leaf", "branches"],
    ];
    const sceneReceipts = [];
    for (const [arrangementId, nodeForm, scaffold] of arrangementScenes) {
      await arrangementPicker.selectOption(arrangementId);
      await mobile.waitForFunction(
        (expected) => {
          const root = document.querySelector("[data-repository-graph]");
          return (
            root.dataset.graphArrangement === expected &&
            root.dataset.graphMorphing === "false"
          );
        },
        arrangementId,
      );
      const scene = await graphSceneState(mobile);
      assert.equal(scene.nodes, 19);
      assert.equal(scene.outside_stage, 0);
      assert.equal(scene.outside_node_bounds, 0);
      assert.equal(scene.selected, "mere");
      assert.equal(scene.node_form, nodeForm);
      assert.equal(scene.scaffold, scaffold);
      if (["orbits", "timeline", "lanes", "tessellation", "branches"].includes(scaffold)) {
        assert.ok(scene.scaffold_items > 0, `${arrangementId} has no scene scaffold`);
      }
      assert.ok(
        scene.minimum_distance >= 28,
        `${arrangementId} crowded repository nodes on mobile`,
      );
      if (arrangementId === "graph_layout:timeline") {
        assert.ok(scene.minimum_hit_width >= 34, "timeline targets are too narrow");
        assert.ok(scene.minimum_hit_height >= 44, "timeline targets are too short");
        assert.equal(scene.overlapping_nodes, 0, "timeline targets overlap");

        const genet = mobile.locator('[data-graph-node-id="genet"]');
        await genet.click({ position: { x: 2, y: 2 } });
        await expectSelectedNode(mobile, "genet");

        const mere = mobile.locator('[data-graph-node-id="mere"]');
        await mere.click({ position: { x: 2, y: 2 } });
        await expectSelectedNode(mobile, "mere");
      }
      sceneReceipts.push(scene);
    }
    mobileState.arrangements = sceneReceipts;
  }
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
    await reduced
      .locator("[data-graph-arrangement]")
      .selectOption("graph_layout:penrose");
    assert.equal(
      await reduced
        .locator("[data-repository-graph]")
        .getAttribute("data-graph-morphing"),
      "false",
    );
    assert.equal(
      await reduced
        .locator("[data-repository-graph]")
        .getAttribute("data-graph-arrangement"),
      "graph_layout:penrose",
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
  assert.equal(fallbackState.repositories, 19);
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

async function expectSelectedNode(page, expectedId) {
  await page.waitForFunction(
    (id) =>
      document.querySelector(".repository-graph-node.is-selected")?.dataset
        .graphNodeId === id,
    expectedId,
  );
}

async function graphNodePositions(page) {
  return page.locator("[data-graph-node-id]").evaluateAll((nodes) =>
    nodes.map((node) => ({
      x: Number.parseFloat(node.style.left),
      y: Number.parseFloat(node.style.top),
    })),
  );
}

async function graphSceneState(page) {
  return page.evaluate(() => {
    const root = document.querySelector("[data-repository-graph]");
    const stage = document.querySelector("[data-graph-stage]");
    const stageRect = stage.getBoundingClientRect();
    const points = [...document.querySelectorAll("[data-graph-node-id]")].map(
      (node) => ({
        x: Number.parseFloat(node.style.left),
        y: Number.parseFloat(node.style.top),
      }),
    );
    const nodeRects = [...document.querySelectorAll("[data-graph-node-id]")].map(
      (node) => node.getBoundingClientRect(),
    );
    let minimumDistance = Number.POSITIVE_INFINITY;
    let overlappingNodes = 0;
    for (let index = 0; index < points.length; index += 1) {
      for (let other = index + 1; other < points.length; other += 1) {
        minimumDistance = Math.min(
          minimumDistance,
          Math.hypot(
            points[index].x - points[other].x,
            points[index].y - points[other].y,
          ),
        );
        const a = nodeRects[index];
        const b = nodeRects[other];
        if (
          Math.min(a.right, b.right) - Math.max(a.left, b.left) > 1 &&
          Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 1
        ) {
          overlappingNodes += 1;
        }
      }
    }
    return {
      arrangement: root.dataset.graphArrangement,
      node_form: root.dataset.graphNodeForm,
      scaffold: root.dataset.graphScaffold,
      scaffold_items: document.querySelector("[data-graph-scene]").children.length,
      nodes: points.length,
      minimum_distance: Math.round(minimumDistance),
      minimum_hit_width: Math.round(
        Math.min(...nodeRects.map((rect) => rect.width)),
      ),
      minimum_hit_height: Math.round(
        Math.min(...nodeRects.map((rect) => rect.height)),
      ),
      overlapping_nodes: overlappingNodes,
      outside_stage: points.filter(
        (point) =>
          point.x < 0 ||
          point.y < 0 ||
          point.x > stageRect.width ||
          point.y > stageRect.height,
      ).length,
      outside_node_bounds: nodeRects.filter(
        (rect) =>
          rect.left < stageRect.left ||
          rect.top < stageRect.top ||
          rect.right > stageRect.right ||
          rect.bottom > stageRect.bottom,
      ).length,
      selected: document.querySelector(".repository-graph-node.is-selected")
        ?.dataset.graphNodeId,
    };
  });
}

async function projectMetadata(page) {
  return page.evaluate(() => {
    const canonical = document.querySelector('link[rel="canonical"]').href;
    const payload = JSON.parse(
      document.querySelector('script[type="application/ld+json"]').textContent,
    );
    const entity = payload["@graph"].find(
      (node) => node["@id"] === `${canonical}#repository`,
    );
    return {
      social_image: document
        .querySelector('meta[property="og:image"]')
        .getAttribute("content"),
      social_image_type: document
        .querySelector('meta[property="og:image:type"]')
        .getAttribute("content"),
      social_image_alt: document
        .querySelector('meta[property="og:image:alt"]')
        .getAttribute("content"),
      twitter_image: document
        .querySelector('meta[name="twitter:image"]')
        .getAttribute("content"),
      twitter_image_alt: document
        .querySelector('meta[name="twitter:image:alt"]')
        .getAttribute("content"),
      structured_type: entity["@type"],
      code_repository: entity.codeRepository,
    };
  });
}
