const http = require("http");
const WebSocket = require("ws");

const port = Number(process.argv[2] || 9226);
const expectedText = process.argv[3] || "Cute Clipboard E2E";

function getJson(path) {
  return new Promise((resolve, reject) => {
    const request = http.get({ host: "127.0.0.1", port, path, timeout: 2000 }, (response) => {
      let body = "";
      response.setEncoding("utf8");
      response.on("data", (chunk) => {
        body += chunk;
      });
      response.on("end", () => {
        try {
          resolve(JSON.parse(body));
        } catch (error) {
          reject(error);
        }
      });
    });
    request.on("error", reject);
    request.on("timeout", () => {
      request.destroy(new Error("CDP request timed out"));
    });
  });
}

async function send(ws, method, params = {}) {
  const id = send.nextId++;
  ws.send(JSON.stringify({ id, method, params }));
  return new Promise((resolve, reject) => {
    const handleMessage = (raw) => {
      const message = JSON.parse(raw);
      if (message.id !== id) return;
      ws.off("message", handleMessage);
      if (message.error) {
        reject(new Error(message.error.message));
      } else {
        resolve(message.result);
      }
    };
    ws.on("message", handleMessage);
  });
}
send.nextId = 1;

async function main() {
  const targets = await getJson("/json/list");
  const pages = targets.filter((target) => target.type === "page" && target.webSocketDebuggerUrl);
  const errors = [];

  for (const target of pages) {
    const matched = await tryAssertContent(target).catch((error) => {
      errors.push(`${target.title || target.url}: ${error.message}`);
      return false;
    });
    if (matched) {
      return;
    }
  }

  throw new Error(`History content "${expectedText}" not found in CDP targets: ${JSON.stringify(targets)} errors=${errors.join("; ")}`);
}

async function tryAssertContent(target) {
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {
    ws.once("open", resolve);
    ws.once("error", reject);
  });

  try {
    const result = await send(ws, "Runtime.evaluate", {
      expression: `
        (() => {
          const text = document.body?.innerText || "";
          return text.includes(${JSON.stringify(expectedText)});
        })()
      `,
      awaitPromise: true,
      returnByValue: true
    });
    return result?.result?.value === true;
  } finally {
    ws.close();
  }
}

main().catch((error) => {
  console.error(error.message);
  process.exit(1);
});
