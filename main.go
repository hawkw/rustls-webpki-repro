package main

import (
	"fmt"
	"os"
	"crypto/x509"
)

func main() {
	doIt("testdata/no-cn-test-ca1/crt.der", "no-cn.test.com")
	println("\n")
	doIt("testdata/cn-test-ca1/crt.der", "cn.test.com")
}

func doIt(path string, subjectName string) {
	fmt.Printf("=== Path: %s===\n\n", path)

	derFile, err := os.Open(path)
	defer derFile.Close()

	if err != nil {
		fmt.Print(err)
		return
	}

	derBytes := make([]byte, 1000)

	count, err := derFile.Read(derBytes)
	if err != nil {
		fmt.Print(err)
		return
	}

	println("--- go crypto/x509 ---\n")

	cert, err := x509.ParseCertificate(derBytes[0:count])
	if err != nil {
		fmt.Print(err)
		return
	}

	verifyErr := cert.VerifyHostname(subjectName)
	if verifyErr != nil {
		fmt.Printf("Not valid for %s: %s\n", subjectName, verifyErr)
	} else {
		fmt.Printf("Verified valid for %s\n", subjectName)
	}

	fmt.Printf("Subject: %s\n", cert.Subject);
	fmt.Printf("NotBefore: %s\n", cert.NotBefore);
	fmt.Printf("NotAfter: %s\n", cert.NotAfter);

	println("\nprinting DNS names...")
	for i, name := range cert.DNSNames {
		fmt.Printf("DNSNames[%d]: %s\n", i, name)
	}


}