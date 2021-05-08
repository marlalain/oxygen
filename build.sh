#!/usr/bin/env sh
set -euo pipefail

DESTDIR="build"
SRCDIR="src"
MAKESH="make.sh"

mkdir -pv $DESTDIR
find $SRCDIR -type d ! -name $SRCDIR -execdir \
	sh -c "cd {}; sh make.sh" \;
