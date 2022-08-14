#!/bin/sh

#
# sakura tinc setup のテスト
#
# RUN:
#   ./debug_run.sh
#
# コンテナでシェルを実行する:
#   docker exec -it sakura_syamimomo_debug bash
#
# sakura tinc setup:
#   docker exec -it sakura_syamimomo_debug sakura tinc setup syamimomo 10.50.255.1
#
# sakura tinc update:
#   docker exec -it sakura_syamimomo_debug sakura tinc update
#

docker_name="sakura_syamimomo_debug"

if ! docker info > /dev/null; then
  {
    echo "docker engineが起動していないかも?"
    echo "終了します"
  } >&2
  exit 1
fi

docker stop "${docker_name}"
docker build --no-cache -t "${docker_name}" . && docker run --rm -d --privileged --name="${docker_name}" "${docker_name}"
