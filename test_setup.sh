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

set -eu

tinc_node_name="syamimomo"
docker_name="sakura_syamimomo_test"


echo_green() {
  printf '\e[32m%s\e[m\n' "${1}"
}

echo_red() {
  printf '\e[31m%s\e[m\n' "${1}"
}

# テストを失敗させるときに呼ぶシェル関数
testfail() {
  {
    echo_red "${1}"
    echo_red "TEST FAILED"
  } >&2
  exit 1
}

mktmp() {
  tmp="$(mktemp -d)" || testfail "テンポラリディレクトリの作成に失敗しました"
  echo "tmp created at: ${tmp}" >&2
}

rmtmp() {
  rm -rf "${tmp}"
}

rmtmpc() {
  rm -rf "${tmp:?}"/*
}

onexit() {
  {
    set +e
    docker stop "${docker_name}"
    rmtmp
  } > /dev/null 2>&1
}

trap onexit EXIT
mktmp

#
# テスト用イメージのビルド
#
docker build --no-cache -t "${docker_name}" . ||
  testfail "docker build failed"


#
# ブリッジインターフェースの重複回避のテスト
#
rmtmpc

# br0 が存在する状態で sakura tinc setup が失敗するかチェック
! {
  docker run --rm -d --privileged --name="${docker_name}" "${docker_name}" &&
    sleep 1 &&
    docker exec "${docker_name}" ip link add br0 type bridge &&
    docker exec "${docker_name}" sakura tinc setup "${tinc_node_name}" 10.50.255.1
} || {
  docker stop "${docker_name}"
  testfail "sakura tinc setup の実行は失敗するはずですが, 成功しました"
}
docker stop "${docker_name}"

# br1 を指定して sakura tinc setup が成功するかチェック
{
  docker run --rm -d --privileged -v "${tmp}":/etc/tinc --name="${docker_name}" "${docker_name}" &&
    sleep 1 &&
    docker exec "${docker_name}" ip link add br0 type bridge &&
    docker exec "${docker_name}" sakura tinc setup -i br1 "${tinc_node_name}" 10.50.255.1 &&
    docker stop "${docker_name}"
} || testfail "sakura tinc setup の実行に失敗しました"

# br1 でtinc-upファイルが作成されているかチェック
grep 'ip link add br1 type bridge' < "${tmp}"/mchkd/tinc-up > /dev/null ||
  testfail "L${LINENO:-?}: /etc/tinc/mchkd/tinc-up が正しく作成されていません"


#
# RSA鍵の配置のテスト
#
rmtmpc

# コンテナを起動 -> sakura tinc setup を実行 -> コンテナを停止
{
  docker run --rm -d --privileged -v "${tmp}":/etc/tinc --name="${docker_name}" "${docker_name}" &&
    sleep 1 &&
    docker exec "${docker_name}" sakura tinc setup "${tinc_node_name}" 10.50.255.1 &&
    docker stop "${docker_name}"
} || testfail "sakura tinc setup の実行に失敗しました"

# 自身のhost定義が作成されたかをチェック
test -f "${tmp}"/mchkd/hosts/"${tinc_node_name}" ||
  testfail "File not found: ${tmp}/mchkd/hosts/${tinc_node_name}"

# 自身のhost定義にRSA公開鍵が存在するかどうかをチェック
grep -- '-----BEGIN RSA PUBLIC KEY-----' "${tmp}"/mchkd/hosts/"${tinc_node_name}" > /dev/null ||
  testfail "RSA pubkey not found in: ${tmp}/mchkd/hosts/${tinc_node_name}"


echo_green "TEST SUCCESS!" >&2
