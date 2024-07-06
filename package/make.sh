#!/bin/sh

CURR_DIR=$(cd "$(dirname "$0")" || exit 1;pwd)
VER="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "check-k8s")  | .version')"
CONTROL_DIR="$CURR_DIR/.."

showhelp()
{
	echo "Usage: make [OPTION]"
	echo "Options:"
	echo " -o               output directory."
	echo " --arch           archtecture."
	echo " --ver            version."
	echo " -h               show this message."
}

build()
{
	ROOT="$(mktemp -d)"
	rm -fr "${ROOT}"
	mkdir -p "${ROOT}"
	cd "$ROOT/" || exit 1

	cp "$CURR_DIR/DEBIAN" "$ROOT/" -af
	export CONTROL="$ROOT/DEBIAN/control"
	mkdir "${ROOT}/usr/sbin" -p
	# mkdir "${ROOT}/lib/systemd/system" -p
    # mkdir "${ROOT}/etc/init.d" -p

	sed -i "s/Version:.*/Version: ${VER}/" "$ROOT/DEBIAN/control"
	sed -i "s/Architecture:.*/Architecture: $ARCH/" "$ROOT/DEBIAN/control"
	# chmod 0755 $ROOT/DEBIAN/prerm

	echo "Copying binaries..."
	cp "${CONTROL_DIR}/target/release/check-k8s-deployments" "$ROOT/usr/sbin/" || exit 1
	cp "${CONTROL_DIR}/target/release/check-k8s-pods" "$ROOT/usr/sbin/" || exit 1
	ls -lah "$ROOT/usr/sbin/"

	# shellcheck disable=SC2181
	if [ $? -ne 0 ]; then
		echo "copy binaries file failed."
		return 1
	fi

	find "${ROOT}/usr/sbin/" -name 'check-k8s-*' -type f -exec chmod 0755 "{}" \;

	dpkg -b "${ROOT}" "$OUTPUTDIR/check-k8s-${OS_ID}$(lsb_release -s -r)_$VER.$FILEARCH.deb"

	rm -fr "${ROOT:?}/"
}

main()
{
	OPTS=$(getopt -o h --long output:,os:,arch:,ver:,filearch: -n  "" -- "$@")

	# shellcheck disable=SC2181
	if [ $? != 0 ] ; then echo "Terminating..." >&2 ; exit 1 ; fi

	# Note the quotes around `$TEMP': they are essential!
	eval set -- "$OPTS"

	while true; do
		case "$1" in
		--arch)
			ARCH="$2"
			shift 2;;
		--filearch)
			FILEARCH="$2"
			shift 2;;
		--os)
			OS_ID="$2"
			shift 2;;
		--ver)
			VER="$2"
			shift 2;;
		--output )
			OUTPUTDIR="$2"
			shift 2;;
		-h | --help )
			showhelp
			return 0
			# shellcheck disable=SC2317
			shift ;;
		-- ) shift; break ;;
		* ) break ;;
		esac
	done


	if [ -z "${OS_ID}" ]; then
		OS_ID=$(lsb_release -i -s)
	fi

	if [ -z "$ARCH" ]; then
		ARCH="$(dpkg --print-architecture)"
	fi

	if [ -z "$FILEARCH" ]; then
		FILEARCH=$ARCH
	fi

	if [ -z "$OUTPUTDIR" ]; then
		OUTPUTDIR=$CURR_DIR;
	fi

	build
}

# shellcheck disable=SC2068
main $@
exit $?
