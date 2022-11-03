use rust_sbml::ModelRaw;

fn main() {
    let file_str = match std::fs::read_to_string("examples/00_Lipid_Carma_KEGG_Model.xml") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Nothing to do! Is '00_Lipid_Carma_KEGG_Model.xml' in your 'examples' directory? Error: {e}");
            return;
        }
    };
    let model = ModelRaw::parse(&file_str).unwrap();
    model
        .list_of_reactions
        .reactions
        .iter()
        // flatten annotation and take the identifiers of the two elements, if any
        .filter_map(|r| match r.annotation.as_ref() {
            Some(s) => match &s.rdf {
                Some(_) => {
                    if let [kegg, eccode] = s.flatten().unwrap()[..] {
                        Some((kegg.split('/').last(), eccode.split('/').last()))
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        })
        // remove empty strings and None options
        .filter(|(a, b)| {
            !a.map(|v| v.is_empty()).unwrap_or(false) & !b.map(|v| v.is_empty()).unwrap_or(false)
        })
        .for_each(|(kegg, eccode)| println!("{},{}", kegg.unwrap(), eccode.unwrap()))
}
