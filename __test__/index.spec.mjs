import test from "ava";

import { Level, Tracing } from "../index.js";

test("test level config", (t) => {
  const tracing = new Tracing();
  tracing.config(Level.Trace);
  t.is(tracing.level, Level.Trace);
});
