use std::{fs::File, io::{Write}};
use clap::{Arg, App};
use anyhow::{Context, Result};

mod fanatec_key_extract;
mod fanatec_decrypt;

// "C:\Program Files\Fanatec\Fanatec Wheel\fw\FwClubSportBaseUpdater.exe"
fn main() -> Result<()>{
    let matches = App::new("VLFX")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Álex R. <eirexe123@gmail.com>")
        .about("Tool used to decrypt Fanatec steering wheel firmware files")
        .subcommand(App::new("extract_key")
            .about("Extracts the decryption key from FwClubSportBaseUpdater.exe")
            .arg(Arg::with_name("input_file")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("The FwClubSportBaseUpdater.exe file to extract the key from")
                .required(true)
            )
            .arg(Arg::with_name("output_file")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Where to output the key file")
                .default_value("fanatec_key.key")
                .required(true)
            )
        )
        .subcommand(App::new("decrypt_firmware")
            .about("Decrypts a Fanatec steering wheel firmware file with the given key")
            .arg(Arg::with_name("key_file")
                .short("k")
                .long("key")
                .help("The key file to use")
                .value_name("FILE")
                .required(true)
            )
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


    if let Some(ref matches) = matches.subcommand_matches("extract_key") {
        let file_path = matches.value_of("input_file").unwrap();
        let file = File::open(file_path).with_context(|| format!("Could not open input file: {}", file_path))?;
        let key = fanatec_key_extract::extract_key(file).with_context(|| "Could not extract key".to_string())?;

        let out_file_path = matches.value_of("output_file").unwrap();
        
        let mut output_file = File::create(out_file_path).with_context(|| format!("Could not open output file: {}", out_file_path))?;
        output_file.write(&key).with_context(|| "Could not write to outuput file".to_string())?;

        println!("The key is: {}", &String::from_utf8_lossy(&key));
        println!("Key written to file {} succesfully!", out_file_path);
    };

    if let Some(ref matches) = matches.subcommand_matches("decrypt_firmware") {
        let input_file_path = matches.value_of("input_file").unwrap();
        let output_file_path = matches.value_of("output_file").unwrap();
        let key_file_path = matches.value_of("key_file").unwrap();

        let key = std::fs::read(key_file_path)?;
        
        let decrypter = fanatec_decrypt::FanatecDecrypter::new(&key)
            .with_context(|| "Could not setup firmware decryption".to_string())?;

        let mut input_file = File::open(input_file_path)
            .with_context(|| format!("Could not open input file: {}", input_file_path))?;

        let decrypted_file_contents = decrypter.decrypt(&mut input_file)
            .with_context(|| "Could not decrypt firmware file".to_string())?;

        let mut output_file = File::create(output_file_path)
            .with_context(|| format!("Could not open output file: {}", output_file_path))?;
        output_file.write(&decrypted_file_contents)?;
        println!("Wrote decrypted firmware to {} succesfully!", output_file_path)
    };
    
    Ok(())
}
