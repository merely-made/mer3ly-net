const benches = document.querySelectorAll("[data-radio-simulator]");

const LOCAL_PAGES = ["STATUS", "POWER", "RADIO", "TRAFFIC"];
const HOST_PAGES = [...LOCAL_PAGES, "IDENTITY", "LINKS", "PEERS"];

const PAGE_CONTENT = {
  STATUS: {
    code: "PHY · OK",
    rows: ["BOARD  HELTEC V4", "FW     RETINUE", "HOST   —", "RADIO  SX1262 READY"],
    ticker: "LOCAL · MODEM READY",
  },
  POWER: {
    code: "PHY · POWER",
    rows: ["SOURCE  USB", "BATTERY —", "DISPLAY ON", "LED     IDLE"],
    ticker: "LOCAL · WAKE USB",
  },
  RADIO: {
    code: "PHY · RADIO",
    rows: ["PROFILE LONGFAST", "FREQ    906.875", "SF11 · BW250", "TX      17 DBM"],
    ticker: "LOCAL · PROFILE APPLIED",
  },
  TRAFFIC: {
    code: "PHY · TRAFFIC",
    rows: ["TX      14 FRAMES", "RX      19 FRAMES", "RSSI    -97 DBM", "SNR     +6 DB"],
    ticker: "LOCAL · LAST RX 243 B",
  },
  IDENTITY: {
    code: "RET · IDENTITY",
    rows: ["NAME    HERALD", "ADDR    4C9F…BD08", "ROLE    HOST NODE", "STATE   RECOGNIZED"],
    ticker: "HOST · TRUSTED SNAPSHOT",
  },
  LINKS: {
    code: "RET · LINKS",
    rows: ["ADMITTED 2", "PENDING  0", "QUEUE    0", "LAST     8S"],
    ticker: "HOST · LINK TRUTH",
  },
  PEERS: {
    code: "RET · PEERS",
    rows: ["● HOLLOW · 8S", "● RIDGE  · 19S", "○ GARAGE · 2M", "+0 MORE"],
    ticker: "HOST · THREE PEERS",
  },
};

const FIRMWARE = {
  rnode: { code: "RND", name: "RNODE" },
  meshtastic: { code: "MST", name: "MESHTASTIC" },
  meshcore: { code: "MCR", name: "MESHCORE" },
};

class RadioBench {
  constructor(root) {
    this.root = root;
    this.screen = root.querySelector("[data-radio-screen]");
    this.header = root.querySelector("[data-screen-header]");
    this.counter = root.querySelector("[data-screen-counter]");
    this.rows = [...root.querySelectorAll("[data-screen-row]")];
    this.ticker = root.querySelector("[data-screen-ticker]");
    this.led = root.querySelector("[data-radio-led]");
    this.boundary = root.querySelector("[data-radio-boundary]");
    this.help = root.querySelector("[data-radio-help]");
    this.firmware = root.querySelector("[data-radio-firmware]");
    this.scenario = root.querySelector("[data-radio-scenario]");
    this.input = root.querySelector("[data-radio-input]");
    this.buttons = [...root.querySelectorAll("[data-radio-action]")];
    this.fallback = root.querySelector("[data-radio-fallback]");
    this.page = 0;
    this.modal = null;
    this.menu = 0;
    this.displayOn = true;
    this.ledTimer = null;

    this.firmware.addEventListener("change", () => this.reset());
    this.scenario.addEventListener("change", () => this.reset());
    this.input.addEventListener("change", () => this.applyInputFace());
    this.buttons.forEach((button) => {
      button.addEventListener("click", () => this.act(button.dataset.radioAction));
    });

    this.fallback.hidden = true;
    this.applyInputFace();
    this.render();
    root.dataset.ready = "true";
  }

  get pages() {
    return this.scenario.value === "host" ? HOST_PAGES : LOCAL_PAGES;
  }

  reset() {
    clearTimeout(this.ledTimer);
    this.ledTimer = null;
    this.page = 0;
    this.menu = 0;
    this.modal = this.scenario.value === "fault" ? "FAULT" : null;
    this.displayOn = true;
    this.applyInputFace();
    this.render();
  }

  applyInputFace() {
    const twoButton = this.input.value === "two";
    const retinue = this.firmware.value === "retinue";
    this.root.dataset.inputFace = this.input.value;
    this.buttons.forEach((button) => {
      const needsTwo = button.dataset.requiresTwo === "true";
      button.hidden = needsTwo && !twoButton;
      button.disabled = !retinue || this.scenario.value === "fault";
    });
    this.help.textContent = twoButton
      ? "A steps forward. B steps back. Hold A+B for the menu. In the menu, A moves and B selects."
      : "Tap the fitted V4 button to step forward. Hold it for the menu; tap to move and hold to select."
    this.render();
  }

  act(action) {
    if (this.firmware.value !== "retinue" || this.scenario.value === "fault") return;
    this.pulse("activity");

    if (!this.displayOn) {
      this.displayOn = true;
      this.render();
      return;
    }

    if (this.modal === "MENU") {
      this.actInMenu(action);
      return;
    }
    if (this.modal === "VERIFY") {
      this.modal = null;
      this.render();
      return;
    }

    if (action === "a-short") this.step(1);
    if (action === "b-short" && this.input.value === "two") this.step(-1);
    if (action === "a-long" && this.input.value === "one") this.openMenu();
    if (action === "a-long" && this.input.value === "two" && this.scenario.value === "host") {
      this.modal = "VERIFY";
      this.render();
    }
    if (action === "b-long" && this.input.value === "two") {
      this.displayOn = false;
      this.render();
    }
    if (action === "chord" && this.input.value === "two") this.openMenu();
  }

