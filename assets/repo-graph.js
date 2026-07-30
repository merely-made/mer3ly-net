import initWasm, { layout_graph as layoutGraph } from "./mer3ly_repo_graph.js";

class GraphUnavailable extends Error {}

const root = document.querySelector("[data-repository-graph]");

if (root) {
  startRepositoryGraph(root).catch((error) => {
    showFallback(
      root,
      error instanceof GraphUnavailable
        ? error.message
        : "The interactive map could not initialize. The complete repository index remains available below.",
    );
    console.warn("Mer3ly repository graph unavailable:", error);
  });
}

async function startRepositoryGraph(graphRoot) {
  const forcedMode = new URLSearchParams(window.location.search).get("graph");
  if (forcedMode === "no-webgpu") {
    throw new GraphUnavailable(
      "WebGPU is unavailable in this browser. The complete repository index remains available below.",
    );
  }
  if (!navigator.gpu) {
    throw new GraphUnavailable(
      "WebGPU is unavailable in this browser. The complete repository index remains available below.",
    );
  }

  const authorityElement = document.querySelector("#repository-graph-data");
  if (!authorityElement) {
    throw new GraphUnavailable(
      "The interactive map has no public graph data. The complete repository index remains available below.",
    );
  }
  const authority = JSON.parse(authorityElement.textContent);

  if (forcedMode === "no-wasm") {
    throw new GraphUnavailable(
      "WebAssembly is unavailable in this browser. The complete repository index remains available below.",
    );
  }
  await initWasm();
  const layout = JSON.parse(layoutGraph(JSON.stringify(authority)));
  validateProjection(authority, layout);

  if (forcedMode === "init-failure") {
    throw new GraphUnavailable(
      "The interactive map could not initialize. The complete repository index remains available below.",
    );
  }

  const renderer = await RepositoryGraphRenderer.create(graphRoot, layout);
  installGraphControls(graphRoot, renderer, layout);
  renderer.fit();
  renderer.schedule();

  graphRoot.dataset.graphState = "ready";
  graphRoot.querySelector("[data-graph-fallback]").hidden = true;
  graphRoot.querySelector("[data-graph-interface]").hidden = false;
  announce(
    graphRoot,
    `${layout.nodes.length} repositories and ${layout.edges.length} relationships arranged by Mere. Select a repository node to inspect it.`,
  );
}

function validateProjection(authority, layout) {
  if (
    authority.schema !== "mer3ly.repo-graph/v1" ||
    layout.schema !== "mer3ly.repo-graph-layout/v1"
  ) {
    throw new Error("repository graph schema mismatch");
  }
  if (layout.authority_schema !== authority.schema) {
    throw new Error("repository graph authority mismatch");
  }
  const authorityNodes = authority.nodes.map(({ id }) => id).sort();
  const layoutNodes = layout.nodes.map(({ id }) => id).sort();
  const authorityEdges = authority.edges.map(({ id }) => id).sort();
  const layoutEdges = layout.edges.map(({ id }) => id).sort();
  if (
    JSON.stringify(authorityNodes) !== JSON.stringify(layoutNodes) ||
    JSON.stringify(authorityEdges) !== JSON.stringify(layoutEdges)
  ) {
    throw new Error("repository graph projection lost authority records");
  }
}

function showFallback(graphRoot, message) {
  graphRoot.dataset.graphState = "unavailable";
  const fallback = graphRoot.querySelector("[data-graph-fallback]");
  const graphInterface = graphRoot.querySelector("[data-graph-interface]");
  if (fallback) {
    fallback.hidden = false;
    fallback.textContent = message;
  }
  if (graphInterface) {
    graphInterface.hidden = true;
  }
  announce(graphRoot, message);
}

function announce(graphRoot, message) {
  const status = graphRoot.querySelector("[data-graph-status]");
  if (status) {
    status.textContent = message;
  }
}

class RepositoryGraphRenderer {
  static async create(graphRoot, layout) {
    const canvas = graphRoot.querySelector("canvas");
    const stage = graphRoot.querySelector("[data-graph-stage]");
    const nodeLayer = graphRoot.querySelector("[data-graph-nodes]");
    const adapter = await navigator.gpu.requestAdapter({
      powerPreference: "low-power",
    });
    if (!adapter) {
      throw new GraphUnavailable(
        "WebGPU could not provide a graphics adapter. The complete repository index remains available below.",
      );
    }
    const device = await adapter.requestDevice();
    const context = canvas.getContext("webgpu");
    if (!context) {
      throw new GraphUnavailable(
        "WebGPU could not create a canvas. The complete repository index remains available below.",
      );
    }
    return new RepositoryGraphRenderer(
      graphRoot,
      stage,
      canvas,
      nodeLayer,
      device,
      context,
      layout,
    );
  }

