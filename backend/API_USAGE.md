# Image Labeling Website API

## Login Endpoint

### POST /api/login

Authenticates a user and returns a JWT token.

**Request Body:**
```json
{
  "username": "string",
  "password": "string", 
  "role": "admin" | "labeler"
}
```

**Success Response (200):**
```json
{
  "token": "jwt_token_string"
}
```

**Error Response (403):**
```json
"Invalid credentials"
```

**Example Usage:**

```bash
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin_user",
    "password": "password123",
    "role": "admin"
  }'
```

## Running the Server

1. Copy the example environment file:
```bash
cd backend
cp .env.example .env
```

2. Edit the `.env` file and set your JWT secret:
```bash
# Generate a secure JWT secret
openssl rand -base64 32
# Add the generated secret to your .env file
```

3. Start the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## JWT Token

The JWT token contains:
- `user_id`: The user's ID from the database
- `role`: Either "admin" or "labeler"
- `exp`: Token expiration time (24 hours from creation)

**Note:** In production, make sure to:
1. Use a secure JWT secret key from environment variables (✅ implemented)
2. Implement proper password hashing when creating users
3. Add proper error handling and logging
4. Use HTTPS for secure communication
5. Never commit the `.env` file to version control
