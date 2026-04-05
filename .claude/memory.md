# Project Memory

## Key Decisions

- Package name: `cuisine` (renamed from `rust` in Cargo.toml)
- Database file: `cuisine.db` in project root
- Testability boundary: `pub fn create_app(pool: SqlitePool) -> Router` in `src/lib.rs`
- DB tests use `#[sqlx::test(migrations = "./migrations")]` — each test gets its own in-memory SQLite
- Shared state via `State<SqlitePool>`, not `Extension`
- All error responses: JSON `{"error": "<message>"}`
- POST returns 201, DELETE returns 204
- `get_recipe_detail` uses 3 queries, not a JOIN, to avoid cartesian product
- Partial updates: fetch → merge Option fields → UPDATE all columns
