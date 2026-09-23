#!/bin/sh
# The ingress starts serving a route a few seconds after its rollout reports
# ready, so the first request is allowed to fail.
set -u

code=000
for _ in $(seq 1 30); do
  code=$(curl -s -w "%{http_code}" -o /dev/null http://localhost)
  if [ "$code" = 200 ]; then
    echo "frontend answered 200"
    exit 0
  fi
  sleep 2
done

echo "frontend answered $code" >&2
kubectl get pods -A -o wide
kubectl get ingress -A
kubectl describe ingress frontendservice-ingress
kubectl logs -n ingress-nginx deployments/ingress-nginx-controller --tail=50
for d in frontendservice quotationservice postgres-deployment redis-deployment; do
  echo "== $d"
  kubectl logs "deployments/$d" --all-containers --tail=50
done
exit 1
