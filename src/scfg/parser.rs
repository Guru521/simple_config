use super::ss_string::SsString;
use std::collections::HashMap;

pub fn parse(scfg_str: &str) -> HashMap<String, String> {
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

pub fn parse_ss_string(scfg_str: &str) -> HashMap<String, SsString> {
    let datas = parse(scfg_str);
    let mut ss_datas: Vec<(String, SsString)> = Vec::new();

    datas
        .iter()
        .for_each(|data| ss_datas.push((data.0.clone(), SsString::from(data.1))));

    ss_datas.into_iter().collect()
}

#[test]
fn test_scfg() {
    assert_eq!(
        parse("  Target_file_name   :main.rs    "),
        HashMap::from([(String::from("Target_file_name"), String::from("main.rs"))])
    );
    assert_eq!(
        parse("Target_file_name: main.rs, \n Output_dir: ../build/"),
        HashMap::from([
            (String::from("Target_file_name"), String::from("main.rs")),
            (String::from("Output_dir"), String::from("../build/"))
        ])
    );
    assert_eq!(
        parse("  # Target_file_name: main.rs, \n Output_dir: # ../build/, User: sudo"),
        HashMap::from([(String::from("User"), String::from("sudo"))])
    );
    assert_eq!(
        parse_ss_string(" # comment, output:  test,  sources   :   main;  utils; math, ver: 2.0")
            .get("sources")
            .unwrap()
            .split(),
        vec!["main", "utils", "math"]
    )
}
