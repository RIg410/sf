# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a fitness center management system with multiple components:
- **Rust backend** - Main API and Telegram bot for fitness center operations
- **Flutter web app** - Web interface for gym management
- **gRPC API** - Communication between frontend and backend
- **MongoDB** - Database storage
- **Docker** - Containerized deployment with nginx

## Architecture

### Backend (Rust)
- **Workspace structure**: Monorepo with multiple crates organized under `crates/`
- **Main entry point**: `bins/src/main.rs` - starts bot, API server, and background processes
- **Core services**: Located in `crates/services/` - handles users, bookings, payments, calendar, etc.
- **Bot modules**: Located in `crates/bot/` - different Telegram bot functionality (trainings, users, finance, etc.)
- **Core utilities**: Located in `crates/core/` - shared utilities (time, rights, identifiers, etc.)
- **Service layer**: `SfServices` in `crates/services/src/lib.rs` coordinates all business logic services

### Frontend (Flutter)
- **Location**: `web_app/` directory
- **Platform detection**: Responsive design with separate `DesktopPage` and `MobilePage` components
- **gRPC client**: Uses generated Dart code from protobuf definitions
- **Proto generation**: Run `web_app/script/generate_protos.sh` to generate Dart gRPC clients

### API Layer
- **gRPC server**: Runs on port 3000, defined in `api/main/src/lib.rs`
- **Protocol buffers**: Definitions in `api/proto/` directory
- **Services**: Auth and Users services with web-compatible gRPC

## Common Development Commands

### Building and Running
```bash
# Start development environment
make up                    # Start Docker containers
make start                 # Start with cargo leptos watch

# Build and deploy
make build-front          # Build Flutter web app and copy to bot-static/
make deploy-front         # Build frontend and sync to deployment
make restart              # Restart both nginx and backend
make restart-back         # Restart only backend
make restart-nginx        # Restart only nginx
```

### Code Quality
```bash
make checks               # Run fmt, test, clippy
make fmt                  # Format code
make clippy               # Run linter
make test                 # Run tests

# After code generation, always run:
cargo fmt                 # Format generated code
cargo clippy --fix        # Fix clippy warnings automatically
```

### Development Workflow
```bash
# Backend development
cargo build --release     # Build backend for production
cargo test                # Run all tests

# Frontend development  
cd web_app
flutter clean             # Clean Flutter build
flutter build web --release  # Build web app for production
sh ./script/generate_protos.sh  # Regenerate gRPC client code
```

### Logs and Monitoring
```bash
make logs                 # View backend container logs
```

## Key Implementation Details

### Service Dependencies
- Services are interconnected with dependency injection through `SfServices`
- `History` service is used by most other services for audit logging
- Services communicate through well-defined interfaces, not direct database access

### Database
- MongoDB with connection handled by `store::Db`
- Database name: `SF_DB_NAME` constant
- Connection configured via `MONGO_URL` environment variable

### Bot Framework
- Built on `teloxide` Telegram bot framework
- State management through `bot_core` with context and handlers
- Modular bot functionality split across different crates in `crates/bot/`

### Authentication & Authorization
- JWT-based authentication through `api/main/src/auth/`
- Telegram token validation
- Rights-based authorization system

### Background Processing
- Background tasks handled by `bg-process` crate
- Includes AI messages, birthdays, backups, notifications, rewards, etc.
- Cron-based scheduling with `tokio-cron-scheduler`

## Environment Configuration

Required environment variables (see `docker-compose.yml`):
- `MONGO_URL` - MongoDB connection string
- `TG_TOKEN` - Telegram bot token  
- `MINI_APP_KEY` - Mini app authentication key
- `YOOKASSA_TOKEN` & `YOOKASSA_SHOP_ID` - Payment processing
- `AI_BASE_URL` & `AI_API_KEY` - AI service integration

## Testing

- Rust tests: Run `cargo test` from project root
- Flutter tests: Run `flutter test` from `web_app/` directory
- No specific test framework mentioned - follows standard Rust and Flutter testing conventions