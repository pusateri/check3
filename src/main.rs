use english_numbers::convert;
use english_numbers::Formatting;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // user input
    let bill_references = ["Some Ref#"];
    let bill_dates = ["03/16/2020"];
    let bill_amounts = [47.10];
    let payable_date = "03/19/2020";
    let payable_to = "Some Person";
    let payable_address1 = "100 Main St";
    let payable_address2 = "Anywhere, AK 11111";
    let account_last4 = "0000";
    let check_number = "1368";

    // number formatting
    let payable_amount_float: f32 = bill_amounts.iter().sum();
    let payable_amount = format!("{:.2}", payable_amount_float);
    let mut fmt = Formatting::all();
    fmt.conjunctions = false;

    let payable_amount_int = payable_amount_float.trunc() as i64;
    let payable_amount_english_int = convert(payable_amount_int, fmt);
    let payable_amount_decimal = (payable_amount_float.fract() * 100.0).round();
    let payable_amount_english_decimal = format!(
        "{} and {:02}/100 ",
        payable_amount_english_int, payable_amount_decimal
    );
    let padded_english = format!("{:*<100}", payable_amount_english_decimal);
    let padded_amount = format!("{:*>8}", payable_amount);
    let right_adjusted_amount = format!("{:>8}", payable_amount);

    // PDF initialization
    let (doc, page1, layer1) = PdfDocument::new("BANK CHECK", Mm(216.0), Mm(279.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let regular = doc
        .add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    // text, font size, x from left edge, y from top edge, font
    current_layer.use_text(payable_date, 10, Mm(180.0), Mm(259.0), &regular);
    current_layer.use_text(payable_to, 11, Mm(27.0), Mm(246.0), &regular);
    current_layer.use_text(padded_amount, 10, Mm(179.0), Mm(246.0), &regular);
    current_layer.use_text(padded_english, 10, Mm(9.8), Mm(237.2), &regular);

    // mailingaddress
    current_layer.use_text(payable_to, 11, Mm(22.0), Mm(229.0), &regular);
    current_layer.use_text(payable_address1, 11, Mm(22.0), Mm(225.0), &regular);
    current_layer.use_text(payable_address2, 11, Mm(22.0), Mm(221.0), &regular);

    for y in vec![87.2, 180.7].iter() {
        current_layer.use_text(payable_date, 10, Mm(18.0), Mm(*y), &bold);
        current_layer.use_text(payable_to, 11, Mm(50.0), Mm(*y), &bold);
    }

    for y in vec![14.0, 105.0].iter() {
        current_layer.use_text("Checking", 11, Mm(9.1), Mm(*y), &bold);
        current_layer.use_text(account_last4, 11, Mm(27.0), Mm(*y), &bold);
        current_layer.use_text("(", 11, Mm(37.0), Mm(*y), &bold);
        current_layer.use_text(account_last4, 11, Mm(39.0), Mm(*y), &bold);
        current_layer.use_text(")", 11, Mm(48.0), Mm(*y), &bold);
        current_layer.use_text(&right_adjusted_amount, 11, Mm(191.5), Mm(*y), &regular);
    }

    for y in vec![81.3, 174.2].iter() {
        current_layer.use_text("Date", 11, Mm(12.0), Mm(*y), &bold);
        current_layer.use_text("Type", 11, Mm(39.0), Mm(*y), &bold);
        current_layer.use_text("Reference", 11, Mm(62.0), Mm(*y), &bold);
        current_layer.use_text("Original Amount", 11, Mm(112.9), Mm(*y), &bold);
        current_layer.use_text("Balance Due", 11, Mm(146.0), Mm(*y), &bold);
        current_layer.use_text("Payment", 11, Mm(190.0), Mm(*y), &bold);
    }

    let y_starts = vec![77.0, 170.2];
    for y in y_starts.iter() {
        for (i, bill_date) in bill_dates.iter().enumerate() {
            let offset = i as f64 * 4.0;
            current_layer.use_text(*bill_date, 11, Mm(12.0), Mm(*y - offset), &regular);
            current_layer.use_text("Bill", 11, Mm(39.3), Mm(*y - offset), &regular);
            current_layer.use_text(bill_references[i], 11, Mm(62.0), Mm(*y - offset), &regular);
            let amount = format!("{:>8.2}", bill_amounts[i]);
            current_layer.use_text(&amount, 11, Mm(124.5), Mm(*y - offset), &regular);
            current_layer.use_text(&amount, 11, Mm(152.0), Mm(*y - offset), &regular);
            current_layer.use_text(&amount, 11, Mm(191.5), Mm(*y - offset), &regular);
        }
    }

    let y_starts2 = y_starts
        .iter()
        .map(|x| x - bill_dates.len() as f64 * 4.0 - 1.0);
    for (_i, y) in y_starts2.enumerate() {
        //let offset = i as f64 * 4.0;
        current_layer.use_text("Check Amount", 11, Mm(93.0), Mm(y), &regular);
        current_layer.use_text(&right_adjusted_amount, 11, Mm(191.5), Mm(y), &regular);
    }

    doc.save(&mut BufWriter::new(
        File::create(format!("check{}.pdf", check_number)).unwrap(),
    ))
    .unwrap();
}
