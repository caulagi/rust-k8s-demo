PHONY: update-proto
update-proto: # Update protobuf definitions for all microservices
	cp proto/quotation.proto quotationservice/proto
	cp proto/quotation.proto frontendservice/proto

PHONY: bootstrap
bootstrap:
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/static/provider/kind/deploy.yaml
	kubectl rollout status --timeout 2m -w deployments/ingress-nginx-controller -n ingress-nginx
	kubectl create secret generic postgres-password --from-literal=pgpassword=panda

.PHONY: e2e
e2e: $(SERVICE_IP)
	skaffold run
	kubectl rollout status --timeout 10m -w deployments/postgres-deployment
	kubectl rollout status --timeout 10m -w deployments/quotationservice
	kubectl rollout status --timeout 10m -w deployments/frontendservice
	kubectl rollout status --timeout 10m -w deployments/ingress-nginx-controller -n ingress-nginx
	kubectl logs -l app=frontendservice
	kubectl logs -l app=quotationservice
	test 200 = $$(curl -sL -w "%{http_code}\\n" http://localhost -o /dev/null)

.PHONY: help
help: # Show this help
	@{ \
	echo 'Targets:'; \
	echo ''; \
	grep '^[a-z/.-]*: .*# .*' Makefile \
	| sort \
	| sed 's/: \(.*\) # \(.*\)/ - \2 (deps: \1)/' `: fmt targets w/ deps` \
	| sed 's/:.*#/ -/'                            `: fmt targets w/o deps` \
	| sed 's/^/    /'                             `: indent`; \
	echo ''; \
	} 1>&2; \
