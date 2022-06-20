from typing import List

from rich import print


def search_second_min_val(nums: List[int]) -> int:
    minimum_val: int = 200000
    second_val: int = 200000
    for num in nums:
        if num < minimum_val:
            second_val = minimum_val
            minimum_val = num
        elif num < second_val:
            second_val = num
    
    return second_val


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 8]
    result: int = search_second_min_val(nums=nums)
    print(result)
