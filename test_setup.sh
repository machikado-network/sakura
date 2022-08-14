#!/bin/sh

#
# sakura tinc setup のテスト
#
# RUN:
#   ./test_setup.sh
#
# DEBUG:
#   sh -x ./test_setup.sh
#

tinc_node_name="syamimomo"
docker_name="sakura_syamimomo_test"

echo_green() {
  printf '\e[32m%s\e[m\n' "${1}"
}

echo_red() {
  printf '\e[31m%s\e[m\n' "${1}"
}

tmp="$(mktemp -d)"
echo "tmp created at: ${tmp}" >&2

docker build --no-cache -t "${docker_name}" . &&
docker run --rm -d --privileged -v "${tmp}":/etc/tinc --name="${docker_name}" "${docker_name}" &&
sleep 1 &&
docker exec "${docker_name}" sakura tinc setup "${tinc_node_name}" 10.50.255.1 &&
docker stop "${docker_name}"

if ! test -f "${tmp}"/mchkd/hosts/"${tinc_node_name}"; then
  {
    echo_red "File not found: ${tmp}/mchkd/hosts/${tinc_node_name}"
    echo_red "TEST FAILED"
  } >&2
  exit 1
fi

if ! grep -- '-----BEGIN RSA PUBLIC KEY-----' "${tmp}"/mchkd/hosts/"${tinc_node_name}" > /dev/null; then
  {
    echo_red "RSA pubkey not found in: ${tmp}/mchkd/hosts/${tinc_node_name}"
    echo_red "TEST FAILED"
  } >&2
  exit 1
fi

rm -rf "${tmp}"
echo_green "TEST SUCCESS!" >&2
