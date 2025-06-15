use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub listen: Listen,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Listen {
    Http {
        tcp_port: u16,
    },
    Https {
        tcp_port_http_redirect: Option<u16>,
        tcp_port: u16,
        udp_port: Option<u16>,
    },
}

fn parse_server_config(toml_str: &str) -> Result<Server, Box<dyn std::error::Error>> {
    let server: Server = toml::from_str(toml_str)?;
    Ok(server)
}

fn main() {
    println!("TOML Server Configuration Examples\n");
    
    // Example 1: HTTP configuration
    let http_toml = r#"
listen.http.tcp_port = 8000
"#;
    
    println!("Example 1 - HTTP Configuration:");
    println!("{}", http_toml.trim());
    
    match parse_server_config(http_toml) {
        Ok(server) => {
            println!("Parsed: {:#?}\n", server);
        }
        Err(e) => {
            println!("Error parsing HTTP config: {}\n", e);
        }
    }
    
    // Example 2: HTTPS configuration with all fields
    let https_full_toml = r#"
listen.https.tcp_port = 443
listen.https.tcp_port_http_redirect = 80
listen.https.udp_port = 443
"#;
    
    println!("Example 2 - HTTPS Configuration (full):");
    println!("{}", https_full_toml.trim());
    
    match parse_server_config(https_full_toml) {
        Ok(server) => {
            println!("Parsed: {:#?}\n", server);
        }
        Err(e) => {
            println!("Error parsing HTTPS full config: {}\n", e);
        }
    }
    
    // Example 3: HTTPS configuration with minimal fields
    let https_minimal_toml = r#"
listen.https.tcp_port = 443
"#;
    
    println!("Example 3 - HTTPS Configuration (minimal):");
    println!("{}", https_minimal_toml.trim());
    
    match parse_server_config(https_minimal_toml) {
        Ok(server) => {
            println!("Parsed: {:#?}\n", server);
        }
        Err(e) => {
            println!("Error parsing HTTPS minimal config: {}\n", e);
        }
    }
}
