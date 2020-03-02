use printpdf::*;
use std::fs::File;
use std::io::BufWriter;


fn main() {
	let (doc, page1, layer1) = PdfDocument::new("BANK CHECK", Mm(216.0), Mm(279.0), "Layer 1");
	let current_layer = doc.get_page(page1).get_layer(layer1);

	let bill_reference = "1";
	let bill_date = "01/06/2020";
	let payable_date = "01/06/2020";
	let payable_to = "Payable To";
	let payable_amount = "529.55";
	let payable_amount_written = "Five hundred twenty-nine and 55/100*************************************";
	let payable_address1 = "Address Line 1";
	let payable_address2 = "Address Line 2";
	let account = "0391";
	let check_number = "1900";

	let regular = doc.add_external_font(File::open("assets/fonts/Roboto-Regular.ttf").unwrap()).unwrap();
	let bold = doc.add_external_font(File::open("assets/fonts/Roboto-Bold.ttf").unwrap()).unwrap();

	// text, font size, x from left edge, y from top edge, font
	current_layer.use_text(payable_date, 10, Mm(180.0), Mm(259.0), &regular);
	current_layer.use_text(payable_to, 11, Mm(27.0), Mm(246.0), &regular);
	current_layer.use_text("**", 10, Mm(179.0), Mm(246.0), &regular);
	current_layer.use_text(payable_amount, 10, Mm(182.0), Mm(246.0), &regular);
	current_layer.use_text(payable_amount_written, 10, Mm(9.8), Mm(237.2), &regular);

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
		current_layer.use_text(account, 11, Mm(26.0), Mm(*y), &bold);
		current_layer.use_text("(", 11, Mm(36.0), Mm(*y), &bold);
		current_layer.use_text(account, 11, Mm(38.0), Mm(*y), &bold);
		current_layer.use_text(")", 11, Mm(47.0), Mm(*y), &bold);
		current_layer.use_text(payable_amount, 11, Mm(193.5), Mm(*y), &regular);
	}

	for y in vec![81.3, 174.2].iter() {
		current_layer.use_text("Date", 11, Mm(12.0), Mm(*y), &bold);
		current_layer.use_text("Type", 11, Mm(39.0), Mm(*y), &bold);
		current_layer.use_text("Reference", 11, Mm(62.0), Mm(*y), &bold);
		current_layer.use_text("Original Amount", 11, Mm(112.9), Mm(*y), &bold);
		current_layer.use_text("Balance Due", 11, Mm(146.0), Mm(*y), &bold);
		current_layer.use_text("Payment", 11, Mm(190.0), Mm(*y), &bold);
	}

	for y in vec![77.0, 170.2].iter() {
		current_layer.use_text(bill_date, 11, Mm(12.0), Mm(*y), &regular);
		current_layer.use_text("Bill", 11, Mm(39.3), Mm(*y), &regular);
		current_layer.use_text(bill_reference, 11, Mm(62.0), Mm(*y), &regular);
		current_layer.use_text(payable_amount, 11, Mm(126.5), Mm(*y), &regular);
		current_layer.use_text(payable_amount, 11, Mm(154.0), Mm(*y), &regular);
		current_layer.use_text(payable_amount, 11, Mm(193.5), Mm(*y), &regular);
	}

	for y in vec![73.0, 166.5].iter() {
		current_layer.use_text("Check Amount", 11, Mm(93.0), Mm(*y), &regular);
		current_layer.use_text(payable_amount, 11, Mm(193.5), Mm(*y), &regular);
	}

	doc.save(&mut BufWriter::new(File::create(format!("check{}.pdf", check_number)).unwrap())).unwrap();
}

