+++
title = "GRTTy Stack: gRPC Rust React Typescript"
date = 2025-02-27
category = "rust"

[extra]
postid = "114075174358638781"
miku_img = "miku_jump"
miku_q = "Hit the GRTTy!"
+++

> **{{title}}**  
> {{date}}

## Stack

- IDL
  - [gRPC](https://grpc.io/)
- Backend
  - [Rust](https://www.rust-lang.org/)
  - [Diesel](https://diesel.rs/)
  - [Sqlite](https://sqlite.org/)
- Frontend
  - [Typescript](https://www.typescriptlang.org/)
  - [React](https://react.dev/)
  - [Tailwind CSS](https://tailwindcss.com/)

> **_NOTE:_** You can find the code for a working full stack example repository [here](https://github.com/chuu-p/GRTTy) [https://github.com/chuu-p/GRTTy](https://github.com/chuu-p/GRTTy)

Why did I choose this stack?

- IDL (Interface Definition Language): gRPC

gRPC offers strong typing, fast performance, and native streaming. Compared to OpenAPI, it’s more efficient for internal
service communication but slightly more complex to work with in frontend environments.

- Backend: Rust, Diesel, Sqlite

Rust is my favourite programming laguage. Diesel is my favourite database crate, beacause you write sql queries as rust
daisychain functions, instead of raw sql, which I like a lot. I chose Sqlite as the database for this example, other
database systems (PostgreSQL, SQLite, or MySQL) can be used, too.

- Frontend: Typescript, React, Tailwind

Typescript is less horrible then Javascript, React is the most popular frontend framework, and the one I know best. I
dont like separate CSS files, insted putting the styling directly inside the react components. In the past I used MUI,
for this project I chose Tailwind for a more custom look.

## API-Specification

A frontend/backend contract via gRPC is a shared `.proto ` protobuf file that defines all the requests, responses, and
services (API functions) the frontend and backend use to talk to each other.

Both sides use this file to generate code, so they always agree on the exact data types and endpoints — ensuring they
don’t accidentally break the communication.

It’s like a single source of truth for the API.

Create the root project directory, cd into it and init git repo.

~~~sh
mkdir example
cd example
git init
~~~

Now we create the gRPC `.proto` api contract and set some environment variables. Both the frontend and backend will use
both files.

`example-spec/example.proto`

~~~proto
syntax = "proto3";

package example;

service HealthCheckService {
  rpc CheckHealth(HealthCheckRequest) returns (HealthCheckReply) {}
}
message HealthCheckRequest {}
message HealthCheckReply {}
~~~

`example-spec/.env`

~~~txt
VITE_GRPC_SERVER_ADDRESS=127.0.0.1
VITE_GRPC_SERVER_PORT=50051
VITE_FRONTEND_ADDRESS=127.0.0.1
VITE_FRONTEND_PORT=5173
DATABASE_URL=example.db
~~~

## Frontend

I use pnpm as a package manager, but u can use npm, pnpm, yarn, bun, or whatever you like.

On naming:

- project root: `example`
  - frontend: `example`
  - backend: `example-api`
  - interface definition: `example-spec`

~~~sh
# 1. create the frontend project
pnpm create vite example --template react-ts
cd example

# 2. add tailwind
pnpm install tailwindcss @tailwindcss/vite
pnpm approve-builds # press: a -> enter
~~~

`example/vite.config.ts`

~~~ts
import path from "path";
import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  envDir: path.resolve(__dirname, "../example-spec"),
});
~~~

add protobuf codegen and generate the typescript gRPC server and gRPC client code from the gRPC spec:

~~~sh
cd example

#  protobuf codegen & runtime
pnpm install -g protoc-gen-ts
pnpm add @protobuf-ts/plugin @protobuf-ts/grpcweb-transport @protobuf-ts/runtime @protobuf-ts/runtime-rpc google-protobuf @grpc/grpc-js

# generate the typescript gRPC server and gRPC client code
mkdir src/protobuf-ts-gen
npx protoc -I=../example-spec/ example.proto --ts_out=./src/protobuf-ts-gen
~~~

you need to run the `npx protoc` command every time you change the `example.proto` spec.

now we add the "health check backend" functionality to frontend.

delete `example/src/App.css`

`example/src/App.tsx`

~~~tsx
import {useState} from "react";
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {HealthCheckServiceClient} from "./protobuf-ts-gen/example.client";
import {
  HealthCheckRequest,
  HealthCheckReply,
} from "./protobuf-ts-gen/example";

const BACKEND_URL = `http://${import.meta.env.VITE_GRPC_SERVER_ADDRESS}:${import.meta.env.VITE_GRPC_SERVER_PORT}`;

const transport = new GrpcWebFetchTransport({
  baseUrl: BACKEND_URL,
});
const echoClient = new HealthCheckServiceClient(transport);

export default function HealthCheck() {
  const [outputValue, setOutValue] = useState(
    "Click 'Check Health!' to trigger a health check",
  );

  const handleSubmit = async () => {
    console.log(`Sending Request to ${BACKEND_URL}`);
    echoClient
      .checkHealth(HealthCheckRequest.create({}))
      .then((res) => setOutValue(`Success! ${res}`))
      .catch((e) => setOutValue(`Error! ${e}`));
  };

  return (
    <>
      <div className="max-w-2xl p-8 text-center">
        <h1 className="text-3xl font-bold underline">Hello world!</h1>
        <button className="border p-1 rounded shadow" onClick={handleSubmit}>
          Check Health!
        </button>
        <p>{outputValue}</p>
      </div>
    </>
  );
}
~~~

~~~sh
cd example
pnpm run dev
# Now press "Check Health!"
# -> Error! RpcError: NetworkError when attempting to fetch resource. Code: INTERNAL Method: example.HealthCheckService/CheckHealth
~~~

There is no backend / gRPC server yet!

## Backend

We will create a rust project for the the gRPC Server.

~~~sh
cd ..
cargo new example-api
cd example-api
cargo add --build tonic-build
~~~

`example-api/cargo.toml`

~~~toml
[package]
name = "example-api"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.13"
tower-layer = "0.3.3"
http = "1"
hyper = "1"
hyper-util = "0.1.4"
bytes = "1"
tracing = "0.1.16"
tokio = { version = "1.43.0", features = ["full"] }
tokio-stream = { version = "0.1.17", features = ["full"] }
tonic = "0.12.3"
tonic-web = "0.12.3"
tower = "0.5.2"
tower-http = { version = "0.6.2", features = [
  "cors",
  "compression-full",
  "fs",
] }
dotenvy = "0.15.7"
prost-types = "0.13.5"
log = "0.4.26"

[build-dependencies]
tonic-build = "*"

[dev-dependencies]
tokio-stream = { version = "0.1.17", features = ["net"] }
~~~

`example-api/.gitignore`

~~~
/target
*.db
~~~

`example-api/build.rs`

~~~rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../example-spec/example.proto")?;
    Ok(())
}
~~~

`example-api/src/main.rs`

~~~rust
use std::env;
use std::path::Path;
use std::time::Duration;
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer};

use openfg::health_check_service_server::{HealthCheckService, HealthCheckServiceServer};
use openfg::{HealthCheckReply, HealthCheckRequest};

pub mod openfg {
    tonic::include_proto!("example");
}

#[derive(Default)]
pub struct HealthCheck {}

#[tonic::async_trait]
impl HealthCheckService for HealthCheck {
    async fn check_health(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = openfg::HealthCheckReply {};
        Ok(Response::new(reply))
    }
}

pub async fn serve(listener: TcpListener) -> Result<(), tonic::transport::Error> {
    Server::builder()
        .accept_http1(true)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any)
                .max_age(Duration::from_secs(60) * 30),
        )
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(HealthCheckServiceServer::new(HealthCheck::default()))
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path(Path::new("../example-spec/.env"))?;
    let server_addr = env::var("VITE_GRPC_SERVER_ADDRESS")?;
    let server_port = env::var("VITE_GRPC_SERVER_PORT")?;
    println!("server starting");
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_addr, server_port)).await?;

    tokio::spawn(async { serve(listener).await });
    println!("server started");

    println!("Waiting now.");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(())
}
~~~

