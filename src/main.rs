mod model;
use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Command line arguments
struct Args {
    #[argh(positional)]
    /// input CSV file path
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    if args.input.exists() && args.input.is_file() {
        // Proceed with processing the file
        let mut reader = csv::Reader::from_path(&args.input)
            .unwrap_or_else(|_| panic!("Failed to open file: {}", args.input.display()));
        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: model::CSVGautrainRecord = result.unwrap_or_else(|_| {
                panic!(
                    "Failed to parse CSV record in file: {}",
                    args.input.display()
                )
            });
            let gautrain_record: model::GautrainRecord = record.into();
            records.push(gautrain_record);
        }

        // group records by date
        records.sort_by(|a, b| a.transaction_date.0.cmp(&b.transaction_date.0));
        let mut grouped_records: Vec<Vec<model::GautrainRecord>> = Vec::new();
        for record in records {
            if let Some(last_group) = grouped_records.last_mut() {
                if last_group.last().unwrap().is_same_date(&record) {
                    last_group.push(record);
                    continue;
                }
            }
            grouped_records.push(vec![record]);
        }

        // process grouped records
        // Select which days to include
        let choices = grouped_records.iter().map(|group| {
            let date = group.first().unwrap().transaction_date.0.date_naive();
            let journey = group
                .iter()
                .map(|r| r.site.clone())
                .collect::<Vec<_>>()
                .join(" -> ");
            let total_topup: f32 = group
                .iter()
                .filter(|r| r.is_topup())
                .map(|r| r.transaction_value)
                .sum();
            let total_spent: f32 = group
                .iter()
                .filter(|r| !r.is_topup())
                .map(|r| r.transaction_value)
                .sum();
            let label = format!(
                "{} - {} - Top-up: R{:.2}, Spent: R{:.2}",
                date, journey, total_topup, total_spent
            );
            label
        });

        let selections = dialoguer::MultiSelect::new()
            .with_prompt("Select the days to include in the summary:")
            .items(&choices.collect::<Vec<_>>())
            .defaults(&vec![true; grouped_records.len()])
            .interact()?;
        let mut final_records: Vec<model::GautrainRecord> = Vec::new();
        for index in selections {
            final_records.extend(grouped_records[index].clone());
        }

        // Calculate totals
        let total_topup: f32 = final_records
            .iter()
            .filter(|r| r.is_topup())
            .map(|r| r.transaction_value)
            .sum();
        let total_spent: f32 = final_records
            .iter()
            .filter(|r| !r.is_topup())
            .map(|r| r.transaction_value)
            .sum();
        println!("Summary for selected days:");
        println!("Total Top-up: R{:.2}", total_topup);
        println!("Total Spent: R{:.2}", total_spent);
        println!("Net Balance Change: R{:.2}", total_topup + total_spent);
    } else {
        eprintln!("Input file does not exist: {}", args.input.display());
    }
    Ok(())
}
