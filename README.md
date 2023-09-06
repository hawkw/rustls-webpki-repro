# rustls-webpki DNS name parsing repro

`rustls-webpki` returns an error when parsing DNS names (subject alternate
names) from a certificate, while Go's `crypto/x509` package parses the DNS name.

## expected output

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/rustls-webpki-repro`
--- rustls-webpki v0.101.4 ---

dns_names[0]: foo.test.com

--- rustls-webpki v0.102.0-alpha.1 ---

dns_names[0]: foo.test.com

$ go run repro
--- go crypto/x509 ---

DNSNames[0]: foo.test.com

```

## actual output

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/rustls-webpki-repro`
--- rustls-webpki v0.101.4 ---

Error: BadDer

--- rustls-webpki v0.102.0-alpha.1 ---

Error: TrailingData(CommonNameOuter)

$ go run repro
--- go crypto/x509 ---

DNSNames[0]: foo.test.com

```