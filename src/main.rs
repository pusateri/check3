use check3::Check3;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let c = Check3 {
        bill_references: vec!["Some Ref#".to_string()],
        bill_dates: vec!["03/16/2020".to_string()],
        bill_amounts: vec![47.10],
        payable_date: "03/19/2020".to_string(),
        payable_to: "Some Person".to_string(),
        payable_address1: "100 Main St".to_string(),
        payable_address2: "Anywhere, AK 11111".to_string(),
        acct_num_last4: "0000".to_string(),
        number: "1368".to_string(),
    };
    let doc = Check3::pdf_us_letter(&c);

    doc.save(&mut BufWriter::new(
        File::create(format!("check{}.pdf", c.number)).expect("create file"),
    ))
    .expect("save failed.");
}
