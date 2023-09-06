fn main() {
    do_it("testdata/no-cn-test-ca1/crt.der", "no-cn.test.com");
    println!("");
    do_it("testdata/cn-test-ca1/crt.der", "cn.test.com");
}

fn do_it(path: &str, subject_name: &str) {
    println!("=== Path: {path} ===\n");

    let cert = match std::fs::read(path) {
        Ok(cert) => cert,
        Err(error) => {
            println!("failed to read DER file '{path}': {error}");
            return;
        }
    };

    println!("--- rustls-webpki v0.101.4 ---\n");
    if let Err(error) = rustls_webpki_101(&cert, subject_name) {
        println!("Error: {error}");
    }

    println!("\n--- rustls-webpki v0.102.0-alpha.1 ---\n");
    if let Err(error) = rustls_webpki_102(&cert, subject_name) {
        println!("Error: {error}");
    }
}

fn rustls_webpki_101(
    cert: impl AsRef<[u8]>,
    subject_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cert = rustls_webpki_101::EndEntityCert::try_from(cert.as_ref())?;
    let sn = rustls_webpki_101::SubjectNameRef::try_from_ascii_str(subject_name)
        .expect("not valid subject name");
    match cert.verify_is_valid_for_subject_name(sn) {
        Ok(_) => println!("Verified valid for {subject_name}"),
        Err(error) => println!("Not valid for {subject_name}: {error}"),
    }

    println!("printing DNS names...");
    for (i, name) in cert.dns_names()?.enumerate() {
        let name: &str = name.into();
        println!("dns_names[{i}]: {name}");
    }

    Ok(())
}

fn rustls_webpki_102(
    cert: impl AsRef<[u8]>,
    subject_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let der = rustls_webpki_102::types::CertificateDer::from(cert.as_ref());
    let cert = rustls_webpki_102::EndEntityCert::try_from(&der)?;
    let sn = rustls_webpki_102::SubjectNameRef::try_from_ascii_str(subject_name)
        .expect("not valid subject name");
    match cert.verify_is_valid_for_subject_name(sn) {
        Ok(_) => println!("Verified valid for {subject_name}"),
        Err(error) => println!("Not valid for {subject_name}: {error}"),
    }

    println!("printing DNS names...");
    for (i, name) in cert.dns_names()?.enumerate() {
        let name: &str = name.into();
        println!("dns_names[{i}]: {name}");
    }

    Ok(())
}
