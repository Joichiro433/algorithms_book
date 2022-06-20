from typing import List

from rich import print


def exhaustive_search(nums: List[int], target: int) -> bool:
    for bit in range(1<<len(nums)):
        total: int = 0
        for i in range(len(nums)):
            if bit & (1<<i):
                total += nums[i]
        if total == target:
            return True
    
    return False


if __name__ == '__main__':
    nums: List[int] = [1, 2, 4, 5, 11]
    target: int = 10
    result: bool = exhaustive_search(nums=nums, target=target)
    
    print(result)
