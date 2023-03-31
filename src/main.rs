use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sniffer")]
#[command(author = "Daniel Beach")]
#[command(version = "1.0")]
#[command(about = "sniffs flat files", long_about = None)]
struct Args {
    #[arg(long)]
    file_path: String,

    #[arg(long)]
    delimiter: String,

    #[arg(long, default_value_t = 0)]
    quote: u32,

    #[arg(long, default_value_t = 0)]
    check_nulls: u32,
}

fn main() {
    let args = Args::parse();

    let file_path = args.file_path;
    let delimiter = args.delimiter;
    let quote = args.quote;
    let check_nulls = args.check_nulls;

    let lines = sniffer::read_number_lines_in_file(&file_path);
    let size_in_mb = sniffer::get_file_size_in_mb(&file_path);
    println!("File size in MB: {}", size_in_mb);
    println!("number of lines: {}", lines);
    sniffer::print_headers(&file_path, &delimiter, &quote);
    sniffer::print_a_few_lines(&file_path, &delimiter, &quote, 3);
    if check_nulls == 1 {
        sniffer::check_all_column_for_nulls(&file_path, &delimiter, &quote);
    }
}
