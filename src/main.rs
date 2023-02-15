use std::{fs::File, io::{Write}};
use clap::{Arg, App};
use anyhow::{Context, Result};

mod fanatec_decrypt;

fn main() -> Result<()>{
    let matches = App::new("VLFX")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Álex R. <eirexe123@gmail.com>")
        .about("Tool used to decrypt Fanatec steering wheel firmware files")
        .subcommand(App::new("decrypt_firmware")
            .about("Decrypts a Fanatec steering wheel firmware file")
            .arg(Arg::with_name("input_file")
                .short("i")
                .long("input")
                .help("The firmware file to decrypt")
                .value_name("FILE")
                .required(true)
            )
            .arg(Arg::with_name("output_file")
                .short("o")
                .long("output")
                .help("Where to output the decrypted firmware file")
                .value_name("FILE")
                .required(true)
            )
    )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableHelpSubcommand)
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();


    if let Some(ref matches) = matches.subcommand_matches("decrypt_firmware") {
        let input_file_path = matches.value_of("input_file").unwrap();
        let output_file_path = matches.value_of("output_file").unwrap();
        
        let decrypter = fanatec_decrypt::FanatecDecrypter::new()
            .with_context(|| "Could not setup firmware decryption".to_string())?;

        let mut input_file = File::open(input_file_path)
            .with_context(|| format!("Could not open input file: {}", input_file_path))?;

        let decrypted_file_contents = decrypter.decrypt(&mut input_file)
            .with_context(|| "Could not decrypt firmware file".to_string())?;

        let mut output_file = File::create(output_file_path)
            .with_context(|| format!("Could not open output file: {}", output_file_path))?;
        output_file.write_all(&decrypted_file_contents)?;
        println!("Wrote decrypted firmware to {} succesfully!", output_file_path)
    };
    
    Ok(())
}
