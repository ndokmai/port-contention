DATA_DIR=$1     # target dir for data
MICROSCOPE_DIR=$2
#CALLER_PATH=$3

CPU1=0
CPU2=4

echo "### Install Microscope"
killall victim > /dev/null
killall monitor > /dev/null
(cd $MICROSCOPE_DIR &&
    make &&
    sudo rmmod microscope.ko &&
    sudo insmod microscope.ko
)

echo "### Building victim & monitor process"
cargo build

echo "### Starting monitor process"
taskset --cpu-list $CPU1 cargo run -q --bin monitor -- $DATA_DIR/side-channel.txt &
sleep 0.1s

echo "### Starting victim process"
taskset --cpu-list $CPU2 cargo run -q --bin victim | tee $DATA_DIR/secret.txt

#REPLAY_HANDLE_OFFSET=0x23A0D0
#VICTIM_PID=$(pidof victim)
#BASE_ADDR=0x$(cat /proc/$VICTIM_PID/maps | head -1 | awk '{split($1,a,"-"); print a[1]}')
#REPLAY_HANDLE=$(printf "0x%x\n" $(($BASE_ADDR + $REPLAY_HANDLE_OFFSET)))
#echo "victim_pid: $VICTIM_PID"
#echo "base_addr: $BASE_ADDR"
#echo "replay_handle: $REPLAY_HANDLE"

#sudo $CALLER_PATH <<STDIN -o other --options
#$VICTIM_PID
#$REPLAY_HANDLE
#STDIN

kill -SIGINT $(pidof monitor)
wait $(pidof monitor)
echo "### Monitoring done"
echo "### Starting analysis"
secret=$(cat $DATA_DIR/secret.txt) && python3 scripts/guess.py $DATA_DIR/side-channel.txt "$secret"