Starting the server:

~~~sh
cd example-api
cargo run
...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.35s
     Running `target/debug/example-api`
server starting
server started
Waiting now.
~~~

We can check the server works by running this command in a separate terminal:
`grpcurl -plaintext -import-path ./example-spec/ -proto example.proto '127.0.0.1:50051' example.HealthCheckService/CheckHealth`

~~~sh
grpcurl -plaintext -import-path ./example-spec/ -proto example.proto '127.0.0.1:50051' example.HealthCheckService/CheckHealth
{}
~~~

If we check the frontend and press 'Check Health!' -> Success! [object Object]

In the server terminal: `Got a request from Some(127.0.0.1:47140)`

It Works!

### Database

A fullstack ususally includes a database. For this project I chose Sqlite, since this is a minimal example and it is the
best choice for the real project I am developing, which inspired the GRTTy Stack.

`example-api/cargo.toml`

~~~toml
[dependencies]
...
chrono = { version = "0.4.39", features = ["serde"] }
diesel = { version = "2.2.7", features = [
  "chrono",
  "returning_clauses_for_sqlite_3_35",
  "sqlite",
] }
diesel_migrations = { version = "2.2.0", features = ["sqlite"] }
~~~

~~~sh
$ diesel setup
$ diesel migration generate create_health_checks
~~~