  constructor(graphRoot, stage, canvas, nodeLayer, device, context, layout) {
    this.graphRoot = graphRoot;
    this.stage = stage;
    this.canvas = canvas;
    this.nodeLayer = nodeLayer;
    this.device = device;
    this.context = context;
    this.layout = layout;
    this.format = navigator.gpu.getPreferredCanvasFormat();
    this.scale = 1;
    this.panX = 0;
    this.panY = 0;
    this.selectedId = layout.focus;
    this.frame = null;
    this.userAdjusted = false;
    this.edgeBuffer = null;
    this.nodeBuffer = null;
    this.edgePipeline = this.createEdgePipeline();
    this.nodePipeline = this.createNodePipeline();

    this.context.configure({
      device: this.device,
      format: this.format,
      alphaMode: "premultiplied",
    });
    this.installNodeButtons();
    this.installPointerControls();
    this.resizeObserver = new ResizeObserver(() => {
      if (!this.userAdjusted) {
        this.fit();
      }
      this.schedule();
    });
    this.resizeObserver.observe(this.stage);
    document.addEventListener("visibilitychange", () => {
      if (!document.hidden) {
        this.schedule();
      }
    });
    this.device.lost.then(() => {
      showFallback(
        this.graphRoot,
        "The WebGPU device was lost. The complete repository index remains available below.",
      );
    });
  }

