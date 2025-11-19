# Realtime Recipe Assistant

A voice-first, AI-powered recipe assistant using **OpenAI Realtime API** for natural conversational cooking guidance, creativity, and publishing.

## 🎯 Features

- **Voice-First Interface**: Natural conversation with OpenAI Realtime API
- **Recipe Management**: Create, update, and version recipes with optimistic concurrency control
- **Cost Analysis**: Estimate recipe costs, optimize for budget, or enrich with premium ingredients
- **Recipe Remixer**: Generate creative variants (improvise, cultural twists, healthy versions)
- **Image Recognition**: Extract recipes from photos, detect pantry items
- **Publisher**: Generate recipe cards and social media assets
- **Analytics & Conversions**: Track views, shares, clicks, and monetization

## 🏗️ Architecture

### Backend (Rust)
- **Axum** web framework with async/await
- **SQLx** for type-safe database queries
- **PostgreSQL** for data persistence
- **OpenAI SDK** integration for Realtime API, vision, and structured outputs

### Frontend Options

#### Option 1: Rust WASM (Experimental) ⚠️
- **Leptos** framework for reactive UI
- Limited WebRTC support in WASM
- Suitable for learning/prototyping

#### Option 2: React/Next.js (Recommended for Production) ✅
- Full WebRTC support via `@openai/realtime-api-beta`
- Mature ecosystem and libraries
- Better browser compatibility

## 📦 Project Structure

```
recipes_one/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── server/             # Axum backend server
│   ├── frontend/           # Leptos WASM frontend (experimental)
│   ├── models/             # Shared data models
│   ├── db/                 # Database layer (SQLx)
│   └── openai/             # OpenAI client wrapper
├── migrations/             # Database migrations
├── docker-compose.yml      # Docker orchestration
└── README.md               # This file
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (https://rustup.rs)
- Docker & Docker Compose
- OpenAI API key (https://platform.openai.com)

### 1. Environment Setup

```bash
# Copy environment template
cp .env.example .env

# Edit .env and add your OpenAI API key
# OPENAI_API_KEY=sk-your-key-here
```

### 2. Start with Docker

```bash
# Start all services (PostgreSQL + Backend)
docker-compose up -d

# View logs
docker-compose logs -f server

# The server will be available at http://localhost:3000
```

### 3. Manual Development Setup

```bash
# Start PostgreSQL
docker-compose up -d postgres

# Run database migrations
cd crates/server
sqlx database create
sqlx migrate run

# Start the backend server
cargo run -p server

# In another terminal, optionally start the frontend
cd crates/frontend
trunk serve --open
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p models
cargo test -p db
cargo test -p server

# Check code
cargo clippy --all-targets
```

## 📡 API Endpoints

### Authentication
- `POST /api/auth/register` - Create new user
- `POST /api/auth/login` - Login and receive JWT

### Recipes
- `GET /api/recipes` - List user's recipes
- `GET /api/recipes/:id` - Get recipe by ID
- `POST /api/recipes` - Create new recipe
- `POST /api/recipes/:id` - Update recipe (with version control)
- `DELETE /api/recipes/:id` - Delete recipe

### Sessions
- `POST /api/sessions` - Create ephemeral token for Realtime API

### Analytics
- `POST /api/analytics/event` - Log analytics event
- `POST /api/analytics/conversion` - Record conversion
- `GET /api/analytics/recipe/:id` - Get recipe analytics

### Webhook
- `POST /api/webhook/realtime` - OpenAI Realtime webhook handler

## 🛠️ Available Tools (for AI Assistant)

The AI assistant has access to these server-side tools:

### Recipe Management
- `get_recipe` - Fetch canonical recipe
- `apply_recipe_patch` - Update recipe with versioning
- `create_ui_nudges` - Generate contextual UI suggestions
- `merge_recipe` - Combine multiple recipes
- `persist_user_prefs` - Save learned preferences
- `generate_shopping_list` - Create shopping list

### Image Processing
- `extract_recipe_from_image` - OCR and parse recipe photos
- `detect_pantry_items` - Recognize ingredients

### Cost Management
- `pricing_compute` - Estimate recipe cost
- `cost_cutter` - Generate budget-friendly version
- `enrich_recipe` - Upgrade with premium ingredients

### Remixer
- `remix_improvise` - Create variant with pantry items
- `remix_cultural_twist` - Adapt to different cuisine
- `remix_healthy_mode` - Healthier version

### Publisher
- `publish_recipe_card` - Generate recipe card
- `publish_social_media` - Create social posts

## 🔐 Security

- **JWT Authentication**: All protected endpoints require Bearer token
- **Password Hashing**: bcrypt with default cost
- **Webhook Verification**: HMAC signature validation (configure `WEBHOOK_SECRET`)
- **SQL Injection Protection**: SQLx compile-time query verification
- **Server-Side Prompts**: Privileged instructions never sent to client

## 📊 Database Schema

See `migrations/001_init.sql` for full schema. Key tables:

- `users` - User accounts
- `user_profiles` - Preferences (diet, cuisine, health, budget, pantry)
- `recipes` - Canonical recipe state
- `recipe_versions` - Version snapshots
- `sessions` - Realtime sessions
- `cost_catalog` - Cached ingredient pricing
- `cost_runs` - Historical cost analyses
- `remix_runs` - Variant generation history
- `publish_jobs` - Asset generation tracking
- `analytics_events` - User interactions
- `conversions` - Monetization tracking

## 🎨 Frontend Implementation Guide

### React/Next.js Frontend (Recommended)

Since WebRTC in WASM is experimental, here's a recommended React setup:

```bash
# Create Next.js app
npx create-next-app@latest recipe-assistant --typescript

