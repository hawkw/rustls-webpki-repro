const CERT: &[u8] = include_bytes!("../testdata/foo-test-ca1/crt.der");

fn main() {
    println!("--- rustls-webpki v0.101.4 ---\n");
    if let Err(error) = rustls_webpki_101() {
        println!("Error: {error}");
    }

    println!("\n--- rustls-webpki v0.102.0-alpha.1 ---\n");
    if let Err(error) = rustls_webpki_102() {
        println!("Error: {error}");
    }
}

fn rustls_webpki_101() -> Result<(), Box<dyn std::error::Error>> {
    let cert = rustls_webpki_101::EndEntityCert::try_from(CERT)?;

    for (i, name) in cert.dns_names()?.enumerate() {
        let name: &str = name.into();
        println!("dns_names[{i}]: {name}");
    }

    Ok(())
}

fn rustls_webpki_102() -> Result<(), Box<dyn std::error::Error>> {
    let der = rustls_webpki_102::types::CertificateDer::from(CERT);
    let cert = rustls_webpki_102::EndEntityCert::try_from(&der)?;

    for (i, name) in cert.dns_names()?.enumerate() {
        let name: &str = name.into();
        println!("dns_names[{i}]: {name}");
    }

    Ok(())
}
