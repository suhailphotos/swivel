# swivel

**One trait, many databases.** A tiny Rust crate that lets you write one interface and swap backends like Notion, Supabase, or Postgres without changing your call sites.

> Status: early scaffold. API may change.

## Why “swivel”
It pivots the same API toward different data backends.

## Highlights
- **Trait-first design**: Define behavior once, implement per backend.
- **Generic or dynamic**: Use generics for zero-cost static dispatch, or trait objects when the backend is chosen at runtime.
- **Backend-agnostic types**: Keep your domain models separate from storage concerns.
- **Small surface**: Start with `get` / `put`, grow as you need.

## Quick start

If you’re bootstrapping a new repo with your script:

```bash
# after creating the repo with scripts/rust/bootstrap_repo.sh ...
cd "$MATRIX/crates/swivel"  # or your chosen clone path
cargo build
cargo run -- --help
```

> Note: your bootstrap script currently initializes a **binary** crate. If you prefer a **library** crate, change the `cargo init` step to `cargo init --lib` (see “Switch to a library crate” below).

## Minimal design

### The trait

```rust
/// `Database` captures the storage operations you care about.
/// Start small; you can split into multiple traits later (Reads, Writes, Pages, etc.).
pub trait Database {
    type Record;
    fn get(&self, id: &str) -> Result<Self::Record, Error>;
    fn put(&self, rec: Self::Record) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error { Oops }
```

### Two tiny backends

```rust
pub struct Notion;    // placeholder; put tokens/clients inside later
pub struct Supabase;  // placeholder

impl Database for Notion {
    type Record = String;
    fn get(&self, id: &str) -> Result<Self::Record, Error> {
        Ok(format!("notion:{{{}}}", id))
    }
    fn put(&self, rec: Self::Record) -> Result<(), Error> {
        println!("Notion PUT {rec}");
        Ok(())
    }
}

impl Database for Supabase {
    type Record = i64;
    fn get(&self, id: &str) -> Result<Self::Record, Error> {
        Ok(id.parse().unwrap_or(0))
    }
    fn put(&self, rec: Self::Record) -> Result<(), Error> {
        println!("Supabase PUT {rec}");
        Ok(())
    }
}
```

### Static dispatch (generic)

```rust
pub fn sync_one<D: Database>(db: &D, id: &str) -> Result<(), Error> {
    let rec = db.get(id)?;
    db.put(rec)
}
```

### Dynamic dispatch (trait object)

```rust
pub fn sync_dyn(db: &dyn Database<Record = String>, id: &str) -> Result<(), Error> {
    let rec = db.get(id)?;
    db.put(rec)
}
```

## CLI (super minimal)

No deps version, using `std::env`:

```rust
fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    // usage: swivel <backend> <op> <id>
    // e.g.,  swivel notion get abc
    if args.len() < 4 { eprintln!("usage: swivel <notion|supabase> <get|put> <id-or-value>"); std::process::exit(2); }

    let backend = args[1].as_str();
    let op = args[2].as_str();

    match (backend, op) {
        ("notion", "get")    => { let n = Notion; println!("{}", n.get(&args[3])?); }
        ("notion", "put")    => { let n = Notion; n.put(args[3].clone())?; }
        ("supabase", "get")  => { let s = Supabase; println!("{}", s.get(&args[3])?); }
        ("supabase", "put")  => { let s = Supabase; s.put(args[3].parse().unwrap_or(0))?; }
        _ => { eprintln!("unknown command"); std::process::exit(2); }
    }
    Ok(())
}
```

## Switch to a library crate

If you want `swivel` to be a **library** (recommended) with an optional CLI:

1. Change your bootstrap script’s `cargo init` line to:
   ```bash
   cargo init --lib --name "${CARGO_NAME}" --edition "${EDITION}" --vcs none
   ```
2. Create `src/lib.rs` with the trait and backends.
3. Add a small binary at `src/bin/swivel.rs` that calls into the library.

## Suggested layout

```
swivel/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # traits, error, prelude
│   ├── backends/
│   │   ├── notion.rs
│   │   └── supabase.rs
│   └── bin/
│       └── swivel.rs       # optional CLI
└── examples/
    └── basics.rs
```

## Roadmap
- [ ] Real clients for Notion and Supabase
- [ ] Feature flags per backend (`notion`, `supabase`, `postgres`)
- [ ] Common model traits (`Serializable`, `Identifiable`)
- [ ] Async support (feature `async`)
- [ ] Error enums per backend with `From` conversions

## License
MIT © You