  createEdgePipeline() {
    const module = this.device.createShaderModule({
      label: "Mer3ly repository edge shader",
      code: `
        struct VertexInput {
          @location(0) position: vec2f,
          @location(1) color: vec4f,
        };
        struct VertexOutput {
          @builtin(position) position: vec4f,
          @location(0) color: vec4f,
        };
        @vertex fn vertex_main(input: VertexInput) -> VertexOutput {
          var output: VertexOutput;
          output.position = vec4f(input.position, 0.0, 1.0);
          output.color = input.color;
          return output;
        }
        @fragment fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
          return input.color;
        }
      `,
    });
    return this.device.createRenderPipeline({
      label: "Mer3ly repository edge pipeline",
      layout: "auto",
      vertex: {
        module,
        entryPoint: "vertex_main",
        buffers: [
          {
            arrayStride: 24,
            attributes: [
              { shaderLocation: 0, offset: 0, format: "float32x2" },
              { shaderLocation: 1, offset: 8, format: "float32x4" },
            ],
          },
        ],
      },
      fragment: {
        module,
        entryPoint: "fragment_main",
        targets: [
          {
            format: this.format,
            blend: {
              color: {
                srcFactor: "src-alpha",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
              alpha: {
                srcFactor: "one",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
            },
          },
        ],
      },
      primitive: { topology: "line-list" },
    });
  }

  createNodePipeline() {
    const module = this.device.createShaderModule({
      label: "Mer3ly repository node shader",
      code: `
        struct VertexInput {
          @location(0) position: vec2f,
          @location(1) local: vec2f,
          @location(2) color: vec4f,
        };
        struct VertexOutput {
          @builtin(position) position: vec4f,
          @location(0) local: vec2f,
          @location(1) color: vec4f,
        };
        @vertex fn vertex_main(input: VertexInput) -> VertexOutput {
          var output: VertexOutput;
          output.position = vec4f(input.position, 0.0, 1.0);
          output.local = input.local;
          output.color = input.color;
          return output;
        }
        @fragment fn fragment_main(input: VertexOutput) -> @location(0) vec4f {
          let distance = length(input.local);
          if (distance > 1.0) {
            discard;
          }
          let rim = smoothstep(0.72, 0.96, distance);
          let rim_color = vec4f(0.22, 0.08, 0.07, 1.0);
          return mix(input.color, rim_color, rim);
        }
      `,
    });
    return this.device.createRenderPipeline({
      label: "Mer3ly repository node pipeline",
      layout: "auto",
      vertex: {
        module,
        entryPoint: "vertex_main",
        buffers: [
          {
            arrayStride: 32,
            attributes: [
              { shaderLocation: 0, offset: 0, format: "float32x2" },
              { shaderLocation: 1, offset: 8, format: "float32x2" },
              { shaderLocation: 2, offset: 16, format: "float32x4" },
            ],
          },
        ],
      },
      fragment: {
        module,
        entryPoint: "fragment_main",
        targets: [
          {
            format: this.format,
            blend: {
              color: {
                srcFactor: "src-alpha",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
              alpha: {
                srcFactor: "one",
                dstFactor: "one-minus-src-alpha",
                operation: "add",
              },
            },
          },
        ],
      },
      primitive: { topology: "triangle-list" },
    });
  }

  installNodeButtons() {
    this.nodeButtons = new Map();
    for (const node of this.layout.nodes) {
      const button = document.createElement("button");
      button.type = "button";
      button.className = `repository-graph-node class-${node.class} status-${node.status}`;
      button.dataset.graphNodeId = node.id;
      button.setAttribute("aria-label", `${node.name}, ${node.class}, ${node.status}`);
      button.setAttribute("aria-pressed", "false");
      button.innerHTML = `
        <span class="repository-graph-node-mark" aria-hidden="true">${escapeMarkup(shortName(node.name))}</span>
        <span class="repository-graph-node-label" aria-hidden="true">${escapeMarkup(node.name)}</span>
      `;
      button.addEventListener("click", () => this.select(node.id));
      button.addEventListener("dblclick", () => this.open(node.id));
      button.addEventListener("focus", () => this.select(node.id));
      button.addEventListener("keydown", (event) => {
        this.handleNodeKey(event, node.id);
      });
      this.nodeLayer.append(button);
      this.nodeButtons.set(node.id, button);
    }
    this.select(this.selectedId, false);
  }

  installPointerControls() {
    let drag = null;
    this.stage.addEventListener("pointerdown", (event) => {
      if (event.target.closest("[data-graph-node-id]")) {
        return;
      }
      drag = {
        id: event.pointerId,
        x: event.clientX,
        y: event.clientY,
      };
      this.stage.setPointerCapture(event.pointerId);
      this.stage.classList.add("is-panning");
    });
    this.stage.addEventListener("pointermove", (event) => {
      if (!drag || drag.id !== event.pointerId) {
        return;
      }
      this.panBy(event.clientX - drag.x, event.clientY - drag.y);
      drag.x = event.clientX;
      drag.y = event.clientY;
    });
    const finishDrag = (event) => {
      if (!drag || drag.id !== event.pointerId) {
        return;
      }
      drag = null;
      this.stage.classList.remove("is-panning");
    };
    this.stage.addEventListener("pointerup", finishDrag);
    this.stage.addEventListener("pointercancel", finishDrag);
    this.stage.addEventListener(
      "wheel",
      (event) => {
        event.preventDefault();
        const rect = this.stage.getBoundingClientRect();
        this.zoomBy(event.deltaY < 0 ? 1.12 : 0.89, {
          x: event.clientX - rect.left,
          y: event.clientY - rect.top,
        });
      },
      { passive: false },
    );
  }

  handleNodeKey(event, currentId) {
    const index = this.layout.nodes.findIndex(({ id }) => id === currentId);
    let next = null;
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      next = (index + 1) % this.layout.nodes.length;
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      next = (index - 1 + this.layout.nodes.length) % this.layout.nodes.length;
    } else if (event.key === "Home") {
      next = 0;
    } else if (event.key === "End") {
      next = this.layout.nodes.length - 1;
    } else if (event.key === "Enter") {
      event.preventDefault();
      this.open(currentId);
      return;
    }
    if (next !== null) {
      event.preventDefault();
      const nextNode = this.layout.nodes[next];
      this.nodeButtons.get(nextNode.id).focus();
    }
  }

  select(id, announceSelection = true) {
    if (!this.nodeButtons.has(id)) {
      return;
    }
    this.selectedId = id;
    for (const [nodeId, button] of this.nodeButtons) {
      const selected = nodeId === id;
      button.classList.toggle("is-selected", selected);
      button.setAttribute("aria-pressed", String(selected));
    }
    if (announceSelection) {
      const node = this.layout.nodes.find((candidate) => candidate.id === id);
      const outgoing = this.layout.edges.filter((edge) => edge.source === id).length;
      const incoming = this.layout.edges.filter((edge) => edge.target === id).length;
      announce(
        this.graphRoot,
        `${node.name} selected. ${outgoing} outgoing and ${incoming} incoming relationships.`,
      );
    }
    this.schedule();
  }

  open(id = this.selectedId) {
    const target = document.querySelector(`#repo-${CSS.escape(id)}`);
    if (!target) {
      return;
    }
    target.tabIndex = -1;
    target.scrollIntoView({
      behavior: prefersReducedMotion() ? "auto" : "smooth",
      block: "start",
    });
    target.focus({ preventScroll: true });
  }

  fit() {
    const width = Math.max(this.stage.clientWidth, 1);
    const height = Math.max(this.stage.clientHeight, 1);
    const positions = this.layout.nodes.map((node) => this.layoutPosition(node));
    const xs = positions.map(({ x }) => x);
    const ys = positions.map(({ y }) => y);
    const minX = Math.min(...xs);
    const maxX = Math.max(...xs);
    const minY = Math.min(...ys);
    const maxY = Math.max(...ys);
    const margin = width < 480 ? 54 : 90;
    const worldWidth = Math.max(maxX - minX, 1);
    const worldHeight = Math.max(maxY - minY, 1);
    this.scale = clamp(
      Math.min(
        (width - margin * 2) / worldWidth,
        (height - margin * 2) / worldHeight,
      ),
      0.2,
      2.5,
    );
    this.panX = width * 0.5 - ((minX + maxX) * 0.5) * this.scale;
    this.panY = height * 0.5 - ((minY + maxY) * 0.5) * this.scale;
    this.userAdjusted = false;
    this.schedule();
  }

  zoomBy(factor, center = null) {
    const width = Math.max(this.stage.clientWidth, 1);
    const height = Math.max(this.stage.clientHeight, 1);
    const pivot = center ?? { x: width * 0.5, y: height * 0.5 };
    const nextScale = clamp(this.scale * factor, 0.18, 4);
    const worldX = (pivot.x - this.panX) / this.scale;
    const worldY = (pivot.y - this.panY) / this.scale;
    this.scale = nextScale;
    this.panX = pivot.x - worldX * nextScale;
    this.panY = pivot.y - worldY * nextScale;
    this.userAdjusted = true;
    this.schedule();
  }

  panBy(x, y) {
    this.panX += x;
    this.panY += y;
    this.userAdjusted = true;
    this.schedule();
  }

  schedule() {
    if (document.hidden || this.frame !== null) {
      return;
    }
    this.frame = requestAnimationFrame(() => {
      this.frame = null;
      this.draw();
    });
  }

  screenPosition(node) {
    const position = this.layoutPosition(node);
    return {
      x: position.x * this.scale + this.panX,
      y: position.y * this.scale + this.panY,
    };
  }

  layoutPosition(node) {
    if (this.stage.clientWidth < 480) {
      return { x: node.y, y: -node.x };
    }
    return node;
  }

  draw() {
    const width = Math.max(this.stage.clientWidth, 1);
    const height = Math.max(this.stage.clientHeight, 1);
    const pixelRatio = Math.min(window.devicePixelRatio || 1, 2);
    const pixelWidth = Math.max(Math.round(width * pixelRatio), 1);
    const pixelHeight = Math.max(Math.round(height * pixelRatio), 1);
    if (this.canvas.width !== pixelWidth || this.canvas.height !== pixelHeight) {
      this.canvas.width = pixelWidth;
      this.canvas.height = pixelHeight;
    }

    const nodeById = new Map(this.layout.nodes.map((node) => [node.id, node]));
    const toClip = ({ x, y }) => ({
      x: (x / width) * 2 - 1,
      y: 1 - (y / height) * 2,
    });
    const edgeVertices = [];
    for (const edge of this.layout.edges) {
      const source = nodeById.get(edge.source);
      const target = nodeById.get(edge.target);
      if (!source || !target) {
        continue;
      }
      const color = edgeColor(edge);
      const sourceClip = toClip(this.screenPosition(source));
      const targetClip = toClip(this.screenPosition(target));
      edgeVertices.push(sourceClip.x, sourceClip.y, ...color);
      edgeVertices.push(targetClip.x, targetClip.y, ...color);
    }

    const nodeVertices = [];
    for (const node of this.layout.nodes) {
      const screen = this.screenPosition(node);
      const selected = node.id === this.selectedId;
      const radius = selected ? 14 : 10;
      const color = nodeColor(node.class, selected);
      const corners = [
        [-1, -1],
        [1, -1],
        [1, 1],
        [-1, -1],
        [1, 1],
        [-1, 1],
      ];
      for (const [localX, localY] of corners) {
        const clip = toClip({
          x: screen.x + localX * radius,
          y: screen.y + localY * radius,
        });
        nodeVertices.push(clip.x, clip.y, localX, localY, ...color);
      }
      const button = this.nodeButtons.get(node.id);
      button.style.left = `${screen.x}px`;
      button.style.top = `${screen.y}px`;
    }

    this.edgeBuffer?.destroy();
    this.nodeBuffer?.destroy();
    this.edgeBuffer = createVertexBuffer(
      this.device,
      new Float32Array(edgeVertices),
      "Mer3ly repository edges",
    );
    this.nodeBuffer = createVertexBuffer(
      this.device,
      new Float32Array(nodeVertices),
      "Mer3ly repository nodes",
    );

    const encoder = this.device.createCommandEncoder({
      label: "Mer3ly repository graph commands",
    });
    const pass = encoder.beginRenderPass({
      label: "Mer3ly repository graph pass",
      colorAttachments: [
        {
          view: this.context.getCurrentTexture().createView(),
          clearValue: { r: 0.949, g: 0.929, b: 0.875, a: 1 },
          loadOp: "clear",
          storeOp: "store",
        },
      ],
    });
    if (edgeVertices.length > 0) {
      pass.setPipeline(this.edgePipeline);
      pass.setVertexBuffer(0, this.edgeBuffer);
      pass.draw(edgeVertices.length / 6);
    }
    if (nodeVertices.length > 0) {
      pass.setPipeline(this.nodePipeline);
      pass.setVertexBuffer(0, this.nodeBuffer);
      pass.draw(nodeVertices.length / 8);
    }
    pass.end();
    this.device.queue.submit([encoder.finish()]);
  }
}

function installGraphControls(graphRoot, renderer, layout) {
  const controls = graphRoot.querySelector("[data-graph-controls]");
  controls.addEventListener("click", (event) => {
    const button = event.target.closest("button[data-graph-action]");
    if (!button) {
      return;
    }
    const action = button.dataset.graphAction;
    if (action === "zoom-in") renderer.zoomBy(1.2);
    if (action === "zoom-out") renderer.zoomBy(0.82);
    if (action === "fit") renderer.fit();
    if (action === "pan-left") renderer.panBy(36, 0);
    if (action === "pan-right") renderer.panBy(-36, 0);
    if (action === "pan-up") renderer.panBy(0, 36);
    if (action === "pan-down") renderer.panBy(0, -36);
    if (action === "open") renderer.open();
  });
  const reduced = prefersReducedMotion();
  graphRoot.dataset.reducedMotion = String(reduced);
  graphRoot.dataset.graphEngine = layout.engine;
}

function createVertexBuffer(device, data, label) {
  const buffer = device.createBuffer({
    label,
    size: Math.max(data.byteLength, 4),
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
  });
  if (data.byteLength > 0) {
    device.queue.writeBuffer(buffer, 0, data);
  }
  return buffer;
}

function edgeColor(edge) {
  if (edge.kind === "host_for") return [0.55, 0.1, 0.08, 0.82];
  if (edge.kind === "renders_with") return [0.76, 0.44, 0.12, 0.82];
  if (edge.provenance === "curated") return [0.7, 0.42, 0.13, 0.78];
  return [0.22, 0.4, 0.5, 0.52];
}

function nodeColor(repositoryClass, selected) {
  const colors = {
    product: [0.55, 0.1, 0.09, 1],
    platform: [0.16, 0.34, 0.44, 1],
    foundation: [0.74, 0.45, 0.15, 1],
    tool: [0.38, 0.34, 0.29, 1],
  };
  const color = colors[repositoryClass] ?? colors.tool;
  return selected
    ? color.map((channel, index) =>
        index === 3 ? channel : Math.min(channel * 1.18, 1),
      )
    : color;
}

function shortName(name) {
  if (name === "Merely organization profile") return "M";
  if (name === "Mer3ly") return "M3";
  const pieces = name.split(/[\s-]+/).filter(Boolean);
  if (pieces.length > 1) {
    return pieces
      .slice(0, 2)
      .map((piece) => piece[0])
      .join("")
      .toUpperCase();
  }
  return name.slice(0, 2).toUpperCase();
}

function escapeMarkup(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function prefersReducedMotion() {
  return (
    window.matchMedia("(prefers-reduced-motion: reduce)").matches ||
    new URLSearchParams(window.location.search).get("motion") === "reduce"
  );
}

function clamp(value, minimum, maximum) {
  return Math.min(Math.max(value, minimum), maximum);
}
