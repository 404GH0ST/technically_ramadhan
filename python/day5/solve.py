def alt_or(lst):
    if lst == []:
        return None

    for b in lst:
        if b == True:
            return True
    return False
