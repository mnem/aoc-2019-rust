#!/bin/bash
for d in day-*/ ; do
	pushd $d
	pwd
	cargo run
	popd
done
