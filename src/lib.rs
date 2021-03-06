// Copyright 2020 Tom Pusateri
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

//! 3-part business check PDF creation library

use english_numbers::convert;
use english_numbers::Formatting;
use printpdf::*;
use std::fs::File;

#[derive(Debug)]
pub struct Check3 {
    pub bill_references: Vec<String>,
    pub bill_dates: Vec<String>,
    pub bill_amounts: Vec<f32>,
    pub payable_date: String,
    pub payable_to: String,
    pub payable_address1: String,
    pub payable_address2: Option<String>,
    pub payable_address3: String,
    pub acct_num_last4: String,
    pub number: String,
}

impl Check3 {
    /// Create a US Letter PDF Document from components
    pub fn pdf_us_letter(check: &Check3) -> PdfDocumentReference {
        // number formatting
        let payable_amount_float: f32 = check.bill_amounts.iter().sum();
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
        let (doc, page1, layer1) = PdfDocument::new(
            format!("Check {}", check.number),
            Mm(216.0),
            Mm(279.0),
            "Layer 1".to_string(),
        );
        let layer = doc.get_page(page1).get_layer(layer1);
        let regular = doc
            .add_external_font(File::open("assets/fonts/Roboto-Regular.ttf").unwrap())
            .unwrap();
        let bold = doc
            .add_external_font(File::open("assets/fonts/Roboto-Bold.ttf").unwrap())
            .unwrap();

        // text, font size, x from left edge, y from top edge, font
        layer.use_text(&check.payable_date, 10, Mm(180.0), Mm(259.0), &regular);
        layer.use_text(&check.payable_to, 11, Mm(27.0), Mm(246.0), &regular);
        layer.use_text(padded_amount, 10, Mm(179.0), Mm(246.0), &regular);
        layer.use_text(padded_english, 10, Mm(9.8), Mm(237.2), &regular);

        // mailingaddress
        layer.use_text(&check.payable_to, 11, Mm(22.0), Mm(229.0), &regular);
        layer.use_text(&check.payable_address1, 11, Mm(22.0), Mm(225.0), &regular);
        if let Some(addr2) = &check.payable_address2 {
            layer.use_text(addr2, 11, Mm(22.0), Mm(221.0), &regular);
            layer.use_text(&check.payable_address3, 11, Mm(22.0), Mm(217.0), &regular);
        } else {
            layer.use_text(&check.payable_address3, 11, Mm(22.0), Mm(221.0), &regular);
        }

        for y in vec![87.2, 180.7].iter() {
            layer.use_text(&check.payable_date, 10, Mm(18.0), Mm(*y), &bold);
            layer.use_text(&check.payable_to, 11, Mm(50.0), Mm(*y), &bold);
        }

        for y in vec![14.0, 105.0].iter() {
            layer.use_text("Checking", 11, Mm(9.1), Mm(*y), &bold);
            layer.use_text(&check.acct_num_last4, 11, Mm(27.0), Mm(*y), &bold);
            layer.use_text("(", 11, Mm(37.0), Mm(*y), &bold);
            layer.use_text(&check.acct_num_last4, 11, Mm(39.0), Mm(*y), &bold);
            layer.use_text(")", 11, Mm(48.0), Mm(*y), &bold);
            layer.use_text(&right_adjusted_amount, 11, Mm(191.5), Mm(*y), &regular);
        }

        for y in vec![81.3, 174.2].iter() {
            layer.use_text("Date", 11, Mm(12.0), Mm(*y), &bold);
            layer.use_text("Type", 11, Mm(39.0), Mm(*y), &bold);
            layer.use_text("Reference", 11, Mm(62.0), Mm(*y), &bold);
            layer.use_text("Original Amount", 11, Mm(112.9), Mm(*y), &bold);
            layer.use_text("Balance Due", 11, Mm(146.0), Mm(*y), &bold);
            layer.use_text("Payment", 11, Mm(190.0), Mm(*y), &bold);
        }

        let y_starts = vec![77.0, 170.2];
        for y in y_starts.iter() {
            for (i, bill_date) in check.bill_dates.iter().enumerate() {
                let offset = i as f64 * 4.0;
                layer.use_text(bill_date, 11, Mm(12.0), Mm(*y - offset), &regular);
                layer.use_text("Bill", 11, Mm(39.3), Mm(*y - offset), &regular);
                layer.use_text(
                    &check.bill_references[i],
                    11,
                    Mm(62.0),
                    Mm(*y - offset),
                    &regular,
                );
                let amount = format!("{:>8.2}", check.bill_amounts[i]);
                layer.use_text(&amount, 11, Mm(124.5), Mm(*y - offset), &regular);
                layer.use_text(&amount, 11, Mm(152.0), Mm(*y - offset), &regular);
                layer.use_text(&amount, 11, Mm(191.5), Mm(*y - offset), &regular);
            }
        }

        let y_starts2 = y_starts
            .iter()
            .map(|x| x - check.bill_dates.len() as f64 * 4.0 - 1.0);
        for (_i, y) in y_starts2.enumerate() {
            layer.use_text("Check Amount", 11, Mm(93.0), Mm(y), &regular);
            layer.use_text(&right_adjusted_amount, 11, Mm(191.5), Mm(y), &regular);
        }
        return doc;
    }
}
