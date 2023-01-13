import subprocess
import time

import numpy

TRIALS = 100
PATH = "./target/release/raytracing-in-one-weekend"

def main():
    times = numpy.zeros((TRIALS))
    with open("/dev/null", "w") as dev_null:
        for i in range(TRIALS):
            start = time.time();
            subprocess.call(PATH, stdout=dev_null)
            times[i] = time.time() - start

    average_time = times.mean()
    stddev = times.std()
    print(f"Average time taken for {TRIALS} runs: {average_time:.5f} ± {stddev:.5f}s")


if __name__ == '__main__':
    main()