`example-api/migrations/.../up.sql`

~~~sql
CREATE TABLE
    health_checks (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        timestamp DATETIME NOT NULL
    );
~~~

`example-api/migrations/.../down.sql`

~~~sql
DROP TABLE health_checks;
~~~

Run the migration.

~~~sh
$ diesel migration run
~~~

`example-api/src/models.rs`

~~~rust
use chrono::NaiveDateTime;
use diesel::{prelude::*, sqlite::Sqlite};

use crate::schema::health_checks;

#[derive(Queryable, Selectable, Debug, Clone, Copy)]
#[diesel(table_name = health_checks)]
#[diesel(check_for_backend(Sqlite))]
pub struct HealthCheck {
    pub id: i32,
    pub timestamp: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = health_checks)]
pub struct NewHealthCheck {
    pub timestamp: NaiveDateTime
}
~~~

`example-api/src/schema.rs`

~~~rust
// @generated automatically by Diesel CLI.

diesel::table! {
    health_checks (id) {
        id -> Integer,
        timestamp -> Timestamp,
    }
}
~~~

`example-api/src/main.rs`

~~~rust
use crate::models::NewHealthCheck;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
...
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
...
#[tonic::async_trait]
impl HealthCheckService for HealthCheck {
    async fn check_health(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let mut connection = establish_connection();
        let record = NewHealthCheck {
            timestamp: Utc::now().naive_utc(),
        };

        let expected_records = diesel::insert_into(health_checks::table)
            .values(&record)
            .execute(&mut connection)
            .unwrap();

        debug_assert_eq!(1, expected_records);

        let reply = openfg::HealthCheckReply {};
        Ok(Response::new(reply))
    }
}
...
~~~

Now we start the server, press the button in the frontend, and check for a new entry in the database.

~~~sh
$ cargo run
...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.72s
     Running `target/debug/example-api`
server starting
server started
Waiting now.
Got a request from Some(127.0.0.1:52352)
Connecting to openfg.db
^C

# check if the entry was written in the db
$ sqlite3 openfg.db "SELECT * from health_checks;"
1|2025-03-02 00:28:04.312533388
~~~

## Tests

### Backend

~~~sh
$ mkdir example-api/tests
~~~

`example-api/tests/health_check.rs`

