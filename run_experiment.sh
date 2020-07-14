DATA_DIR=$1     # target dir for data
N=$2            # number of data points

killall victim > /dev/null

echo "### Starting experiments on CPU #0 ###"
echo "#### Baseline experiment ###"
taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/baseline.txt

number=$(expr $RANDOM % 2)

if [ $number == 0 ]; then
    echo "#### Subnormal experiment ###"
    taskset --cpu-list 4 cargo run -q --bin victim -- false &
    taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/subnormal.txt
    killall victim
    echo "#### Normal experiment ###"
    taskset --cpu-list 4 cargo run -q --bin victim -- true &
    taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/normal.txt
    killall victim
else
    echo "#### Normal experiment ###"
    taskset --cpu-list 4 cargo run -q --bin victim -- true &
    taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/normal.txt
    killall victim
    echo "#### Subnormal experiment ###"
    taskset --cpu-list 4 cargo run -q --bin victim -- false &
    taskset --cpu-list 0 cargo run -q --bin monitor -- $N > $DATA_DIR/subnormal.txt
    killall victim
fi


echo "Done!"

