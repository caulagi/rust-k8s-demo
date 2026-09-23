#!/bin/sh
# Mints the CA, server and client certificates cert-manager issues in a
# cluster, for running Postgres and the quotation service outside one.
# Writes them under ./certs in the layout the Kubernetes secrets would have.
set -eu

out=${1:-certs}
mkdir -p "$out/server" "$out/client"
cd "$out"

openssl ecparam -name prime256v1 -genkey -noout -out ca.key
openssl req -x509 -new -key ca.key -days 3650 -subj "/CN=rust-k8s-demo postgres CA" -out ca.crt

openssl ecparam -name prime256v1 -genkey -noout -out server/tls.key
openssl req -new -key server/tls.key -subj "/CN=postgres-service" \
  -addext "subjectAltName=DNS:postgres-service,DNS:localhost,DNS:host.containers.internal" \
  -out server/tls.csr
openssl x509 -req -in server/tls.csr -CA ca.crt -CAkey ca.key -CAcreateserial -days 365 \
  -copy_extensions copy -out server/tls.crt
cp ca.crt server/ca.crt

openssl ecparam -name prime256v1 -genkey -noout -out client/tls.key
openssl req -new -key client/tls.key -subj "/CN=postgres" -out client/tls.csr
openssl x509 -req -in client/tls.csr -CA ca.crt -CAkey ca.key -CAcreateserial -days 365 \
  -out client/tls.crt
cp ca.crt client/ca.crt

rm -f server/tls.csr client/tls.csr ca.srl
# Postgres accepts a key it does not own only at these permissions.
chmod 0640 server/tls.key client/tls.key
echo "certificates written to $(pwd)"
