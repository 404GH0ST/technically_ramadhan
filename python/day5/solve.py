def alt_or(lst):
    if lst == []:
        return None

    for b in lst:
        if b:
            return True
    return False
