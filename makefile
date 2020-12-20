test:
	target/debug/chinese_fuzzy_melon tests/hello/flaw -i tests/hello/cfm_input/ -o ./tests/hello/cfm_output/

test-clear:
	rm -f /tests/hello/core.*
	rm /tests/hello/cfm_output/*
