from typing import List, Dict

from rich import print


def get_753_times(target: int) -> int:
    times: int = 0
    for num in range(1, target+1):
        valid_counter: Dict[str, int] = {'7': 0, '5': 0, '3': 0}
        for key in valid_counter.keys():
            valid_counter[key] = list(str(num)).count(key)
        if all(valid_counter.values()) and sum(valid_counter.values()) == len(str(num)):
            times += 1
    return times


def get_recurrent_753_times(target: int, num: int, valid_bit: int, result_patterns: List[int]) -> None:
    if num > target:
        return None  # ベースラインを超えたら関数を終了
    if valid_bit == 0b111:
        result_patterns.append(num)

    get_recurrent_753_times(target=target, num=num*10+7, valid_bit=valid_bit | 0b100, result_patterns=result_patterns)
    get_recurrent_753_times(target=target, num=num*10+5, valid_bit=valid_bit | 0b010, result_patterns=result_patterns)
    get_recurrent_753_times(target=target, num=num*10+3, valid_bit=valid_bit | 0b001, result_patterns=result_patterns)
    

if __name__ == '__main__':
    target: int = 575
    # result: int = get_753_times(target=target)
    result_patterns: List[int] = []
    get_recurrent_753_times(target=target, num=0, valid_bit=0b000, result_patterns=result_patterns)
    result: int = len(result_patterns)
    print(result)
