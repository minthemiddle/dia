# Dia - Smart Work Diary

Dia is a command-line tool for tracking your work with semantic tagging and SQLite storage.

## Features

- Log work entries with semantic tags:
  - People: `@name`
  - Projects: `%project` 
  - Tags: `#tag`
- Full-text search of entries
- Filter entries by date, people, projects and tags
- SQLite backend for reliable storage
- Cross-platform (macOS, Linux, Windows)

## Installation

### Using Pre-built Binaries

1. Go to the [Releases page](https://github.com/yourusername/dia/releases)
2. Download the appropriate binary for your platform:
   - Windows: `dia-windows-x86_64.exe`
   - macOS: `dia-macos-x86_64`
   - Linux: `dia-linux-x86_64`
3. Make the binary executable (macOS/Linux):
   ```bash
   chmod +x dia-*-x86_64
   ```
4. Move it to your PATH (example for macOS/Linux):
   ```bash
   sudo mv dia-*-x86_64 /usr/local/bin/dia
   ```
5. Verify installation:
   ```bash
   dia --version
   ```

### Building from Source

If you prefer to build from source:

```bash
cargo install --path .
```

This will install `dia` to `~/.cargo/bin/dia`

### System Requirements

- Rust 1.65+
- SQLite 3.35+

## Usage

### Logging Entries

```bash
dia log "Completed the diary feature"
dia log "Worked on %Dia #data-model with @JohnK" --date 2024-03-15
```

### Viewing Entries

```bash
# Show all entries
dia show entries

# Filter entries
dia show entries --date 2024-03-15
dia show entries --search "diary"
dia show entries --person JohnK
dia show entries --project Dia
dia show entries --tag data-model
```

### Viewing Entities

```bash
dia show people
dia show projects
dia show tags
```

### Statistics

```bash
dia stats --period "last week"
```

### Database Access

```bash
# Open the database file with your system's default application
dia db
```

### Review

```bash
dia review
```

## Configuration

Configuration is stored in `~/.config/dia/config.toml`. The main setting is:

```toml
diary_db_path = "/path/to/diary.db"
```

## Database Schema

The SQLite database contains these tables:

- `entries`: Main diary entries
- `people`, `projects`, `tags`: Semantic entities
- `entry_people`, `entry_projects`, `entry_tags`: Relationships
- `entries_fts`: Full-text search index

## Roadmap

### Core Features
- [x] Basic entry logging
- [x] Semantic tagging
- [x] SQLite storage
- [x] Database file access
- [ ] Full-text search
- [ ] Advanced filtering
- [ ] Statistics and insights
- [ ] Spaced repetition review

## Contributing

Contributions are welcome! Please open an issue or pull request.

## License

MIT
