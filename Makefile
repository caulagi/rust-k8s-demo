SERVICE_IP = $(kubectl get svc --selector=app=frontend,component=loadbalancer -o json | jq --raw-output ".items[0].status.loadBalancer.ingress[0].ip")

PHONY: update-proto
update-proto: # Update protobuf definitions for all microservices
	cp proto/quotation.proto quotationservice/proto
	cp proto/quotation.proto frontendservice/proto

.PHONY: e2e
e2e: $(SERVICE_IP)
	kubectl apply -f https://raw.githubusercontent.com/google/metallb/v0.8.3/manifests/metallb.yaml
	kubectl create secret generic postgres-password --from-literal=pgpassword=panda
	skaffold run
	kubectl rollout status --timeout 2m -w deployments/postgres-deployment
	kubectl rollout status --timeout 2m -w deployments/quotationservice
	kubectl rollout status --timeout 2m -w deployments/frontendservice
	@echo "Frontend service loadbalancer ip: $(value SERVICE_IP)"
	test 200 = $$(curl -sL -w "%{http_code}\\n" http://$(value SERVICE_IP) -o /dev/null)

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
