#!/usr/bin/env python3

import os
import unittest

class TestMethods(unittest.TestCase):

    def test_frequency_find(self):
        with open("test.txt",mode="r") as f:
            numbers = generate_number_list(f)

        self.assertEqual(10,frequency_process(numbers,0,True,{}))
        self.assertNotEqual(0,frequency_process(numbers,0,True,{}))
        self.assertNotEqual(None,frequency_process(numbers,0,True,{}))

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

    unittest.main()