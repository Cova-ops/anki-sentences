# 🦀 Anki Sentences – Rust CLI for Spaced Repetition Learning

A **production-grade Rust CLI application** inspired by **Anki / SRS systems**, focused on learning **sentences and vocabulary** with **audio support**, **persistent review history**, and **clean, scalable architecture**.

This repository is intentionally designed as both:

- a **real usable learning tool**, and
- a **portfolio project** showcasing modern Rust engineering practices.

---

## ✨ Features

- 📚 Sentence & vocabulary training
- 🔁 Custom **Spaced Repetition System (SRS)** inspired by Anki (SM-2–like)
- 🧠 Adaptive review scheduling based on user performance
- 🔊 **Text-to-Speech audio** (ElevenLabs API)
- 💾 **SQLite database** with migrations & repositories
- 🧩 **Procedural macros** to reduce DB boilerplate
- 🧪 Automated tests with **snapshot testing (`insta`)**
- ⚙️ **GitHub Actions CI**
- 🧱 Modular, scalable folder architecture
- 🔐 Environment-based configuration (`.env`)

---

## 🏗 Project Structure

The project is organized as a **Cargo workspace**, separating concerns clearly between
application logic, shared traits, and procedural macros.

```text
.
├── app/ # Main application crate
│   ├── src/
│   ├── console/ # CLI interaction & user input/output
│   ├── db/ # Database layer
│   │   ├── repositories/ # Repository pattern (SQL access)
│   │   ├── schemas/ # Domain schemas
│   │   └── migrations/ # SQL migrations
│   │
│   ├── helpers/ # Cross-cutting helpers
│   │   ├── audio/ # Audio generation & playback
│   │   ├── console/ # CLI helpers
│   │   ├── time/ # Date & time utilities
│   │   └── string/ # String utilities
│   │
│   ├── services/ # Domain services / orchestration
│   ├── traits/ # Shared traits (conversion, abstractions)
│   ├── utils/ # Small reusable utilities
│   └── main.rs # Application entry point
│
├── sql_model/ # Shared DB traits
│   └── src/
│   └── lib.rs # SqlNew, SqlRaw, FromRaw traits
│
├── sql_model_derive/ # Procedural macros crate
│   └── src/
│   └── lib.rs # #[derive(SqlModel)]
│
├── assets/
│   ├── audios/ # Cached TTS audio files
│   │   ├── worte/
│   └   └── setze/
│
├── data/ # Seed / import data
├── .env # Environment variables (not committed)
├── Cargo.toml # Workspace manifest
├── Cargo.lock
├── README.md
└── anki_satze.sql # Schema / reference SQL
```

---

## 🧠 Spaced Repetition System (SRS)

A custom review algorithm inspired by **Anki’s SM-2**, adapted for CLI usage.

Each review stores:

- `interval` – days until next review
- `ease_factor` – difficulty coefficient
- `repetitions` – successful recall count
- `last_review`
- `next_review`

### Review Quality Scale

| Value | Meaning               |
| ----: | --------------------- |
|     0 | Incorrect / forgotten |
|     1 | Correct with effort   |
|     2 | Easy recall           |

Intervals grow dynamically based on past performance.

---

## 🔊 Audio System (Text-to-Speech)

- Uses **ElevenLabs API**
- Audio generated **once** and cached locally
- Playback is **non-blocking**
- Supports both words and sentences

Audio files are stored locally:

```text
assets/audios/
├── worte/
└── setze/
```

This avoids repeated API calls and keeps the app fast and offline-friendly.

---

## 🗄 Database Layer

- **SQLite** for portability
- Explicit SQL (no ORM magic)
- Repository pattern per domain
- Foreign keys & constraints enforced
- Bulk inserts wrapped in transactions

Example repository signature:

```rust
pub fn bulk_insert_tx(
    tx: &Transaction,
    data: &[NewSchema]
) -> Result<Vec<Schema>>
```

## 🧩 Procedural Macros (`SqlModel`)

To reduce repetitive DB mapping code, the project includes a custom derive macro: **`SqlModel`**.

This macro helps generate common boilerplate for models used with `rusqlite`, such as:

- `to_params()` → for INSERT/UPDATE parameter binding
- `from_sql()` → for mapping a `rusqlite::Row` into a `Raw*` struct
- `from_raw()` / `from_vec_raw()` → conversion into the final domain schema

Example:

```rust
#[derive(Debug, Clone, SqlModel)]
#[sql(
    insert(code, name),
    raw(id, code, name, created_at, deleted_at)
)]
pub struct NewGramTypeSchema {
    pub code: String,
    pub name: String,
}
```

The goal is to keep:

- 🧠 **Explicit SQL**  
  No hidden ORM logic. All queries are written by hand so behavior is always clear and debuggable.

- 🧩 **Strongly-typed schemas**  
  Each table is represented by three explicit structs:
  - `Raw*Schema` → raw DB representation (strings, ints, nullable fields)
  - `New*Schema` → insert/update payloads
  - `*Schema` → final domain model with rich types (`DateTime`, `bool`, relations, etc.)

- 🔁 **Consistent conversions**  
  The `SqlModel` derive macro guarantees a uniform approach for:
  - row → raw
  - raw → domain
  - domain → SQL params

- ✂️ **Minimal boilerplate**  
  Procedural macros remove repetitive code while preserving full control over SQL and data flow.

---

## 🧪 Testing Strategy

Testing is a first-class concern in this project.

Tools used:

- Rust built-in test framework (`#[test]`)
- **Snapshot testing** with `insta`
- In-memory SQLite databases for repository tests
- Deterministic outputs (timestamps normalized where needed)

Example:

```rust
insta::assert_debug_snapshot!(result);
```

Snapshots are reviewed explicitly, making regressions easy to detect when:

- SQL changes
- schema evolution occurs
- repository logic is refactored

---

## ⚙️ CI – GitHub Actions

The project includes a GitHub Actions pipeline that runs on every push and pull request.

Steps:

- cargo build
- cargo test

On Linux runners, audio dependencies are required due to rodio / cpal:

```bash
sudo apt-get install -y pkg-config libasound2-dev
```

This ensures the project builds consistently across environments.

---

## 🔐 Configuration & Secrets

Sensitive data is never committed.

Configuration is handled via:

- .env files (ignored by git)
- dotenvy for loading environment variables
- Cached access via once_cell::Lazy

Example:

```env
ELEVENLABS_API_KEY=your_api_key_here
```

This allows safe API usage in local development and CI.

---

## 🚧 Planned Improvements

Planned enhancements to demonstrate scalability and production-ready design:

- 📡 MQTT integration
  - Event-driven persistence
  - Decouple user interaction from database writes
  - Prevent transaction loss on crashes or interruptions
- ☁️ Optional cloud synchronization
- 📊 Review statistics and learning analytics
- 🔄 Import / export decks
- 🎧 Audio caching & prefetching

---

## 🎯 Why This Project?

This repository demonstrates real-world Rust development practices:

- Clear modular architecture
- Ownership and lifetimes in non-trivial flows
- Error handling with color-eyre
- SQLite repositories with transactional safety
- Procedural macros to eliminate boilerplate
- Snapshot-based testing (insta)
- CI automation

This is not a toy project — it is designed as a solid, extensible foundation for a production application.

---

## 👤 Author

Developed by **Daniel**  
Software Engineer focused on **Rust**, backend systems, and clean architecture.

🌐 Portfolio: https://portfolio.dacovasan.dev
