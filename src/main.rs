// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

mod db_chirho;
mod hebrew_chirho;
mod import_mapm_chirho;
mod import_solid_rock_chirho;
mod transpose_chirho;

use anyhow::Result as ResultChirho;
use clap::{Parser as ParserChirho, Subcommand as SubcommandChirho};

#[derive(ParserChirho)]
#[command(name = "accent-transpose-chirho")]
#[command(about = "Hebrew Bible cantillation transposition from MapM to Solid Rock")]
struct CliChirho {
    #[command(subcommand)]
    command_chirho: CommandsChirho,
}

#[derive(SubcommandChirho)]
enum CommandsChirho {
    /// Import MapM JSON into SQLite
    #[command(name = "import-mapm-chirho")]
    ImportMapmChirho,

    /// Import Solid Rock TEI XML into SQLite
    #[command(name = "import-solid-rock-chirho")]
    ImportSolidRockChirho,

    /// Transpose cantillation marks from MapM onto Solid Rock
    #[command(name = "transpose-chirho")]
    TransposeChirho,

    /// Generate statistics report
    #[command(name = "report-chirho")]
    ReportChirho,

    /// Export cantillated text
    #[command(name = "export-chirho")]
    ExportChirho,
}

fn main() -> ResultChirho<()> {
    let cli_chirho = CliChirho::parse();

    let conn_chirho = db_chirho::open_db_chirho()?;
    db_chirho::create_tables_chirho(&conn_chirho)?;

    match cli_chirho.command_chirho {
        CommandsChirho::ImportMapmChirho => {
            println!("Importing MapM JSON...");
            import_mapm_chirho::import_mapm_chirho(&conn_chirho)?;
            println!("MapM import complete.");
        }
        CommandsChirho::ImportSolidRockChirho => {
            println!("Importing Solid Rock TEI XML...");
            import_solid_rock_chirho::import_solid_rock_chirho(&conn_chirho)?;
            println!("Solid Rock import complete.");
        }
        CommandsChirho::TransposeChirho => {
            println!("Transposing cantillation marks...");
            transpose_chirho::transpose_chirho(&conn_chirho)?;
        }
        CommandsChirho::ReportChirho => {
            println!("Report not yet implemented.");
        }
        CommandsChirho::ExportChirho => {
            println!("Export not yet implemented.");
        }
    }

    Ok(())
}
