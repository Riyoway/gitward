# Gitward

**Keep the right identity on every repository.**

Gitward is a desktop workspace manager for people who juggle multiple Git
identities. Work and personal accounts, a client's org, an alt for open source:
each repository needs the right `user.name`/`user.email`, the right GitHub CLI
account, and the matching credential in the OS store. Get one wrong and you push
as the wrong person, leak an email, or fight an auth prompt.

Gitward makes the correct identity the default. Register your repositories once,
attach the git and GitHub accounts they belong to, and **Sync** aligns local git
config, `gh` auth, and the credential manager in a single action, then launches
your editor or terminal in the right place.

## Why

- **No more wrong-author commits.** The account a repo belongs to is explicit and
  enforced, not remembered.
- **One action, whole identity.** `git config`, `gh auth switch`,
  `gh auth setup-git`, and credential cleanup run as one step with a full log.
- **Auth mismatches, surfaced.** Gitward cross-checks the active `gh` account
  against the credential manager and tells you when they drift.
- **Your tools, one click.** Detects installed editors and terminals and opens
  the repo in the one you choose.

## How it works

The frontend holds all UI and state; every side effect (running `git`, `gh`,
the credential manager, spawning tools) is delegated to a stateless Rust
backend over Tauri IPC. External commands go through a single `CommandRunner`
abstraction, so every result carries `stdout`, `stderr`, and exit code, and the
backend is testable without touching a real `git` or `gh`.

```
React (HeroUI · Zustand · TanStack Query)
        │  Tauri IPC (invoke)
Rust (commands → services → git / gh / credential manager / OS)
```


## Tech stack

| Layer      | Choice                                             |
| ---------- | -------------------------------------------------- |
| Shell      | [Tauri 2](https://tauri.app)                       |
| UI         | React 19, [HeroUI](https://heroui.com), Tailwind 3 |
| State      | Zustand (client) · TanStack Query (external state) |
| Backend    | Rust                                               |
| Persistence| Tauri Store plugin (JSON)                          |
| i18n       | i18next (English · 日本語)                          |

## Development

Prerequisites: [Node](https://nodejs.org) ≥ 20, the
[Rust toolchain](https://www.rust-lang.org/tools/install), and the
[Tauri 2 system dependencies](https://tauri.app/start/prerequisites/) for your
platform.

```bash
npm install
npm run tauri dev      # run the app with hot reload
npm run tauri build    # produce a release bundle
```

Frontend-only checks:

```bash
npm run lint
npm run typecheck
npm run test
```

Backend checks:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
```

## License

[MIT](LICENSE) © Riyoway
