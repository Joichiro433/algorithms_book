from typing import List

from rich import print


def search_min_pair(nums1: List[int], nums2: List[int], threshold: int) -> int:
    min_val: int = 1e10
    for num1 in nums1:
        for num2 in nums2:
            pair: int = num1 + num2
            if pair < threshold:
                continue
            if pair < min_val:
                min_val = pair

    return min_val


if __name__ == '__main__':
    nums1: List[int] = [8, 5, 4]
    nums2: List[int] = [4, 1, 9]
    threshold: int = 10
    result: int = search_min_pair(nums1=nums1, nums2=nums2, threshold=threshold)
    
    print(result)
