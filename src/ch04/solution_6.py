from typing import List

from rich import print


def exist_if_sum_pair(i: int, target: int, nums: List[int], memo: List[List[int]]) -> bool:
    if i == 0:
        return target == 0
    
    if memo[i][target] != -1:
        return memo[i][target]

    if exist_if_sum_pair(i=i-1, target=target, nums=nums, memo=memo):
        memo[i][target] = 1
        return True
    if exist_if_sum_pair(i=i-1, target=target-nums[i-1], nums=nums, memo=memo):
        memo[i][target] = 1
        return True

    memo[i][target] = 0
    return False


if __name__ == '__main__':
    target: int = 10
    nums: List[int] = [1, 2, 4, 5, 11]
    N: int = len(nums)
    memo: List[List[int]] = [[-1 for _ in range(target+1)] for _ in range(N+1)]
    
    result: bool = exist_if_sum_pair(i=N, target=target, nums=nums, memo=memo)

    print(result)
