use my_domains::fetch_whois::DomainInfo;

#[test]
fn test_fetch_domains() {
    let domains: Vec<String> = vec!["ionescu.net", "happyhacker.io"]
        .iter()
        .map(|s| String::from(*s))
        .collect();
    //println!("{:#?}", &DomainInfo::fetch_info(&domains))
}
