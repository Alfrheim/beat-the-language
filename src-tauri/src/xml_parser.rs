extern crate quick_xml;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;

// This function parses the XML file and extracts its content
pub(crate) fn parse_xml_file() -> HashMap<String, (String, String)> {
      // Open the XML file
    let file = File::open("src/resources/en-es.xml").unwrap();
    let file = BufReader::new(file);

    // Create a new XML reader
    let mut reader = Reader::from_reader(file);
    reader.trim_text(true);
    // Create a HashMap to store the extracted content
    let mut extracted_content = HashMap::new();

    // Parse the XML file and extract its content
    let mut buf = Vec::new();
    let mut in_l_element = false;
    let mut in_c_element = false;
    let mut in_d_element = false;
    let mut in_t_element = false;
    let mut current_c_value = String::new();
    let mut current_d_value = String::new();
    let mut current_t_value = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // Handle the Start event
                if e.name() == quick_xml::name::QName(b"l") {
                    in_l_element = true;
                }

                // Check if we are in the <c>, <d>, or <t> element
                if in_l_element {
                    if e.name() == quick_xml::name::QName(b"c") {
                        in_c_element = true;
                    } else if e.name() == quick_xml::name::QName(b"d") {
                        in_d_element = true;
                    } else if e.name() == quick_xml::name::QName(b"t") {
                        in_t_element = true;
                    }
                }
            },
            Ok(Event::Text(e)) => {
                // Handle the Text event
                if in_c_element {
                    current_c_value = e.unescape().unwrap().to_string();
                } else if in_d_element {
                    current_d_value = e.unescape().unwrap().to_string();
                } else if in_t_element {
                    current_t_value = e.unescape().unwrap().to_string();
                }
            },
            Ok(Event::End(ref e)) => {
                // Handle the End event
                if e.name() == quick_xml::name::QName(b"c") {
                    in_c_element = false;
                } else if e.name() == quick_xml::name::QName(b"d") {
                    in_d_element = false;
                } else if e.name() == quick_xml::name::QName(b"t") {
                    in_t_element = false;
                }

                // Check if we have reached the end of the <w> element
                if e.name() ==  quick_xml::name::QName(b"w"){
                    // If we have reached the end of the <w> element, store the values of the <c>, <d>, and <t> elements in the HashMap
                    extracted_content.insert(current_c_value.clone(), (current_d_value.clone(), current_t_value.clone()));
                    current_c_value.clear();
                    current_d_value.clear();
                    current_t_value.clear();
                }

                // Check if we have reached the end of the <l> element
                if e.name() == quick_xml::name::QName(b"l") {
                    // If we have reached the end of the <l> element, stop parsing the XML file
                    in_l_element = false;
                    break;
                }
            },
            _ => (),
        }
    }

    // Return the extracted content
    extracted_content
}