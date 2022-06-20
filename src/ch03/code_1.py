from typing import List

from rich import print


def linear_search(nums: List[int], target: int) -> bool:
    for num in nums:
        if num == target:
            return True
    return False


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 7]
    target: int = 5
    result: bool = linear_search(nums=nums, target=target)
    print(result)