# Axum Tonic: Unified REST API and gRPC Server in Rust

A production-ready reference implementation demonstrating how to serve both an HTTP REST API and a Tonic gRPC service simultaneously on a single network port in Rust. Built with Axum, Tonic, Tokio, and SQLx with PostgreSQL, this project follows Clean Architecture and Domain-Driven Design principles while maintaining isolated and shared middleware pipelines for each protocol.

## Overview

Modern microservice architectures often require supporting both external RESTful JSON clients and high-performance internal gRPC communication. Instead of managing two separate servers or binding to multiple ports, this project merges an Axum REST router and a Tonic gRPC router into a single unified service listening on one port.

Furthermore, the architecture demonstrates how to apply protocol-specific middleware layers alongside global shared middleware, ensuring clean observability, tracing, and request handling across HTTP/1.1 and HTTP/2 traffic.

## Key Features

- **Single-Port Multiplexing**: Serves both JSON REST endpoints and Protocol Buffers gRPC services concurrently on port 9090.
- **Layered Middleware Pipeline**: Supports dedicated middleware for REST routes, dedicated middleware for gRPC services, and shared global middleware for cross-cutting concerns such as request duration logging.
- **Clean Architecture and Domain-Driven Design**: Strictly separates business logic into Domain, Application, Infrastructure, and Interface layers for maximum testability and maintainability.
- **Asynchronous PostgreSQL Persistence**: Uses SQLx with connection pooling, automated schema migrations on startup, seeded sample data, and externalized raw SQL query files.
- **Zero-Config Protobuf Compilation**: Compiles Protocol Buffers automatically at build time using a vendored protoc binary, removing the need to install external protobuf compilers on the host machine.
- **Built-in Dual-Protocol Client**: Includes an executable client binary that tests both REST and gRPC endpoints against the running server.

## Architecture Overview

The codebase is organized into four distinct concentric layers:

1. **Domain Layer**: Contains core enterprise entities (BankAccount) and abstract repository trait contracts with zero dependencies on web frameworks or databases.
2. **Application Layer**: Orchestrates domain use cases through business services and maps domain models to Data Transfer Objects (DTOs).
3. **Infrastructure Layer**: Implements domain repository traits using PostgreSQL and SQLx, manages database connection pools, runs startup migrations, and stores SQL queries in dedicated files.
4. **Interface Layer**: Exposes the application to the outside world by merging Axum REST routes and Tonic gRPC routes, managing shared application state, and applying protocol-specific and global middleware layers.

## Project Structure

```text
.
├── Cargo.toml
├── build.rs
├── migrations/
│   └── 0001_init.sql
├── proto/
│   └── banking.proto
└── src/
    ├── main.rs
    ├── error.rs
    ├── bin/
    │   └── client.rs
    ├── domain/
    │   ├── entities/
    │   └── repositories/
    ├── application/
    │   ├── dto/
    │   └── services/
    ├── infrastructure/
    │   ├── database/
    │   └── repositories/
    │       └── sql/
    └── interface/
        ├── state.rs
        ├── rest/
        │   ├── handlers/
        │   ├── layers/
        │   └── routes.rs
        ├── grpc/
        │   ├── layers/
        │   ├── services/
        │   └── routes.rs
        └── shared/
            └── layers/
```

## Technology Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.7
- **gRPC Framework**: Tonic 0.12 & Prost 0.13
- **Async Runtime**: Tokio 1.0
- **Database & ORM**: PostgreSQL with SQLx 0.8
- **Serialization**: Serde & Serde JSON
- **HTTP Client**: Reqwest 0.12
- **Observability**: Tracing & Tracing Subscriber

## Prerequisites

- Rust toolchain (Cargo and Rustc supporting Edition 2024)
- Running PostgreSQL instance

Note: You do not need to install the protobuf compiler manually because it is vendored automatically during the Cargo build process.

## Getting Started

### 1. Configure Environment Variables

Create or update the environment configuration file in the project root with your PostgreSQL connection string:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/test_db
```

### 2. Run the Unified Server

Start the main server binary. On startup, the application connects to PostgreSQL, executes database migrations in the migrations directory, seeds two sample bank accounts, and binds to localhost on port 9090:

```bash
cargo run --bin axum-tonic
```

### 3. Run the Dual-Protocol Test Client

In a separate terminal window, execute the built-in client binary to send both HTTP REST requests and gRPC requests to the same server port:

```bash
cargo run --bin client
```

## API and Service Reference

### REST API Endpoints

The REST interface responds with JSON payloads and includes a custom response header identifying the protocol as rest-axum.

- **GET /bank-accounts** — Retrieves a list of all bank accounts.
- **GET /bank-accounts/:id** — Retrieves a single bank account by its UUID.

### gRPC Service Definition

The gRPC interface is defined in the banking package under BankAccountGrpcService and includes a custom response header identifying the protocol as grpc-tonic.

- **FindAllAccounts** — Unary RPC that returns all bank accounts as a repeated list of protobuf items.
- **FindAccountById** — Unary RPC that accepts an account UUID string and returns the matching bank account protobuf item.

## Middleware Execution Flow

Every incoming request on port 9090 passes through a structured middleware hierarchy:

1. **Shared Duration Log Layer**: Captures the HTTP method, URI path, content type, and total execution time for every incoming REST or gRPC call.
2. **Protocol-Specific Trace Layer**:
   - Requests routed to REST endpoints pass through the REST trace middleware, which logs HTTP REST activity and attaches the rest-axum protocol header.
   - Requests routed to gRPC services pass through the gRPC trace middleware, which logs Tonic gRPC activity and attaches the grpc-tonic protocol header.
3. **Handler / Service Execution**: Invokes the shared BankAccountService in the application layer.

## License

This project is open-source and available under the MIT License.