# Install dependencies
cd recipe-assistant
npm install @openai/realtime-api-beta
npm install axios uuid
```

**Key Components:**

1. **RealtimeClient.tsx** - WebRTC connection to OpenAI
2. **RecipeViewer.tsx** - Display canonical recipe
3. **ChipsPanel.tsx** - Contextual action buttons
4. **VoiceControls.tsx** - Start/stop voice session
5. **ImageUploader.tsx** - Upload photos for recognition

**Example RealtimeClient:**

```typescript
import { RealtimeClient } from '@openai/realtime-api-beta';

export class RecipeRealtimeClient {
  private client: RealtimeClient;

  async connect(ephemeralToken: string) {
    this.client = new RealtimeClient({
      apiKey: ephemeralToken,
      dangerouslyAllowAPIKeyInBrowser: true,
    });

    await this.client.connect();

    // Set up event handlers
    this.client.on('conversation.item.created', (event) => {
      // Handle tool calls, responses, etc.
    });
  }
}
```

### Leptos WASM Frontend (Experimental)

The current Leptos implementation provides:
- Basic UI structure
- API client for backend
- Component architecture
- Placeholder WebRTC module

**Limitations:**
- WebRTC support incomplete
- Requires additional WASM bindings
- Browser compatibility concerns

## 🚢 Deployment

### Production Checklist

- [ ] Set strong `JWT_SECRET`
- [ ] Set secure `WEBHOOK_SECRET`
- [ ] Use production OpenAI API key
- [ ] Enable HTTPS/TLS
- [ ] Set up database backups
- [ ] Configure environment variables
- [ ] Set appropriate `RUST_LOG` level
- [ ] Review CORS settings
- [ ] Implement rate limiting
- [ ] Set up monitoring/observability

### Docker Production

```bash
# Build production images
docker-compose build

# Run with production env
docker-compose --env-file .env.production up -d
```

## 📝 Development Notes

### Model Configuration

Models are configurable via the `OpenAIModel` enum in `crates/openai/src/models_config.rs`:

- `RealtimeMini` - Realtime dialogue (gpt-4o-realtime-preview)
- `Gpt4Mini` - Fast structured outputs (gpt-4o-mini)
- `Gpt4` - Complex reasoning (gpt-4o)
- `Gpt4Vision` - Image analysis (gpt-4o)

### Adding New Tools

1. Define tool in `crates/server/src/tools/prompts.rs`
2. Implement handler in appropriate tools module
3. Register in `execute_tool()` match statement
4. Update privileged prompt with tool description

### Database Migrations

```bash
# Create new migration
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## 🐛 Troubleshooting

### Database Connection Issues
```bash
# Check PostgreSQL is running
docker-compose ps postgres

# Reset database
docker-compose down -v
docker-compose up -d postgres
sqlx migrate run
```

### OpenAI API Errors
- Verify `OPENAI_API_KEY` is set correctly
- Check API key permissions
- Review rate limits and quotas

### Frontend WebRTC Issues
- Consider switching to React/Next.js
- Check browser console for errors
- Ensure ephemeral token is valid

## 📚 Resources

- [OpenAI Realtime API Documentation](https://platform.openai.com/docs/guides/realtime)
- [Axum Framework](https://github.com/tokio-rs/axum)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Leptos Framework](https://leptos.dev)

## 📄 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions welcome! Please open an issue or PR.

---

**Built with ❤️ using Rust, Axum, SQLx, and OpenAI Realtime API**
