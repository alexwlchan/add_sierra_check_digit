def add_check_digit(id_str):
    total = sum(
        i * int(digit)
        for i, digit in enumerate(reversed(id_str), start=2)
    )

    remainder = total % 11

    if remainder == 10:
        return id_str + 'x'
    else:
        return id_str + str(remainder)
