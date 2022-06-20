from typing import List

from rich import print


def search_min_val(nums: List[int]) -> int:
    min_val: int = nums[0]
    for num in nums:
        if num < min_val:
            min_val = num

    return min_val


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 7]
    result: int = search_min_val(nums=nums)
    print(result)