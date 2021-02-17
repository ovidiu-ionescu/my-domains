use askama::Template;

use crate::fetch_whois::DomainInfo;

#[derive(Template)]
#[template(path = "domains.html")]
struct DomainsTemplate<'a> {
    domains: &'a [DomainInfo],
}

pub fn render(domains: &[DomainInfo]) -> Result<String, askama::Error> {
    let domains_template = DomainsTemplate { domains };
    domains_template.render()
}

#[test]
fn test_render() {
    let domains = vec![
        DomainInfo {
            name: String::from("my domain"),
            expire: String::new(),
            registrar: String::new(),
            error: String::new(),
        },
        DomainInfo {
            name: String::from("my other domain"),
            expire: String::new(),
            registrar: String::new(),
            error: String::new(),
        },
    ];
    println!("{}", render(&domains).unwrap());
}
