# port-contention
Port-contention side-channel demonstration based on floating-point arithmetic side-channels.
## Benchmark
To ensure there exist floating-point arithmetic side-channels in the system, run
```bash
  cargo run --bin bench
```
Observe the clock cycles discrepancy between 'integer' and 'small' on the one hand and 'subnormal' on the other.
## Side-channel measurement
Run the script,
```bash
./run_experiment.sh <output_dir> <num_data_points>
```
For example,
```bash
./run_experiment.sh data 100
```
After the script finishes, 3 text files will be generated: `baseline.txt`, `normal.txt`, and `subnormal.txt`.

## Plotting
In [`plot/`](plot), run
```bash
python plot.py /path/to/baseline.txt /path/to/normal.txt /path/to/subnormal.txt
```
