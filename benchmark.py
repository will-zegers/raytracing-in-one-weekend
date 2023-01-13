import subprocess
import time

TRIALS = 100
PATH = "./target/release/raytracing-in-one-weekend"

def main():
    average_time = 0
    with open("/dev/null", "w") as dev_null:
        for _ in range(TRIALS):
            start = time.time();
            subprocess.call(PATH, stdout=dev_null)
            average_time += (time.time() - start) / TRIALS

    print(f"Average time taken for {TRIALS} runs: {average_time}")


if __name__ == '__main__':
    main()
