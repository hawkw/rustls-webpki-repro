# rustls-webpki DNS name parsing repro

`rustls-webpki` returns an error when parsing DNS names (subject alternate
names) from a DER-encoded certificate, while Go's `crypto/x509` package parses
the DNS name. Both `rustls-webpki` and `crypto/x509` verify the certificates as
valid. This occurs with both `rustls-webpki` v0.101.4 and with v0.102.0-alpha.1,
although v0.102 emits a different error when there is no subject name.

Certificates are generated using [`cloudflare/cfssl`] and converted from PEM to
DER using `openssl`. See [`gen-certs.sh`] for the script used to generate
certificates.

[`cloudflare/cfssl`]: https://github.com/cloudflare/cfssl
[`gen-certs.sh`]: ./gen-certs.sh

## expected output

```console
:; cargo run
   Compiling rustls-webpki-repro v0.1.0 (/home/eliza/Code/rustls-webpki-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/rustls-webpki-repro`
=== Path: testdata/no-cn-test-ca1/crt.der ===

--- rustls-webpki v0.101.4 ---

Verified valid for no-cn.test.com
printing DNS names...
dns_names[0]: no-cn.test.com

--- rustls-webpki v0.102.0-alpha.1 ---

Verified valid for no-cn.test.com
printing DNS names...
dns_names[0]: no-cn.test.com

=== Path: testdata/cn-test-ca1/crt.der ===

--- rustls-webpki v0.101.4 ---

Verified valid for cn.test.com
printing DNS names...
dns_names[0]: cn.test.com

--- rustls-webpki v0.102.0-alpha.1 ---

Verified valid for cn.test.com
printing DNS names...
dns_names[0]: cn.test.com

$ go run repro
=== Path: testdata/no-cn-test-ca1/crt.der===

--- go crypto/x509 ---

Verified valid for no-cn.test.com
Subject:
NotBefore: 2023-09-06 17:21:00 +0000 UTC
NotAfter: 2033-09-03 17:21:00 +0000 UTC

printing DNS names...
DNSNames[0]: no-cn.test.com


=== Path: testdata/cn-test-ca1/crt.der===

--- go crypto/x509 ---

Verified valid for cn.test.com
Subject: CN=cn.test.com
NotBefore: 2023-09-06 17:21:00 +0000 UTC
NotAfter: 2033-09-03 17:21:00 +0000 UTC

printing DNS names...
DNSNames[0]: cn.test.com

```

## actual output

```console
$ cargo run
   Compiling rustls-webpki-repro v0.1.0 (/home/eliza/Code/rustls-webpki-repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/rustls-webpki-repro`
=== Path: testdata/no-cn-test-ca1/crt.der ===

--- rustls-webpki v0.101.4 ---

Verified valid for no-cn.test.com
printing DNS names...
Error: BadDer

--- rustls-webpki v0.102.0-alpha.1 ---

Verified valid for no-cn.test.com
printing DNS names...
Error: TrailingData(CommonNameOuter)

=== Path: testdata/cn-test-ca1/crt.der ===

--- rustls-webpki v0.101.4 ---

Verified valid for cn.test.com
printing DNS names...
Error: BadDer

--- rustls-webpki v0.102.0-alpha.1 ---

Verified valid for cn.test.com
printing DNS names...
Error: BadDer


$ go run repro
=== Path: testdata/no-cn-test-ca1/crt.der===

--- go crypto/x509 ---

Verified valid for no-cn.test.com
Subject:
NotBefore: 2023-09-06 17:21:00 +0000 UTC
NotAfter: 2033-09-03 17:21:00 +0000 UTC

printing DNS names...
DNSNames[0]: no-cn.test.com


=== Path: testdata/cn-test-ca1/crt.der===

--- go crypto/x509 ---

Verified valid for cn.test.com
Subject: CN=cn.test.com
NotBefore: 2023-09-06 17:21:00 +0000 UTC
NotAfter: 2033-09-03 17:21:00 +0000 UTC

printing DNS names...
DNSNames[0]: cn.test.com

```