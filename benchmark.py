from argparse import ArgumentParser
from datetime import datetime
import subprocess
import time

import numpy

PATH = "./target/release/raytracing-in-one-weekend"
BENCHMARK_FILE = "./benchmarks.csv"

def main():
    parser = ArgumentParser()
    parser.add_argument(
        '-n',
        nargs='?',
        type=int,
        default=100,
        help="number of rounds to run the benchmark"
    )
    parser.add_argument(
        '--commit',
        action='store_true',
        default=False,
        help="record the benchmark stats to a file"
    )
    args = parser.parse_args()

    times = numpy.zeros((args.n))
    with open("/dev/null", "w") as dev_null:
        for i in range(args.n):
            start = time.time();
            subprocess.call(PATH, stdout=dev_null)
            times[i] = time.time() - start

    average_time = times.mean()
    stddev = times.std()
    print(f"Average time taken for {args.n} runs: {average_time:.5f} ± {stddev:.5f}s")

    if args.commit:
        time_str = datetime.now().strftime("%Y.%m.%d@%H:%M:%S")
        with open(BENCHMARK_FILE, 'a') as f:
            f.write(f"{time_str},{average_time:.5f},{stddev:.5f},{args.n}\n")


if __name__ == '__main__':
    main()
