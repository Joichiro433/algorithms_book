from rich import print


def sum_string(num_str: str) -> int:
    result: int = 0
    for bit in range(1<<(len(num_str)-1)):
        temp: int = 0
        for idx, s in enumerate(num_str[:-1]):
            temp += int(s)
            if bit & (1<<idx):
                result += temp
                temp = 0
            else:
                temp *= 10
        result += temp + int(num_str[-1])

    return result


if __name__ == '__main__':
    num_str: str = '125'
    result: int = sum_string(num_str=num_str)

    print(result)