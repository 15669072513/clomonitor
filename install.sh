#!/bin/bash

#cargo install --path clomonitor-linter

# use
#clomonitor-linter --mode local --path  /Users/liuhq/clomonitor   --check-set ant-incubator --format json

#copy
cp /Users/liuhq/.cargo/bin/clomonitor-linter /Users/liuhq/incubator/app/service/src/main/resources/clomonitor-linter-mac

#scopy;从linux机器拷贝Linux版本到本地项目
scp root@8.218.11.98:/root/clomonitor/linter-exe /Users/liuhq/incubator/app/service/src/main/resources/clomonitor-linter-linux