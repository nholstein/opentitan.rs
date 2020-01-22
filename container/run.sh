#!/bin/bash

TOPDIR=$(cd $(dirname $0)/.. && pwd)
docker run --rm -it \
	-v$TOPDIR:/opentitan \
	-w /opentitan \
	opentitan.rs
