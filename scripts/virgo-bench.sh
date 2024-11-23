#!/bin/bash

# 定义输出文件的路径
OUTPUT_FILE="outputs/virgo.log"
PROOF_SIZE_FILE="virgo-proofsize.csv"

echo "virgo benchmarking..."
# 运行cargo bench命令并将结果输出到文件
cargo bench -p virgo -- --nocapture --quiet > $OUTPUT_FILE

# 检查命令是否成功执行
if [ $? -eq 0 ]; then
    echo "Benchmark results have been written to $OUTPUT_FILE"
else
    echo "Benchmark failed to run"
fi

echo "virgo proofsizing..."
cargo test -p virgo --release -- --nocapture --quiet
cp virgo/virgo.csv outputs/$PROOF_SIZE_FILE
echo "virgo proofsize has been written to $PROOF_SIZE_FILE"



