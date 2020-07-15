DATA_DIR=$1     # target dir for data
N=$2            # number of data points

echo "### Starting experiments on CPU"
taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/side-channel.txt &
taskset --cpu-list 4 cargo run -q --bin victim

wait $(jobs -p)
echo "Done!"
