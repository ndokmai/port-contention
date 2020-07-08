DATA_DIR=$1     # target dir for data
N=$2            # number of data points
ROUNDS=$3       # number of rounds per data point

echo "### Starting experiments on CPU #0 ###"
echo "#### Baseline experiment ###"
taskset 0x1 cargo run --bin monitor -- $N $ROUNDS > $DATA_DIR/baseline.txt

echo "#### Normal experiment ###"
taskset 0x1 cargo run --bin victim -- true &
taskset 0x1 cargo run --bin monitor -- $N $ROUNDS > $DATA_DIR/normal.txt
kill $(pidof victim)

echo "#### Subnormal experiment ###"
taskset 0x1 cargo run --bin victim -- false &
taskset 0x1 cargo run --bin monitor -- $N $ROUNDS > $DATA_DIR/subnormal.txt
kill $(pidof victim)

echo "Done!"

