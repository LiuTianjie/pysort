# Pysort
> @Nickname4th

The python file sort.py contains 7 different arithmetics to sort an array.
#### Initial array
First of all, let's creat an random array including 10 or more numbers by a built-in module named "random" like this:
``` python
import random
import copy
arr = [random.randint(0, 1000) for i in range(10)]
```
In order to check the differences after we sort the array, each time we should copy one deeply using the module "copy" like this:
```
a = copy.deepcopy(arr)
length = len(a)
```
#### bubble sort
The key of bubble sort is to compare two Neighboring numbers to realize "bubble", what's more, after each single "for" loop, we got the biggest numbers in this journey, so next time we needn't to ergodic every number in the array, that is why the second loop's argument is "length - 1 - k".
``` python
    def bubble_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    for k in range(0, length-1):
        for i in range(length-1-k):
            if a[i] > a[i+1]:
                a[i], a[i+1] = a[i+1], a[i] 
    return a
```
#### insert sort

``` python
def insert_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    for i in range(1, length):
        for j in range(i-1, -1, -1):
            if j == 0 and a[i] < a[j]:  
                # When the length of ordered part is 1 and a[i] is smaller than it, insert into it's head deriectly.
                a.insert(j, a[i])
                a.pop(i+1)
                break
            if a[i] < a[j] and a[i] >= a[j-1]:  
                # Insert a[i] into the order part.
                a.insert(j, a[i])
                a.pop(i+1)
    return a
```
### Add Rust sort
