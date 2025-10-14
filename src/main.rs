pub mod content;

pub fn main() -> anyhow::Result<()> {
    glados_highlight::language::Lang::ALL
        .iter()
        .for_each(|lang| {
            println!(
                "{}: {:p}",
                lang.names().first().copied().unwrap_or("{UNKNOWN}"),
                lang.func().into_raw()
            )
        });
    Ok(())
}
