const root = document.querySelector("[data-message-path-lab]");

if (root) {
  const nodeIds = ["fire", "church", "water", "ridge", "garage"];
  const defaultPositions = {
    fire: { x: 17, y: 72 },
    church: { x: 34, y: 24 },
    water: { x: 57, y: 55 },
    ridge: { x: 78, y: 20 },
    garage: { x: 83, y: 76 },
  };
  const labels = {
    fire: "Fire station",
    church: "Church steeple",
    water: "Water tower",
    ridge: "Ridgeline",
    garage: "County garage",
  };
  const routes = {
    blocked: ["fire-church", "church-water", "water-garage"],
    direct: ["fire-water", "water-garage"],
  };
  const scenarios = {
    blocked: [
      {
        event: "Route selected through church and water",
        status: "Alternate route selected.",
        header: "RET · ROUTE",
        state: "READY",
        hop: "VIA CHURCH",
        node: "fire",
      },
      {
        event: "Fire station queues the message",
        status: "Fire station queued the message.",
        header: "RET · TRAFFIC",
        state: "TX QUEUED",
        hop: "CHURCH",
        node: "fire",
      },
      {
        event: "Church steeple receives the frame",
        status: "The church steeple received the frame.",
        header: "RET · TRAFFIC",
        state: "RX FRAME",
        hop: "CHURCH",
        node: "church",
        edge: "fire-church",
      },
      {
        event: "Water tower receives the relay",
        status: "The water tower received the relay.",
        header: "RET · TRAFFIC",
        state: "RX FRAME",
        hop: "WATER",
        node: "water",
        edge: "church-water",
      },
      {
        event: "Water tower forwards to the garage",
        status: "The water tower forwarded toward the garage.",
        header: "RET · TRAFFIC",
        state: "TX FRAME",
        hop: "GARAGE",
        node: "garage",
        edge: "water-garage",
      },
      {
        event: "County garage confirms delivery",
        status: "Message delivered by three relays.",
        header: "RET · DELIVERED",
        state: "RX FRAME",
        hop: "GARAGE",
        node: "garage",
      },
    ],
    direct: [
      {
        event: "Direct route selected through the water tower",
        status: "Direct route selected.",
        header: "RET · ROUTE",
        state: "READY",
        hop: "VIA WATER",
        node: "fire",
      },
      {
        event: "Fire station queues the message",
        status: "Fire station queued the message.",
        header: "RET · TRAFFIC",
        state: "TX QUEUED",
        hop: "WATER",
        node: "fire",
      },
      {
        event: "Water tower receives the direct frame",
        status: "The water tower received the direct frame.",
        header: "RET · TRAFFIC",
        state: "RX FRAME",
        hop: "WATER",
        node: "water",
        edge: "fire-water",
      },
      {
        event: "Water tower selects the garage link",
        status: "The water tower selected the garage link.",
        header: "RET · ROUTE",
        state: "FORWARD",
        hop: "GARAGE",
        node: "water",
      },
      {
        event: "Water tower forwards to the garage",
        status: "The water tower forwarded toward the garage.",
        header: "RET · TRAFFIC",
        state: "TX FRAME",
        hop: "GARAGE",
        node: "garage",
        edge: "water-garage",
      },
      {
        event: "County garage confirms delivery",
        status: "Message delivered by the direct route.",
        header: "RET · DELIVERED",
        state: "RX FRAME",
        hop: "GARAGE",
        node: "garage",
      },
    ],
  };

  const stage = root.querySelector("[data-path-stage]");
  const links = root.querySelector("[data-path-links]");
  const packet = root.querySelector("[data-path-packet]");
  const stepInput = root.querySelector("[data-path-step]");
  const blockedInput = root.querySelector("[data-path-blocked]");
  const status = root.querySelector("[data-path-status]");
  const routeLabel = root.querySelector("[data-path-route]");
  const screenHeader = root.querySelector("[data-path-screen-header]");
  const screenCount = root.querySelector("[data-path-screen-count]");
  const screenRows = new Map(
    [...root.querySelectorAll("[data-path-screen-row]")].map((row) => [
      row.dataset.pathScreenRow,
      row,
    ]),
  );
  const nodes = new Map(
    [...root.querySelectorAll("[data-lab-node]")].map((node) => [
      node.dataset.labNode,
      node,
    ]),
  );
  const edges = new Map(
    [...root.querySelectorAll("[data-lab-edge]")].map((edge) => [
      edge.dataset.labEdge,
      edge,
    ]),
  );
  const eventRows = [...root.querySelectorAll("[data-lab-event]")];
  const positions = structuredClone(defaultPositions);
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  let blocked = true;
  let step = 5;
  let playback = null;
  let drag = null;

  function clamp(value, minimum, maximum) {
    return Math.min(maximum, Math.max(minimum, value));
  }

  function stopPlayback() {
    if (playback !== null) {
      window.clearInterval(playback);
      playback = null;
    }
    root.dataset.playing = "false";
  }

  function currentScenario() {
    return blocked ? scenarios.blocked : scenarios.direct;
  }

  function activeRoute() {
    return blocked ? routes.blocked : routes.direct;
  }

  function setNodePosition(id, x, y) {
    positions[id] = {
      x: clamp(Number(x), 7, 93),
      y: clamp(Number(y), 10, 90),
    };
    const node = nodes.get(id);
    node.style.left = `${positions[id].x}%`;
    node.style.top = `${positions[id].y}%`;
    node.dataset.x = positions[id].x.toFixed(1);
    node.dataset.y = positions[id].y.toFixed(1);
  }

  function updateLinks() {
    const stageRect = stage.getBoundingClientRect();
    if (stageRect.width === 0 || stageRect.height === 0) return;
    links.setAttribute("viewBox", `0 0 ${stageRect.width} ${stageRect.height}`);
    for (const edge of edges.values()) {
      const fromRect = nodes.get(edge.dataset.from).getBoundingClientRect();
      const toRect = nodes.get(edge.dataset.to).getBoundingClientRect();
      edge.setAttribute("x1", fromRect.left + fromRect.width / 2 - stageRect.left);
      edge.setAttribute("y1", fromRect.top + fromRect.height / 2 - stageRect.top);
      edge.setAttribute("x2", toRect.left + toRect.width / 2 - stageRect.left);
      edge.setAttribute("y2", toRect.top + toRect.height / 2 - stageRect.top);
    }
    const activeEdge = [...edges.values()].find((edge) =>
      edge.classList.contains("is-active"),
    );
    if (activeEdge) {
      packet.removeAttribute("hidden");
      packet.setAttribute(
        "cx",
        (Number(activeEdge.getAttribute("x1")) +
          Number(activeEdge.getAttribute("x2"))) /
          2,
      );
      packet.setAttribute(
        "cy",
        (Number(activeEdge.getAttribute("y1")) +
          Number(activeEdge.getAttribute("y2"))) /
          2,
      );
    } else {
      packet.setAttribute("hidden", "hidden");
    }
  }

  function render(announce = true) {
    const scenario = currentScenario();
    const event = scenario[step];
    const route = activeRoute();
    root.dataset.blocked = String(blocked);
    root.dataset.step = String(step);
    blockedInput.checked = blocked;
    stepInput.value = String(step);
    root.querySelector("[data-path-step-output]").textContent = `${step + 1} of ${scenario.length}`;
    routeLabel.textContent = blocked
      ? "Reroute · fire → church → water → garage"
      : "Direct · fire → water → garage";

    for (const [id, edge] of edges) {
      edge.classList.toggle("is-route", route.includes(id));
      edge.classList.toggle("is-blocked", id === "fire-water" && blocked);
      edge.classList.toggle("is-active", event.edge === id);
    }
    for (const [id, node] of nodes) {
      node.classList.toggle("is-active", event.node === id);
      node.setAttribute(
        "aria-label",
        `${labels[id]}. Drag or use arrow keys to move this radio.`,
      );
    }

    eventRows.forEach((row, index) => {
      row.querySelector("[data-path-event-copy]").textContent =
        scenario[index].event;
      row.classList.toggle("is-current", index === step);
      row.classList.toggle("is-complete", index < step);
      if (index === step) row.setAttribute("aria-current", "step");
      else row.removeAttribute("aria-current");
    });

    screenHeader.textContent = event.header;
    screenCount.textContent = `${step + 1}/${scenario.length}`;
    screenRows.get("state").textContent = event.state;
    screenRows.get("hop").textContent = event.hop;
    screenRows.get("sequence").textContent = String(step).padStart(2, "0");
    screenRows.get("host").textContent = "ATTACHED";
    if (announce) status.textContent = event.status;
    updateLinks();
  }

  function setStep(nextStep, announce = true) {
    step = clamp(Math.round(Number(nextStep)), 0, currentScenario().length - 1);
    render(announce);
  }

  function play() {
    stopPlayback();
    if (reducedMotion.matches) {
      setStep(5);
      status.textContent = `${currentScenario()[5].status} Motion is reduced.`;
      return;
    }
    setStep(0);
    root.dataset.playing = "true";
    playback = window.setInterval(() => {
      if (step >= currentScenario().length - 1) {
        stopPlayback();
        return;
      }
      setStep(step + 1);
    }, 650);
  }

  function parseScene() {
    const params = new URLSearchParams(window.location.hash.slice(1));
    if (params.get("message-path") !== "v1") return;
    blocked = params.get("blocked") !== "0";
    const parsedStep = Number(params.get("step"));
    if (Number.isInteger(parsedStep)) step = clamp(parsedStep, 0, 5);
    const serializedPositions = params.get("positions") ?? "";
    for (const item of serializedPositions.split("|")) {
      const [id, rawX, rawY] = item.split(",");
      const x = Number(rawX);
      const y = Number(rawY);
      if (nodeIds.includes(id) && Number.isFinite(x) && Number.isFinite(y)) {
        setNodePosition(id, x, y);
      }
    }
  }

  async function shareScene() {
    stopPlayback();
    const params = new URLSearchParams(window.location.hash.slice(1));
    params.set("message-path", "v1");
    params.set("blocked", blocked ? "1" : "0");
    params.set("step", String(step));
    params.set(
      "positions",
      nodeIds
        .map(
          (id) =>
            `${id},${positions[id].x.toFixed(1)},${positions[id].y.toFixed(1)}`,
        )
        .join("|"),
    );
    window.history.replaceState(null, "", `#${params.toString()}`);
    try {
      await navigator.clipboard.writeText(window.location.href);
      status.textContent = "Scene link copied.";
    } catch {
      status.textContent = "Scene link is ready in the address bar.";
    }
  }

  root.querySelector('[data-path-action="send"]').addEventListener("click", play);
  root
    .querySelector('[data-path-action="previous"]')
    .addEventListener("click", () => {
      stopPlayback();
      setStep(step - 1);
    });
  root.querySelector('[data-path-action="next"]').addEventListener("click", () => {
    stopPlayback();
    setStep(step + 1);
  });
  root
    .querySelector('[data-path-action="share"]')
    .addEventListener("click", shareScene);
  stepInput.addEventListener("input", () => {
    stopPlayback();
    setStep(stepInput.value);
  });
  blockedInput.addEventListener("change", () => {
    stopPlayback();
    blocked = blockedInput.checked;
    setStep(0);
  });

  for (const [id, node] of nodes) {
    node.addEventListener("pointerdown", (event) => {
      if (event.button !== 0) return;
      stopPlayback();
      event.preventDefault();
      node.setPointerCapture(event.pointerId);
      drag = { id, pointerId: event.pointerId };
      node.classList.add("is-dragging");
    });
    node.addEventListener("pointermove", (event) => {
      if (!drag || drag.id !== id || drag.pointerId !== event.pointerId) return;
      const rect = stage.getBoundingClientRect();
      setNodePosition(
        id,
        ((event.clientX - rect.left) / rect.width) * 100,
        ((event.clientY - rect.top) / rect.height) * 100,
      );
      updateLinks();
    });
    node.addEventListener("pointerup", (event) => {
      if (!drag || drag.id !== id || drag.pointerId !== event.pointerId) return;
      node.releasePointerCapture(event.pointerId);
      node.classList.remove("is-dragging");
      drag = null;
      status.textContent = `${labels[id]} moved. Share the scene to preserve it.`;
    });
    node.addEventListener("keydown", (event) => {
      const movement = {
        ArrowLeft: [-1, 0],
        ArrowRight: [1, 0],
        ArrowUp: [0, -1],
        ArrowDown: [0, 1],
      }[event.key];
      if (!movement) return;
      event.preventDefault();
      stopPlayback();
      const distance = event.shiftKey ? 5 : 2;
      setNodePosition(
        id,
        positions[id].x + movement[0] * distance,
        positions[id].y + movement[1] * distance,
      );
      updateLinks();
      status.textContent = `${labels[id]} moved.`;
    });
  }

  for (const id of nodeIds) {
    setNodePosition(id, positions[id].x, positions[id].y);
  }
  parseScene();
  root.dataset.ready = "true";
  root.dataset.playing = "false";
  render(false);
  new ResizeObserver(updateLinks).observe(stage);
  window.addEventListener("visibilitychange", () => {
    if (document.hidden) stopPlayback();
  });
}
