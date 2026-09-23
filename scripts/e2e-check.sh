#!/bin/sh
# Envoy serves a new route a few seconds after the Gateway reports Programmed,
# so the first request is allowed to fail.
set -u

code=000
for _ in $(seq 1 30); do
  code=$(curl -s --max-time 5 -w "%{http_code}" -o /dev/null http://localhost)
  if [ "$code" = 200 ]; then
    echo "frontend answered 200"
    exit 0
  fi
  sleep 2
done

echo "frontend answered $code" >&2
kubectl get pods -A -o wide
kubectl get gateway,httproute -A
kubectl describe gateway demo
kubectl logs -n envoy-gateway-system deployments/envoy-gateway --tail=50
kubectl logs -n envoy-gateway-system -l gateway.envoyproxy.io/owning-gateway-name=demo --tail=50
for d in frontendservice quotationservice postgres-deployment redis-deployment; do
  echo "== $d"
  kubectl logs "deployments/$d" --all-containers --tail=50
done
exit 1
