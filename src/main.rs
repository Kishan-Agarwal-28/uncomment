mod emitter;
mod lexer;
mod rules;
mod token;

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use rules::{build_index, builtin_config};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "uncomment",
    about = "Strip (or list) comments in source files using a language-aware lexer",
    version
)]
struct Cli {
    #[arg(required = true)]
    files: Vec<PathBuf>,

    #[arg(short, long, value_name = "EXT")]
    lang: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(long, value_enum, default_value_t = Mode::Strip)]
    mode: Mode,

    #[arg(long)]
    dry_run: bool,

    #[arg(long, value_name = "FILE")]
    langs: Option<PathBuf>,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Strip,

    List,

    Count,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    if cli.output.is_some() && cli.files.len() > 1 {
        bail!("--output can only be used with a single input file");
    }

    let mut config = builtin_config();
    if let Some(ref path) = cli.langs {
        let src =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let extra: rules::LanguageConfig =
            toml::from_str(&src).with_context(|| format!("parsing {}", path.display()))?;

        config.language.extend(extra.language);
    }
    let index: HashMap<String, rules::LanguageRules> = build_index(config.language);

    for file in &cli.files {
        process_file(file, &cli, &index)
            .with_context(|| format!("processing {}", file.display()))?;
    }
    Ok(())
}

fn process_file(
    path: &Path,
    cli: &Cli,
    index: &HashMap<String, rules::LanguageRules>,
) -> Result<()> {
    let ext = cli.lang.clone().unwrap_or_else(|| {
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
    });

    let lang_rules = index.get(&ext).with_context(|| {
        format!(
            "no language rules for extension '.{ext}'\n  \
             hint: use --lang <ext> to override, or --langs <file> to add a definition"
        )
    })?;

    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

    let chars: Vec<char> = source.chars().collect();
    let tokens = lexer::Lexer::new(&chars, lang_rules).tokenize();

    match cli.mode {
        Mode::List => {
            println!("── {} ({}) ──", path.display(), lang_rules.name);
            emitter::emit_list(&tokens);
        }

        Mode::Count => {
            let (lc, bc) = emitter::emit_count(&tokens);
            println!(
                "{}: {} line comment{}, {} block comment{}",
                path.display(),
                lc,
                if lc == 1 { "" } else { "s" },
                bc,
                if bc == 1 { "" } else { "s" },
            );
        }

        Mode::Strip => {
            let stripped = emitter::emit_stripped(&tokens);

            if cli.dry_run {
                print!("{stripped}");
                return Ok(());
            }

            let dest = cli.output.as_deref().unwrap_or(path);

            let tmp = dest.with_extension("uncomment.tmp");
            fs::write(&tmp, &stripped).with_context(|| format!("writing {}", tmp.display()))?;
            fs::rename(&tmp, dest)
                .with_context(|| format!("renaming temp file to {}", dest.display()))?;

            println!("stripped  {}  ({})", dest.display(), lang_rules.name);
        }
    }

    Ok(())
}
