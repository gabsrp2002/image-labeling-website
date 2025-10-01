# Image Labeling Website API documentation

## Login Endpoint

### POST /api/v1/login

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
curl -X POST http://127.0.0.1:8080/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin_user",
    "password": "password123",
    "role": "admin"
  }'
```

# Running the Server

1. Create a `.env` file in the backend directory and set your configuration:
```bash
# Generate a secure JWT secret
openssl rand -base64 32
# Add the generated secret to your .env file

# Optional: OpenAI API key for AI-powered tag suggestions using GPT-4 Vision
# If not set, the system will use mock suggestions
OPENAI_API_KEY=your_openai_api_key_here
```

2. Start the server:
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
