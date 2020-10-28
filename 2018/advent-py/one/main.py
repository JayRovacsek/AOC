#!/usr/bin/env python3
import os

def frequency_process(numbers,frequency,find_duplicate,r):
    for line in numbers:
        frequency += int(line)
        if find_duplicate:
            if frequency in r:
                find_duplicate = False
                return frequency

            r[frequency] = 'Seen'

    if find_duplicate:
        return frequency_process(numbers,frequency,True,r)
    return frequency

def generate_number_list(f):
    l = []
    for i in f:
        l.append(int(i))
    return l

if __name__ == '__main__':
    with open("input.txt",mode="r") as f:
        numbers = generate_number_list(f)

    print("Solution to part 1: {}".format(frequency_process(numbers,0,False,{})))
    print("Solution to part 2: {}".format(frequency_process(numbers,0,True,{})))