  actInMenu(action) {
    const items = this.menuItems();
    const move = action === "a-short";
    const select =
      (this.input.value === "one" && action === "a-long") ||
      (this.input.value === "two" && action === "b-short");
    if (move) {
      this.menu = (this.menu + 1) % items.length;
      this.render();
      return;
    }
    if (this.input.value === "two" && action === "b-long") {
      this.modal = null;
      this.render();
      return;
    }
    if (!select) return;
    const selected = items[this.menu];
    if (selected === "BACK") this.modal = null;
    if (selected === "VERIFY") this.modal = "VERIFY";
    if (selected === "DISPLAY OFF") {
      this.modal = null;
      this.displayOn = false;
    }
    this.render();
  }

  menuItems() {
    return this.scenario.value === "host"
      ? ["BACK", "VERIFY", "DISPLAY OFF"]
      : ["BACK", "DISPLAY OFF"];
  }

  openMenu() {
    this.modal = "MENU";
    this.menu = 0;
    this.pulse("operation");
    this.render();
  }

  step(direction) {
    this.page = (this.page + direction + this.pages.length) % this.pages.length;
    this.render();
  }

  pulse(kind) {
    clearTimeout(this.ledTimer);
    this.led.dataset.ledState = kind;
    this.ledTimer = setTimeout(() => {
      this.led.dataset.ledState = "idle";
    }, kind === "operation" ? 1300 : 700);
  }

  setScreen(header, counter, rows, ticker, mode = "page") {
    this.screen.dataset.screenMode = mode;
    this.header.textContent = header;
    this.counter.textContent = counter;
    this.rows.forEach((row, index) => {
      row.textContent = rows[index] ?? "";
      row.classList.toggle("is-selected", rows[index]?.startsWith(">") ?? false);
    });
    this.ticker.textContent = ticker;
    this.screen.setAttribute(
      "aria-label",
      [header, ...rows, ticker].filter(Boolean).join(". "),
    );
  }

  render() {
    if (!this.screen) return;
    const retinue = this.firmware.value === "retinue";
    this.root.dataset.firmwareOwner = retinue ? "retinue" : "upstream";
    this.led.dataset.ledState = this.scenario.value === "fault" ? "fault" : "idle";

    if (!retinue) {
      const owner = FIRMWARE[this.firmware.value];
      this.setScreen(
        `${owner.code} · HANDOFF`,
        "—",
        [owner.name, "UPSTREAM IMAGE", "OWNS DISPLAY", "RETINUE FACE —"],
        "SIMULATION STOPS AT OWNER BOUNDARY",
        "handoff",
      );
      this.boundary.textContent = `${owner.name} is the selected image. Its upstream firmware owns the screen and controls; this bench does not counterfeit that interface.`;
      return;
    }

    if (this.scenario.value === "fault") {
      this.setScreen(
        "PHY · FAULT",
        "E01",
        ["FAULT", "SX1262 INIT", "FAILED", "RETRY 4S"],
        "LOCAL · SEE HOST LOG",
        "fault",
      );
      this.boundary.textContent = "The radio fault preempts every page. This is local firmware truth, independent of an attached host.";
      return;
    }

    if (!this.displayOn) {
      this.setScreen("", "", ["", "DISPLAY OFF", "", "KEY TO WAKE"], "", "off");
      this.boundary.textContent = "The display is off. The next control press wakes it and is consumed, matching the controller contract.";
      return;
    }

    if (this.modal === "MENU") {
      const items = this.menuItems();
      const rows = items.map((item, index) => `${index === this.menu ? ">" : " "} ${item}`);
      this.setScreen(
        "MENU",
        `${this.menu + 1}/${items.length}`,
        rows,
        this.input.value === "two" ? "A MOVE · B SELECT" : "TAP MOVE · HOLD SELECT",
        "menu",
      );
      this.boundary.textContent = "The menu exposes only implemented actions. Pairing and OTA placeholders stay absent until their contracts exist.";
      return;
    }

    if (this.modal === "VERIFY") {
      this.setScreen(
        "VERIFY · HOST",
        "OK",
        ["HERALD", "4C9F…BD08", "ADMITTED 2", "SNAPSHOT TRUSTED"],
        "PRESS ANY KEY TO RETURN",
        "verify",
      );
      this.boundary.textContent = "Verification appears only with an attached trusted host snapshot; the radio does not invent node identity.";
      return;
    }

    const pageName = this.pages[this.page];
    const page = PAGE_CONTENT[pageName];
    const rows = [...page.rows];
    if (this.scenario.value === "host" && pageName === "STATUS") rows[2] = "HOST   ATTACHED";
    this.setScreen(page.code, `${this.page + 1}/${this.pages.length}`, rows, page.ticker);
    this.boundary.textContent = this.scenario.value === "host"
      ? "Attached-host mode adds identity, link, and peer pages from a deterministic trusted snapshot."
      : "Local-radio mode exposes only board, power, radio, traffic, and fault facts the firmware owns.";
  }
}

benches.forEach((bench) => new RadioBench(bench));
