/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum Level {
  Trace = 0,
  Debug = 1,
  Info = 2,
  Warn = 3,
  Error = 4
}
export class Tracing {
  level?: Level
  config(level: Level): void
  constructor()
  trace(value: string): void
}
