package main

import (
	"fmt"
	"log"
	"os"
	"crypto/x509"
)

func main() {
	derFile, err := os.Open("testdata/foo-test-ca1/crt.der")
	defer derFile.Close()

	if err != nil {
		log.Fatal(err)
	}

	derBytes := make([]byte, 1000)

	count, err := derFile.Read(derBytes)
	if err != nil {
		log.Fatal(err)
	}

	println("--- go crypto/x509 ---\n")

	cert, err := x509.ParseCertificate(derBytes[0:count])
	if err != nil {
		log.Fatal(err)
	}

	for i, name := range cert.DNSNames {
		fmt.Printf("DNSNames[%d]: %s\n", i, name)
	}

}
