import "dotenv/config";
import { serve } from "@hono/node-server";
import app from "./app";

const port = 3000;
console.log(`Server is running on port ${port}`);
console.log(`Admin UI: http://localhost:${port}/admin`);

serve({
  fetch: app.fetch,
  port,
});
