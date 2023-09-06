#!/usr/bin/env bash
#
# Requires:
# go install github.com/cloudflare/cfssl/cmd/cfssl@latest
# go install github.com/cloudflare/cfssl/cmd/cfssljson@latest
#
set -euox pipefail


basedir=$( cd "${0%/*}" && pwd )

ca() {
  name=$1
  filename=$2

  echo "{\"names\":[{\"CN\": \"${name}\",\"OU\":\"None\"}], \"ca\": {\"expiry\": \"87600h\"}}" \
    | cfssl genkey -initca - \
    | cfssljson -bare "${filename}"

  rm "${filename}.csr"
}

ee() {
  ca_name=$1
  ee_name=$2
  ee_ns=$3

  hostname="${ee_name}.${ee_ns}.com"

  ee="${ee_name}-${ee_ns}-${ca_name}"
  echo '{}' \
    | cfssl gencert -ca "${ca_name}.pem" \
      -ca-key "${ca_name}-key.pem" \
      -hostname "${hostname}" -config="../ca-config.json" - \
    | cfssljson -bare "${ee}"
  mkdir -p "${ee}"

  openssl pkcs8 -topk8 -nocrypt -inform pem -outform der \
    -in "${ee}-key.pem" \
    -out "${ee}/key.p8"
  rm "${ee}-key.pem"

  openssl x509 -inform pem -outform der \
    -in "${ee}.pem" \
    -out "${ee}/crt.der"
  rm "${ee}.pem"

  ## TODO DER-encode?
  #openssl x509 -inform pem -outform der \
  #  -in "${ee}.csr" \
  #  -out "${ee}/csr.der"
  mv "${ee}.csr" "${ee}/csr.pem"
}


cd "${basedir}/testdata"
ca 'Cluster-local CA 1' ca1
ee ca1 foo test