from typing import List

from rich import print


def linear_search(nums: List[int], target: int) -> int:
    for idx, num in enumerate(nums):
        if num == target:
            return idx
    return -1


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 7]
    target: int = 5
    result: int = linear_search(nums=nums, target=target)
    print(result)