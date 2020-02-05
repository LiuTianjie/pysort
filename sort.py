# 产生一个随机的数组a
import random
import copy
a = [random.randint(0, 1000) for i in range(20)]


# 冒泡排序
def mp_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    for k in range(0, length-1):
        for i in range(length-1-k):
            if a[i] > a[i+1]:
                a[i], a[i+1] = a[i+1], a[i]  # 比较相邻元素，实现”冒泡“
    return a


# 插入排序
def insert_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    for i in range(1, length):
        for j in range(i-1, -1, -1):
            if j == 0 and a[i] < a[j]:  # 当有序部分长度为1且当前元素比其小时，直接插入到头部
                a.insert(j, a[i])
                a.pop(i+1)
                break
            if a[i] < a[j] and a[i] >= a[j-1]:  # 将当前元素插入到有序部分
                a.insert(j, a[i])
                a.pop(i+1)
    return a


# 快速排序

def fast_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    if length <= 1:
        return a    # 增加出口，避免陷入死循环
    else:
        base = length//2
        left = []
        center = []
        right = []
        for i in range(length):
            if a[i] < a[base]:
                left.append(a[i])
            if a[i] > a[base]:
                right.append(a[i])
            if a[i] == a[base]:
                center.append(a[i])  # 如果不加等于情况判断，则数组中有重复的值时会漏掉
        return fast_sort(left) + center + fast_sort(right)  # 递归调用


# 希尔排序
def shell_sort(arr):  # 接受一个数组和一个初始增量
    a = copy.deepcopy(arr)
    length = len(a)
    increment = length//2
    while increment >= 1:
        for i in range(0, increment):
            # print(a[i:length:increment])  # 查看每次进行插入排序的数组是否正确
            a[i:length:increment] = insert_sort(
                a[i:length:increment])  # 调用插入排序
        increment //= 2
    return a


# 基数排序
def basenum_sort(arr):
    a = copy.deepcopy(arr)
    length = len(str(max(a)))   # 找出最大数的位数
    for i in range(length):
        pos = pow(10, i)
        for k in range(0, len(a)):  # 内部使用冒泡排序
            for j in range(len(a)-1-k):
                if a[j] / pos > a[j+1] / pos:
                    a[j], a[j+1] = a[j+1], a[j]
    return a


# 堆排序
# 调整非叶结点的父子大小关系
def heap_node_sort(a, i, new_length):
    if a[2*i+1] > a[i]:
        a[2*i+1], a[i] = a[i], a[2*i+1]
    if 2*i+2 < new_length and a[2*i+2] > a[i]:
        a[2*i+2], a[i] = a[i], a[2*i+2]

# 调整子树的


def heap_child_sort(a, new_length):
    # 根节点0，父节点i，左孩子2i-1，右孩子2i+1，最后一个非叶子节点len//2-1
    i = new_length//2 - 1
    while i >= 0:
        heap_node_sort(a, i, new_length)
        i -= 1
    # print("当前最大值：", a[0])
    a[new_length-1], a[0] = a[0], a[new_length-1]


# 主函数
def heap_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    while length > 1:
        heap_child_sort(a, length)
        length -= 1
    return a


# 归并排序
def merge_sort(arr):
    a = copy.deepcopy(arr)
    length = len(a)
    i = 0
    while pow(2, i) <= length:
        for j in range(0, length, pow(2, i+1)):
            # 先分组，再合并的思想
            a[j:j+pow(2, i+1)] = insert_sort(a[j:j+pow(2, i+1)])
        i += 1
    return a


print("原数组:", a)
print("1.冒泡排序:", mp_sort(a))
print("2.插入排序:", insert_sort(a))
print("3.快速排序:", fast_sort(a))
print("4.希尔排序:", shell_sort(a))
print("5.基数排序:", basenum_sort(a))
print("6.堆排序:", heap_sort(a))
print("7.归并排序:", merge_sort(a))
