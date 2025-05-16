use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::fs::File;
use uuid::Uuid;

const PACKAGE: &str = "guest";

fn main() {
    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with verification

    let f = File::open("./src/diabetes.csv").expect("unable to find dataset");
    let mut rdr = csv::Reader::from_reader(f);

    let mut records: Vec<guest::Record> = Vec::new();
    rdr.records().for_each(|maybe_record| {
        if let Ok(record) = maybe_record {
            let mut simple = guest::Record::new();

            record.iter().for_each(|field| {
                simple.push_field(field);
            });

            records.push(simple);
        }
    });

    let uuids = (0..records.len()).map(|_| {
        Uuid::new_v4()
    }).collect();

    println!("Proving execution of vm...");
    let (view, proof) = prover.prove_with_input::<Vec<guest::Record>, Vec<Uuid>>(&records, &uuids).expect("failed to prove program");

    println!(
        ">>>>> Logging\n{}<<<<<",
        view.logs().expect("failed to retrieve debug logs").join("")
    );
    assert_eq!(
        view.exit_code().expect("failed to retrieve exit code"),
        nexus_sdk::KnownExitCodes::ExitSuccess as u32
    );

    print!("Verifying execution...");

    #[rustfmt::skip]
    proof
        .verify_expected::<Vec<Uuid>, Vec<guest::Record>>(
            &uuids,
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &view.public_output().expect("failed to retrieve output"),
            &elf,
            &[],
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");
}
