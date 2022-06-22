from typing import List

from rich import print


def exist_if_sum_pair(i: int, target: int, nums: List[int]) -> bool:
    if i == 0:
        return target == 0
    if exist_if_sum_pair(i=i-1, target=target, nums=nums):
        return True
    if exist_if_sum_pair(i=i-1, target=target-nums[i-1], nums=nums):
        return True
    return False


if __name__ == '__main__':
    target: int = 10
    nums: List[int] = [1, 2, 4, 5, 11]
    result: bool = exist_if_sum_pair(i=len(nums), target=target, nums=nums)

    print(result)
