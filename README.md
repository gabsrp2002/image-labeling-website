# Image Labeling Website

A web portal to facilitate labeling of images for AI training. This application consists of a Rust backend API and a Next.js frontend, providing a complete solution for image annotation workflows.

## Features

- **Admin Dashboard**: Manage groups, labelers, images, and tags
- **Labeler Interface**: Label images with predefined tags
- **AI-Powered Suggestions**: OpenAI integration for automatic tag suggestions
- **Group Management**: Organize images and labelers into groups
- **Export Functionality**: Export labeled data for AI training

## Architecture

- **Backend**: Rust with Actix Web, SQLite database, JWT authentication
- **Frontend**: Next.js with TypeScript, Tailwind CSS
- **Database**: SQLite (configurable)
- **AI Integration**: OpenAI API for tag suggestions

## Prerequisites

Before running the application, ensure you have the following installed:

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **Node.js** (v18 or later) - [Install Node.js](https://nodejs.org/)
- **npm** (comes with Node.js)

## Environment Variables

The application requires several environment variables to be set. Create a `.env` file in the `backend` directory with the following variables:

### Required Environment Variables

```bash
# Database Configuration
DATABASE_URL=sqlite:sqlite.db

# JWT Authentication
JWT_SECRET=your-super-secret-jwt-key-here

# OpenAI API (for tag suggestions)
OPENAI_API_KEY=your-openai-api-key-here
```

### Environment Variable Details

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | SQLite database connection string | `sqlite:sqlite.db` | No |
| `JWT_SECRET` | Secret key for JWT token signing | - | **Yes** |
| `OPENAI_API_KEY` | OpenAI API key for tag suggestions | - | **Yes** |

### Example .env File

Create `backend/.env`:

```bash
# Database
DATABASE_URL=sqlite:sqlite.db

# JWT Secret (generate a strong random string)
JWT_SECRET=my-super-secret-jwt-key-change-this-in-production

# OpenAI API Key (get from https://platform.openai.com/api-keys)
OPENAI_API_KEY=sk-your-openai-api-key-here
```

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd image-labeling-website
```

### 2. Install Dependencies

#### Backend Dependencies
```bash
cd backend
cargo build
```

#### Frontend Dependencies
```bash
cd frontend
npm install
```

#### Or install both at once:
```bash
make install
```

## Running the Application

### Development Mode

#### Option 1: Run Both Services Together
```bash
make dev
```

This will start:
- Backend server on `http://localhost:8080`
- Frontend development server on `http://localhost:3000`

#### Option 2: Run Services Separately

**Backend only:**
```bash
make backend
# or
cd backend && cargo run
```

**Frontend only:**
```bash
make frontend
# or
cd frontend && npm run dev
```

### Production Mode

```bash
make prod
```

This builds and runs both services in production mode.

## Default Credentials

The application creates a default admin user on first run:

- **Username**: `admin`
- **Password**: `admin`

**⚠️ Important**: Change these credentials in production!

## API Endpoints

The backend API runs on `http://localhost:8080/api/v1` and provides the following main endpoints:

### Authentication
- `POST /login` - User login

### Admin Endpoints
- `GET /admin/groups` - List all groups
- `POST /admin/groups` - Create a new group
- `GET /admin/groups/{id}` - Get group details
- `DELETE /admin/groups/{id}` - Delete a group
- `POST /admin/image` - Upload an image
- `GET /admin/tag/group/{group_id}` - Get tags for a group
- `POST /admin/tag` - Create a new tag
- `GET /admin/export/bulk` - Export all data

### Labeler Endpoints
- `GET /labeler/groups` - Get groups assigned to labeler
- `GET /labeler/groups/{group_id}/images` - Get images in a group
- `GET /labeler/groups/{group_id}/images/{image_id}` - Get image details
- `PUT /labeler/groups/{group_id}/images/{image_id}/tags` - Update image tags
- `POST /labeler/images/{image_id}/suggest_tags` - Get AI tag suggestions

## Development Workflow

### Using Make Commands

The project includes a comprehensive Makefile with useful commands:

```bash
# Development
make dev          # Run both frontend and backend
make backend      # Run only backend
make frontend     # Run only frontend

# Building
make build        # Build both services
make build-backend # Build only backend
make build-frontend # Build only frontend

# Testing
make test         # Run all tests
make test-backend # Run backend tests
make test-frontend # Run frontend tests

# Utilities
make clean        # Clean build artifacts
make lint         # Run linters
make format       # Format code
```

### Database Management

The application uses SQLite by default. The database file (`sqlite.db`) is *NOT* created automatically in the `backend` directory when the backend runs for the first time, but it's populated automatically, as the application creates the necessary database tables on startup.

To create a database, simply run `touch sqlite.db` inside the backend directory.

## Project Structure

```
image-labeling-website/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── entity/         # Database entities
│   │   ├── repository/     # Data access layer
│   │   ├── routes/         # API routes
│   │   ├── service/        # Business logic
│   │   └── middleware/     # Authentication middleware
│   ├── Cargo.toml
│   └── sqlite.db          # SQLite database (created automatically on first backend run)
├── frontend/               # Next.js frontend
│   ├── src/
│   │   ├── app/           # Next.js app router pages
│   │   ├── components/    # React components
│   │   ├── contexts/      # React contexts
│   │   └── utils/         # Utility functions
│   └── package.json
└── Makefile               # Development commands
```

## Troubleshooting

### Common Issues

1. **Backend won't start**: Check that all environment variables are set correctly
2. **Frontend can't connect to backend**: Ensure backend is running on port 8080
3. **Database errors**: Check that the `DATABASE_URL` is correct and the database file is writable
4. **OpenAI API errors**: Verify your `OPENAI_API_KEY` is valid and has sufficient credits

### Logs

- Backend logs are displayed in the terminal where you run `cargo run`
- Frontend logs are displayed in the terminal where you run `npm run dev`
- Check browser developer console for frontend errors

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `make test`
5. Run linters: `make lint`
6. Submit a pull request

## License

[Add your license information here]
