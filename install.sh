#!/bin/bash

#cargo install --path clomonitor-linter
#cd clomonitor-linter
#cargo build   --release
#cargo build  --target x86_64-unknown-linux-musl --release
#cd ..


# use
#clomonitor-linter --mode local --path  /Users/liuhq/clomonitor   --check-set ant-incubator --format json

#copy
#cp /Users/liuhq/.cargo/bin/clomonitor-linter /Users/liuhq/incubator/app/service/src/main/resources/clomonitor-linter-mac

#scopy;从linux机器拷贝Linux版本到本地项目
#scp root@8.218.11.98:/root/clomonitor/target/release/clomonitor-linter /Users/liuhq/incubator/app/service/src/main/resources/clomonitor-linter-linux-musl
scp root@8.218.11.98:/root/clomonitor/target/release/clomonitor-linter /Users/liuhq/incubator/app/service/src/main/resources/clomonitor-linter-linux-glib217



