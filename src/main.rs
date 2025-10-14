pub mod content;

pub fn main() -> anyhow::Result<()> {
    let mut langs = glados_highlight::language::Lang::ALL
        .iter()
        .copied()
        .collect::<Vec<_>>();

    langs.sort_by_key(|l| l.func().into_raw());

    langs.iter().for_each(|lang| {
        println!(
            "{}: {:p}",
            lang.names().first().copied().unwrap_or("{UNKNOWN}"),
            lang.func().into_raw()
        )
    });
    Ok(())
}
