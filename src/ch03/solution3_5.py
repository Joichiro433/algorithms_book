from typing import List

from rich import print


def get_div_two_times(num: int) -> int:
    times: int = 0
    while num % 2 == 0:
        num = num // 2
        times += 1
    return times


if __name__ == '__main__':
    nums: List[int] = [8, 12, 40]
    result: int = 200000
    for num in nums:
        result = min(result, get_div_two_times(num=num))
    print(result)
