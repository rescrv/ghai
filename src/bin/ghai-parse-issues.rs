use std::io::Read;

use ghai::Issue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = None;
    let mut issues = vec![];
    for arg in std::env::args().skip(1) {
        if arg == "-" && stdin.is_none() {
            let mut buf = vec![];
            std::io::stdin().read_to_end(&mut buf)?;
            stdin = Some(String::from_utf8(buf)?);
        }
        let data = if arg == "-" {
            // SAFETY(rescrv):  The immediate block above checks when this is None.
            stdin.as_ref().unwrap().clone()
        } else {
            std::fs::read_to_string(arg)?
        };
        let local: Vec<serde_json::Value> = serde_json::from_str(&data)?;
        for issue in local {
            let issue: Issue = match serde_json::from_str(&issue.to_string()) {
                Ok(issue) => issue,
                Err(e) => {
                    eprintln!("Error parsing {}: {:?}", issue, e);
                    continue;
                }
            };
            issues.push(issue);
        }
    }
    Ok(())
}
