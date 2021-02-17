use futures::{prelude::*, stream::futures_unordered::FuturesUnordered};
use itertools::Itertools;
use whois_rust::{WhoIs, WhoIsLookupOptions};

#[derive(Debug, Clone)]
pub struct DomainInfo {
    pub name: String,
    pub expire: String,
    pub registrar: String,
    pub error: String,
}

impl DomainInfo {
    pub async fn fetch_info(config_domains: &[String]) -> Vec<DomainInfo> {
        let whois = WhoIs::from_path("./servers.json").expect("Failed to open ./servers.json to read whois servers");

        let mut domains: Vec<&String> = config_domains.iter().collect();
        domains.sort_by(|a, b| ext(&a).cmp(ext(&b)));

        // group the domains by extension
        let exts: Vec<Vec<&String>> = domains
            .iter()
            .group_by(|a| ext(a))
            .into_iter()
            .map(|(_, g)| g.collect::<Vec<&&String>>().iter().map(|&s| *s).collect())
            .collect();

        // resolve each extension group asynchronously
        let resolved_exts = exts
            .iter()
            .map(|d| resolve_extension(&whois, d))
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await;

        // flatten the results
        let mut res: Vec<DomainInfo> = resolved_exts.into_iter().flatten().collect();
        // sort increasing by expiration date, format is ISO so we can use lexicographical ordering
        res.sort_by(|a, b| a.expire.cmp(&b.expire));
        res
    }
}

async fn resolve_extension(whois: &WhoIs, domains: &[&String]) -> Vec<DomainInfo> {
    let mut result: Vec<DomainInfo> = Vec::with_capacity(domains.len());
    for domain in domains {
        result.push(resolve_domain(whois, *domain).await);
    }
    result
}

async fn resolve_domain(whois: &WhoIs, domain: &str) -> DomainInfo {
    let name = String::from(domain);
    let whois_result = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).await;
    let result = match whois_result {
        Ok(s) => s,
        Err(err) => {
            println!("Error fetching whois {:#?}", err);
            return DomainInfo {
                name,
                expire: String::new(),
                registrar: String::new(),
                error: String::from("whois call failed"),
            };
        }
    };

    let mut expire = String::new();
    let mut registrar = String::new();

    for tline in result.lines() {
        let line = tline.trim();
        //dbg!("{}", line);
        if let Some(s) = line.strip_prefix("Registry Expiry Date:") {
            expire = String::from(s);
        }
        if let Some(s) = line.strip_prefix("Expires On:") {
            expire = String::from(s);
        }
        if let Some(s) = line.strip_prefix("Registrar: ") {
            registrar = String::from(s);
        }
        if let Some(s) = line.strip_prefix("Name: ") {
            registrar = String::from(s);
        }
    }
    // if registrar.is_empty() {
    //     println!("{}", result);
    // }

    DomainInfo {
        name,
        expire,
        registrar,
        error: String::new(),
    }
}

fn ext(s: &str) -> &str {
    &s[1 + s.rfind('.').unwrap()..]
}
#[test]
fn test_sort_extensions() {
    assert_eq!("net", ext("aha.net"));
    let mut v = vec!["aha.net", "aha.com", "aha.org"];
    v.sort_by(|a, b| ext(&a).cmp(ext(&b)));
    assert_eq!(vec!("aha.com", "aha.net", "aha.org"), v);
}
