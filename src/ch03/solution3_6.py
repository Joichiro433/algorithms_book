from rich import print


def get_three_pattern_times(target: int, threshold: int) -> int:
    counter: int = 0
    for x in range(min(target, threshold)+1):
        for y in range(min(target, threshold)+1):
            z = target - (x + y)
            if 0 <= z <= threshold:
                counter += 1

    return counter


if __name__ == '__main__':
    target: int = 4
    threshold: int = 5
    result: int = get_three_pattern_times(target=target, threshold=threshold)

    print(result)
