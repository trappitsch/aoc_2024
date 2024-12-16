import numpy as np


with open('input', 'r') as f:
    datain = f.read().splitlines()

data = []
for line in datain:
    tmp = []
    for it in line.split():
        tmp.append(int(it))
    data.append(tmp)

def check_ok(dat: np.ndarray) -> bool:
    """Check if a given line is okay.

    Args:
        dat (np.ndarray): A line of data, must be majority monotonically rising.
    """
    diff = dat[1:] - dat[:-1]
    if np.all(diff > 0) and np.all(diff <= 3):
        return True

def make_rising(dat: np.ndarray) -> np.ndarray:
    """Turn the list into a monotonically rising list.

    Args:
        dat (np.ndarray): A line of data.
    """
    diff = dat[1:] - dat[:-1]
    if np.where(diff >= 0)[0].sum() < np.where(diff <= 0)[0].sum(): 
        return dat * -1
    else:
        return dat

tot = 0
data_leftover = []
for dat in data:
    dat = np.array(dat)
    if check_ok(make_rising(dat)):
        tot += 1
    else:
        data_leftover.append(dat)

# check for dampened - BRUTE FORCE
for dat in data_leftover:
    dat = np.array(dat)
    for rmi in range(len(dat)):
        if check_ok(make_rising(np.delete(dat, rmi))):
            tot += 1
            break

print(tot)

