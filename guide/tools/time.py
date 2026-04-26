import re
from sys import argv

if __name__ == "__main__":
    if len(argv) < 2:
        print("Usage: python time.py <target_dir>")
        exit(1)
    target_dir = argv[1]
    if not target_dir:
        print("Usage: python time.py <target_dir>")
        exit(1)

    with open(f"{target_dir}/flow-graph.stat.json", "r") as f:
        content = f.read()

    time_pattern = re.compile(r'"(\w+)_time":\{"secs":(\d+),"nanos":(\d+)\}')
    all_times = time_pattern.findall(content)
    sum_time = 0
    for name, secs, nanos in all_times:
        total_time = int(secs) + int(nanos) / 1e9
        print(f"{name}: {total_time:.3f} seconds")
        sum_time += total_time
    print("-" * 40)
    print(f"Total time: {sum_time:.3f} seconds")