~~~rust
use diesel::prelude::*;
use example::health_check_service_client::HealthCheckServiceClient;
use example::HealthCheckRequest;
use example_api::schema::health_checks;
use example_api::{establish_connection, serve};
use std::env;
use std::path::Path;

pub mod example {
    tonic::include_proto!("example");
}

#[tokio::test]
async fn health_check_works() -> Result<(), Box<dyn std::error::Error>> {
    // arrange
    dotenvy::from_path(Path::new("../example-spec/.env"))?;

    let server_addr = env::var("VITE_GRPC_SERVER_ADDRESS")?;
    let server_port = env::var("VITE_GRPC_SERVER_PORT")?;

    println!("server starting");
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_addr, server_port)).await?;
    tokio::spawn(async { serve(listener).await });
    println!("server started");

    // empty database
    let mut connection = establish_connection();

    diesel::delete(health_checks::table).execute(&mut connection)?;
    assert_eq!(
        Ok(0),
        health_checks::table.count().first::<i64>(&mut connection)
    );

    let client_addr = format!("http://{}:{}", server_addr, server_port);
    println!("client starting on: {:?}", client_addr);
    let mut client = HealthCheckServiceClient::connect(client_addr).await?;
    println!("client connected");

    let request = tonic::Request::new(HealthCheckRequest {});

    // act
    let response = client.check_health(request).await?;

    // assert
    println!("RESPONSE={:?}", response);

    // check response status
    let status = response
        .metadata()
        .get("grpc-status")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!("0", status);

    // check database entry
    let rows = health_checks::dsl::health_checks
        .count()
        .get_result::<i64>(&mut connection)
        .unwrap();
    assert_eq!(1, rows);

    // let id = health_checks::dsl::health_checks
    //     .select(health_checks::id)
    //     .first(&mut connection);

    Ok(())
}
~~~

### Frontend

I will get to this soon ^^

TODO unwraps -> nice errror handling!

`example/src/App.test.ts`

~~~ts
import { expect, test } from 'vitest'

test('adds 1 + 2 to equal 3', () => {
  expect(1 + 2).toBe(3)
})
~~~

## Sources

- [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production)
- examples from [protobuf-ts](https://github.com/timostamm/protobuf-ts)
- examples from [tokio-rs/prost](https://github.com/tokio-rs/prost)
- [Tonic Getting Started](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

## Misc

I use [just](https://github.com/casey/just) to save and run project-specific commands. This is the justfile:

`justfile`

~~~yaml
set shell := ["fish", "-c"]

fdev:
  cd example && pnpm run dev

fgen:
  cd example && pnpm dlx protoc -I=../example-spec/ example.proto --ts_out=./src/protobuf-ts-gen

bdev:
  cd example-api && cargo watch -x run

btest:
  grpcurl -plaintext -import-path ./example-spec/ -proto example.proto '127.0.0.1:50051' example.HealthCheckService/CheckHealth

ci: test fmt clippy # audit # coverage

audit:
  cd example && pnpm audit
  cd example-api && cargo deny check advisories

test:
  cd example && pnpm test run
  cd example-api && cargo test --all-features

fmt:
  cd example && pnpm prettier --check .
  cd example-api && cargo fmt --all -- --check

clippy:
  cd example && pnpm eslint --max-warnings=0
  cd example-api && cargo clippy -- -D warnings

coverage:
  cd example && pnpm test -- --coverage
  cd example-api && cargo tarpaulin --ignore-tests

unused:
  cd example && pnpm prune
  cd example-api && cargo +nightly udeps --workspace

outdated:
  cd example && pnpm outdated
  cd example-api && cargo outdated --root-deps-only --workspace

migrate:
  cd example && npx protoc -I=../example-spec/ example.proto --ts_out=./src/protobuf-ts-gen
  cd example-api && diesel migration redo --all

fmtwrite:
  cd example && pnpm prettier --write .
  cd example-api && cargo fmt --all
~~~