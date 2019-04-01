PHONY: update-proto
update-proto: # Update protobuf definitions for all microservices
	cp proto/fortune.proto src/fortune/proto
	cp proto/fortune.proto src/frontend/proto


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
