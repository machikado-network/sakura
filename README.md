# sakura Stack

まちかどネットワークの結界の構築や調整をおこなうツール

使い方: https://scrapbox.io/machikado-network/%E3%81%BE%E3%81%A1%E3%82%AB%E3%83%89%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF%E3%81%B8%E3%81%AE%E7%B9%8B%E3%81%8E%E6%96%B9


## ビルド

```sh
cargo build
```


## Dockerでデバッグ

**ビルド&コンテナを起動**

```sh
./debug_run.sh
```

**コンテナで sakura tinc setup を実行する**

```sh
docker exec -it sakura_syamimomo_debug sakura tinc setup syamimomo 10.50.255.1
```

**コンテナで sakura tinc update を実行する**

```sh
docker exec -it sakura_syamimomo_debug sakura tinc update
```

**正しいSTORE_ADDRESSで sakura tinc update を実行する**

```sh
docker exec -it -e STORE_ADDRESS="0x34eee539739466f8ce4d005bcfb59271824e139f130681849490836482dd1e84" sakura_syamimomo_debug sakura tinc update
```

**誤ったSTORE_ADDRESSで sakura tinc update を実行する**

```sh
docker exec -it -e STORE_ADDRESS="0x34eee539739466f8ce4d005bcfb59271824e139f130681849490836482dd1e85" sakura_syamimomo_debug sakura tinc update
```

**コンテナでシェルを実行する**

```sh
docker exec -it sakura_syamimomo_debug bash
```


## Dockerでテスト

**tinc setup のテスト**

```sh
./test_setup.sh
```
