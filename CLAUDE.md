# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

French cuisine REST API written in Rust. Personal reference application for French culinary terminology and recipes with bilingual (French/Japanese) content. No authentication — single-user.

## Commands

```bash
# First-time setup
cargo install sqlx-cli --no-default-features --features sqlite

# Build & run
cargo build
cargo run                        # starts server on :3000, creates cuisine.db, runs migrations

# Testing
cargo test                       # all tests (unit + integration)
cargo test <name>                # filter by test name substring
cargo test --test terms_api      # run specific integration test file

# Lint & format
cargo clippy -- -D warnings
cargo fmt

# Security audit
cargo audit --ignore RUSTSEC-2023-0071
# RUSTSEC-2023-0071 (rsa Marvin Attack) is ignored because it comes via sqlx-mysql,
# which sqlx's macro layer unconditionally compiles even though this project uses SQLite only.
# See audit.toml for details. Re-evaluate when a fix for rsa becomes available.

# Migrations (when adding new tables/columns)
sqlx migrate add <name>          # create new migration file in migrations/
sqlx migrate run                 # apply pending migrations to cuisine.db
sqlx migrate revert              # revert last migration

# Database inspection
sqlite3 cuisine.db
```

## Architecture

```
src/
  main.rs          tokio entry point; connects pool, runs migrations, serves on :3000
  lib.rs           pub fn create_app(pool: SqlitePool) -> Router  ← testability boundary
  error.rs         AppError (NotFound | Db | BadRequest) implementing IntoResponse → JSON
  models/
    term.rs        Term, NewTerm, UpdateTerm, TermQuery structs
    recipe.rs      Recipe, RecipeDetail (with ingredients+steps), NewRecipe, UpdateRecipe
  db/
    terms.rs       raw sqlx queries: list_terms / get_term / create_term / update_term / delete_term
    recipes.rs     raw sqlx queries: list_recipes / get_recipe_detail / create_recipe / update_recipe / delete_recipe
  routes/
    terms.rs       axum handlers for /terms
    recipes.rs     axum handlers for /recipes
migrations/
  0001_create_terms.sql
  0002_create_recipes.sql
  0003_create_recipe_steps.sql
  0004_create_recipe_ingredients.sql
tests/
  terms_api.rs     integration tests (HTTP layer)
  recipes_api.rs   integration tests (HTTP layer)
```

## API

```
GET    /terms              ?category=dish|ingredient|utensil|technique  &q=<search>
GET    /terms/:id
POST   /terms              → 201
PUT    /terms/:id
DELETE /terms/:id          → 204

GET    /recipes
GET    /recipes/:id        includes ingredients[] and steps[]
POST   /recipes            → 201
PUT    /recipes/:id
DELETE /recipes/:id        → 204
```

## Data Model

```
terms              id, french, japanese, category CHECK(dish|ingredient|utensil|technique), notes, created_at
recipes            id, name_french, description_japanese, difficulty CHECK(easy|medium|hard), created_at
recipe_steps       id, recipe_id→recipes, step_number, instruction_french, instruction_japanese
recipe_ingredients recipe_id→recipes, term_id→terms, quantity, notes  PK(recipe_id,term_id)
```

## Key Design Decisions

- **No ORM**: raw sqlx queries with `FromRow` derive for full SQL control
- **Shared state**: `State<SqlitePool>` (not `Extension`) — type-checked at compile time
- **Migrations**: embedded via `sqlx::migrate!()` in both `main.rs` and tests; no runtime CLI needed
- **Testability**: `create_app(pool)` in `lib.rs` lets integration tests inject an in-memory pool without a TCP server
- **DB tests**: `#[sqlx::test(migrations = "./migrations")]` gives each test an isolated SQLite instance
- **Partial update pattern**: fetch existing row → merge `Option` fields → UPDATE all columns (avoids dynamic SQL)
- **`get_recipe_detail`**: uses 3 separate queries (recipe + ingredients JOIN terms + steps) assembled in Rust, not one cartesian JOIN
- **DATABASE_URL**: required at compile time for sqlx query macros; store in `.env` (not committed)

## Rust Robustness Guidelines

### Error Handling
- Avoid `unwrap()` / `expect()` in non-test code — use `?` or `match`
- Use a custom error enum (`AppError`) with `thiserror` or manual `impl`; already done in `error.rs`
- Attach context to errors with `.map_err` or `anyhow::Context` before propagating

### Type System
- **Newtype pattern**: wrap primitives in `struct UserId(i64)` to prevent value mix-ups
- Model domain states with `enum`, not bare `String`, so invalid values are rejected at compile time
- Prefer constrained standard types (`NonZeroU32`, etc.) where applicable

### Input Validation
- Validate at the boundary (DTO structs like `NewTerm`, `UpdateTerm`) before touching the DB
- Combine DB-level `CHECK` constraints with Rust-side `enum` for double protection

### Concurrency & Memory
- Prefer inherently thread-safe types (`SqlitePool` via `State<SqlitePool>`) over `Arc<Mutex<T>>`
- Audit `clone()` calls — avoid cloning large structures unnecessarily

### Testing
- Use `#[sqlx::test(migrations = "./migrations")]` for isolated per-test DB instances (already adopted)
- Cover error paths: 404, 400, duplicate key, constraint violation — not only happy paths
- Run `cargo test` and `cargo clippy -- -D warnings` together; treat any warning as a failure

### Toolchain
- `cargo audit --ignore RUSTSEC-2023-0071` — re-evaluate the ignore when an rsa fix lands
- `cargo fmt` — enforce consistent formatting in CI
- Consider `cargo deny` for license/dependency auditing if the dependency tree grows

## Term Registration Rules

When adding or editing entries in `dishes`, `ingredients`, `utensils`, `techniques`:

### Notes style
- Write notes primarily in Japanese.
- Any concept that corresponds to a registered term (in any of the four tables) **must** be written in its French form — not Japanese — so `linkNotes` auto-links it.
- Do **not** wrap French terms in parentheses. Write them inline as plain text.

### Adding missing terms (recursive self-containment)
1. If a French term appears in a notes field and is not yet registered, add it.
2. After writing notes for the newly added term, check whether any French terms in those notes are also unregistered — if so, add them too.
3. Repeat until no new unregistered French terms remain (self-contained).

### Partial-link safety
- `linkNotes` uses substring matching without word boundaries.
- Do **not** write a word in notes that contains a registered term as a prefix/substring (e.g. writing `tamiser` when `tamis` is registered creates a broken half-link).
- If a longer phrase (e.g. `beurre d'escargot`) includes a registered shorter term (`beurre`, `escargot`), register the full phrase — the longer match wins and suppresses the partial match.

### Project-Specific Improvements (Backlog)
- `AppError::BadRequest` could carry `{ field: &'static str, reason: String }` for richer error responses
- `term.rs` `category` field: migrate from `String` to `enum Category` so invalid categories are rejected by the type system, not only by the DB `CHECK` constraint
