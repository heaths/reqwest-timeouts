const express = require("express");
const app = express();
const PORT = process.env.PORT || 3000;

// Health check — instant response.
app.get("/health", (_req, res) => {
  res.send("ok");
});

// Delay the entire response by `seconds` (tests ClientBuilder::timeout).
app.get("/delay", (req, res) => {
  const seconds = parseFloat(req.query.seconds) || 1;
  const ms = seconds * 1000;
  console.log(`/delay: waiting ${seconds}s before responding`);
  setTimeout(() => {
    res.send(`delayed response after ${seconds}s`);
  }, ms);
});

// Send a chunked body with pauses between chunks (tests ClientBuilder::read_timeout).
app.get("/slow-body", (req, res) => {
  const chunkDelay = parseFloat(req.query.chunk_delay) || 1;
  const chunks = parseInt(req.query.chunks, 10) || 5;
  const delayMs = chunkDelay * 1000;

  console.log(
    `/slow-body: sending ${chunks} chunks with ${chunkDelay}s delay each`,
  );

  res.writeHead(200, {
    "Content-Type": "text/plain",
    "Transfer-Encoding": "chunked",
  });
  // Flush headers immediately so the client enters body-reading state.
  res.flushHeaders();

  let sent = 0;
  const interval = setInterval(() => {
    sent++;
    res.write(`chunk ${sent}/${chunks}\n`);
    if (sent >= chunks) {
      clearInterval(interval);
      res.end();
    }
  }, delayMs);
});

app.listen(PORT, () => {
  console.log(`Test server listening on http://localhost:${PORT}`);
});
