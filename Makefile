COMPONENTS = loadbalancer monitoring database app

reverse = $(if $(1),$(call reverse,$(wordlist 2,$(words $(1)),$(1)))) $(firstword $(1))

run:
	@for cmp in ${COMPONENTS}; do\
		echo "run $$cmp";\
		pushd $$cmp; make run; popd;\
	done

clean:
	@for cmp in $(call reverse,$(COMPONENTS)); do\
		echo "clean $$cmp";\
		pushd $$cmp; make clean; popd;\
	done;\
	docker volume prune
