from typing import List

from rich import print


def search_max_diff(nums: List[int]) -> int:
    min_val: int = 200000
    max_val: int = -200000
    for num in nums:
        if num < min_val:
            min_val = num
        if num > max_val:
            max_val = num
    
    return max_val - min_val


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 8]
    result: int = search_max_diff(nums=nums)
    print(result)
