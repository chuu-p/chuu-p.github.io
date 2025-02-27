+++
title = "GRTTy Stack: gRPC Rust React Typescript"
date = 2025-02-27
[extra]
postid = 114075174358638781
MASTODON_POST_ID = 114075174358638781
+++

> Hit the GRTTy!

## spec

- frontend / backend contract 

example-spec/example.proto

~~~proto
syntax = "proto3";

package example;

service HealthCheckService {
  rpc CheckHealth(HealthCheckRequest) returns (HealthCheckReply) {}
}
message HealthCheckRequest {}
message HealthCheckReply {}
~~~

example-spec/.env

~~~ini
VITE_GRPC_SERVER_ADDRESS=127.0.0.1
VITE_GRPC_SERVER_PORT=50051
VITE_FRONTEND_ADDRESS=127.0.0.1
VITE_FRONTEND_PORT=5173
DATABASE_URL=example.db
~~~

## frontend

~~~sh
pnpm create vite example --template react-ts
cd example
pnpm install tailwindcss @tailwindcss/vite
pnpm approve-builds # press: a -> enter
~~~

vite.config.ts

~~~ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
})
~~~

delete App.css

App.tsx 

~~~ts
export default function App() {
  return (
    <>
      <h1 className="underline">Vite + React</h1>
    </>
  )
}
~~~

## backend
