#!/usr/bin/env sh
set -euo pipefail

DESTDIR="build"
SRCDIR="src"
MAKESH="make.sh"

red="\e[0;91m"
green="\e[0;92m"
blue="\e[0;94m"
reset="\e[0m"

echo -e "${red}=>${reset} Cleaning old build folder..."
rm -rf $DESTDIR
echo -e "${green}=>${reset} Creating '$DESTDIR'"
mkdir -p $DESTDIR
find $SRCDIR -type d ! -name $SRCDIR -execdir \
	sh -c "cd {}; sh make.sh" \;
echo -e "${blue}=>${reset} Done"
