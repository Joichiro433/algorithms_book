from typing import List

from rich import print


def count_num(nums: List[int], target: int) -> int:
    counter: int = 0
    for num in nums:
        if num == target:
            counter += 1
    
    return counter


if __name__ == '__main__':
    nums: List[int] = [3, 4, 1, 2, 5, 1]
    target: int = 1
    result: int = count_num(nums=nums, target=target)
    print(result)
