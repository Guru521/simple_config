use std::collections::HashMap;

pub fn get_scfgs(scfg_str: &str) -> HashMap<String, String> {
    let datas: Vec<&str> = scfg_str.split(',').collect();
    let datas: Vec<&str> = datas.iter().map(|string| string.trim()).collect();

    let mut option_names_values: HashMap<String, String> = HashMap::new();
    datas
        .iter()
        .map(|string| string.split(':').collect::<Vec<&str>>())
        .filter(|string| {
            !string[0].trim().to_owned().starts_with('#')
                && !string[1].trim().to_owned().starts_with('#')
        })
        .for_each(|string| {
            option_names_values.insert(string[0].trim().to_owned(), string[1].trim().to_owned());
        });

    option_names_values
}

#[test]
fn test_scfg() {
    assert_eq!(
        get_scfgs("  Target_file_name   :main.rs    "),
        HashMap::from([(String::from("Target_file_name"), String::from("main.rs"))])
    );
    assert_eq!(
        get_scfgs("Target_file_name: main.rs, \n Output_dir: ../build/"),
        HashMap::from([
            (String::from("Target_file_name"), String::from("main.rs")),
            (String::from("Output_dir"), String::from("../build/"))
        ])
    );
    assert_eq!(
        get_scfgs("  # Target_file_name: main.rs, \n Output_dir: # ../build/, User: sudo"),
        HashMap::from([(String::from("User"), String::from("sudo"))])
    );
}
