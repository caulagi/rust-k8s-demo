FROM postgres:18-trixie

COPY data /docker-entrypoint-initdb.d
COPY pg_hba.conf /etc/postgresql/pg_hba.conf

# Satisfies the entrypoint's initdb check; the pg_hba.conf above decides access.
ENV POSTGRES_HOST_AUTH_METHOD=trust

# The certificates are mounted from the postgres-server-tls secret.
CMD ["postgres", \
     "-c", "hba_file=/etc/postgresql/pg_hba.conf", \
     "-c", "ssl=on", \
     "-c", "ssl_cert_file=/etc/postgresql/tls/server/tls.crt", \
     "-c", "ssl_key_file=/etc/postgresql/tls/server/tls.key", \
     "-c", "ssl_ca_file=/etc/postgresql/tls/server/ca.crt"]
