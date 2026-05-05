<p align="center">
  <img src="../public/logo-dark.svg" alt="duck-diag examples" width="120"/>
</p>

<h1 align="center">duck-diag examples</h1>

<p align="center">
  Runnable examples showing duck-diag in different domains.
</p>

<p align="center">
  <a href="../LICENSE">MIT</a> -
  <a href="../README.md">repo</a>
</p>

---

## Run

```sh
cargo run --example <name>
```

## Catalog

| example | what it shows |
| --- | --- |
| `demo` | simplest diagnostic with one label + suggestion |
| `compiler` | language-front-end shape: lexer / parser / type errors |
| `multi_file` | diagnostics referencing more than one source file |
| `sql_engine` | SQL parser hooking unknown table / ambiguous column |
| `config_linter` | YAML / TOML / JSON validator with hints |
| `api_validator` | REST request / response shape validation |
| `json_output` | stable JSON envelope (feature `json`) |
| `diff_suggestions` | structured-edit suggestions ready for IDE quick-fix |
| `suggestion` | hint formatting + applicability levels |
| `showcase` | every renderer combined |

Each is single-file under `examples/`. Tweak and re-run to learn the
API.

## Output

Color-aware terminal renderer + stable JSON when the `json` feature
is on. See [the root README](../README.md) for the matrix.
