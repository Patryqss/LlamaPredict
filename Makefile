default: build-all

CONTRACTS="conditional_psp22"
ALL_CONTRACTS="$(CONTRACTS) factory_contract pair_contract router_contract"

.PHONY: build-all
build-all:
	cd amm && make build-amm
	@for d in $(CONTRACTS); do \
	 	echo "Building $$d contract" ; \
	 	cargo contract build --quiet --manifest-path contracts/$$d/Cargo.toml --release ; \
	done
