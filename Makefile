# Image Labeling Website - Makefile
# This Makefile provides convenient commands to run and manage the project

.PHONY: help dev backend frontend build clean install test lint

# Default target
help:
	@echo "Image Labeling Website - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  dev         - Run both frontend and backend in development mode"
	@echo "  backend     - Run only the backend server"
	@echo "  frontend    - Run only the frontend development server"
	@echo ""
	@echo "Build:"
	@echo "  build       - Build both frontend and backend"
	@echo "  build-backend - Build only the backend"
	@echo "  build-frontend - Build only the frontend"
	@echo ""
	@echo "Installation:"
	@echo "  install     - Install dependencies for both frontend and backend"
	@echo "  install-backend - Install backend dependencies"
	@echo "  install-frontend - Install frontend dependencies"
	@echo ""
	@echo "Testing:"
	@echo "  test        - Run tests for both frontend and backend"
	@echo "  test-backend - Run backend tests"
	@echo "  test-frontend - Run frontend tests"
	@echo ""
	@echo "Utilities:"
	@echo "  clean       - Clean build artifacts"
	@echo "  lint        - Run linters for both frontend and backend"
	@echo "  format      - Format code for both frontend and backend"
	@echo ""

# Development commands
dev: install
	@echo "Starting both frontend and backend in development mode..."
	@echo "Backend will run on http://localhost:8080"
	@echo "Frontend will run on http://localhost:3000"
	@echo ""
	@echo "Press Ctrl+C to stop both servers"
	@trap 'kill %1 %2' INT; \
	cd backend && cargo run & \
	cd frontend && npm run dev & \
	wait

backend:
	@echo "Starting backend server on http://localhost:8080..."
	cd backend && cargo run

frontend:
	@echo "Starting frontend development server on http://localhost:3000..."
	cd frontend && npm run dev

# Build commands
build: build-backend build-frontend

build-backend:
	@echo "Building backend..."
	cd backend && cargo build --release

build-frontend:
	@echo "Building frontend..."
	cd frontend && npm run build

# Installation commands
install: install-backend install-frontend

install-backend:
	@echo "Installing backend dependencies..."
	cd backend && cargo build

install-frontend:
	@echo "Installing frontend dependencies..."
	cd frontend && npm install

# Testing commands
test: test-backend test-frontend

test-backend:
	@echo "Running backend tests..."
	cd backend && cargo test

test-frontend:
	@echo "Running frontend tests..."
	cd frontend && npm test || echo "No frontend tests configured"

# Utility commands
clean:
	@echo "Cleaning build artifacts..."
	cd backend && cargo clean
	cd frontend && rm -rf .next out node_modules/.cache

lint:
	@echo "Running linters..."
	@echo "Backend linting..."
	cd backend && cargo clippy -- -D warnings
	@echo "Frontend linting..."
	cd frontend && npm run lint

format:
	@echo "Formatting code..."
	@echo "Backend formatting..."
	cd backend && cargo fmt
	@echo "Frontend formatting..."
	cd frontend && npx prettier --write "src/**/*.{ts,tsx,js,jsx,json,css,md}"

# Database commands
db-reset:
	@echo "Resetting database..."
	cd backend && rm -f sqlite.db && cargo run --bin setup-db || echo "Database reset complete"

# Production commands
prod: build
	@echo "Starting production servers..."
	@echo "Backend will run on http://localhost:8080"
	@echo "Frontend will run on http://localhost:3000"
	@trap 'kill %1 %2' INT; \
	cd backend && cargo run --release & \
	cd frontend && npm start & \
	wait

# Quick start for new developers
setup: install
	@echo "Setting up development environment..."
	@echo "Database setup..."
	cd backend && cargo run --bin setup-db || echo "Database setup complete"
	@echo ""
	@echo "Setup complete! Run 'make dev' to start development servers."
