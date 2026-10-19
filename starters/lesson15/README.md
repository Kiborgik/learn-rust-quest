# Lesson 15 · The Archive 📜

> serde: one set of derives, every format.

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.9"
```

```rust
#[derive(Serialize, Deserialize)]
struct Hero { name: String, level: u32 }

// to_string_pretty for indented
let json = serde_json::to_string(&hero)?;
let hero: Hero = serde_json::from_str(&json)?;
let cfg: Config = toml::from_str(&text)?;
```

## Attributes

| Attribute | Where | Does |
|---|---|---|
| `rename = "hp"` | field | different name in the file |
| `rename_all = "camelCase"` | struct/enum | rename every field/variant |
| `default` | struct/field | missing → default value |
| `skip_serializing_if = "Option::is_none"` | field | leave out when None |
| `skip` | field | never read or written |
| `deny_unknown_fields` | struct | extra fields are an error |
| `flatten` | field | inline a nested struct's fields |

## Enum shapes

| Attribute | `Fireball { power: 40 }` becomes |
|---|---|
| (none) | `{"Fireball":{"power":40}}` |
| `tag = "type"` | `{"type":"Fireball","power":40}` |
| `tag = "t", content = "c"` | `{"t":"Fireball","c":{"power":40}}` |
| `untagged` | `{"power":40}` |

## Dynamic JSON

```rust
let v: serde_json::Value = serde_json::from_str(text)?;
v["party"][0]["name"].as_str();       // Option<&str>, never panics
let e = serde_json::json!({ "event": "win", "xp": 50 });
```

## Coming from other languages

- Like Jackson / System.Text.Json / Pydantic, but generated at compile time.
- No reflection: the shape is fixed by your types, and mismatches are errors.
- Evolving formats: add new fields with `#[serde(default)]`.

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's files back to the start (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
